//! Точка сборки приложения.
//!
//! Здесь же пока живёт спайк Фазы 0 — он доказал четыре вещи, на которых
//! держится весь замысел:
//!   1. ComfyUI запускается напрямую через python.exe, без .bat;
//!   2. браузер при этом не открывается и окно консоли не всплывает;
//!   3. stderr стримится в интерфейс живьём, а не после завершения;
//!   4. дочерний вебвью грузит ComfyUI без 403 от origin-middleware.
//!
//! Спайк остаётся до Фазы 3 как единственный способ проверить встраивание:
//! путь захардкожен, состояние примитивное. Настоящая архитектура — с Фазы 1.

// Модули публичные: по ним ходят примеры в examples/, которыми
// проверяется распаковка на реальном архиве.
pub mod comfy_api;
pub mod discovery;
pub mod error;
pub mod installer;
pub mod instances;
pub mod ports;
pub mod process;
pub mod profiles;
pub mod run;
pub mod settings;
pub mod shared_models;
pub mod supervise;
pub mod workflows;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri::{LogicalPosition, LogicalSize, Manager, WebviewUrl};
use tauri_specta::Event;

use crate::discovery::windows_portable::WindowsPortable;
use crate::error::AppError;
use crate::installer::history::ArchiveRecord;
use crate::installer::{
    ArchiveInfo, InstallCancel, InstallLock, InstallProgress, InstallTarget, TargetCheck,
};
use crate::instances::{
    Accent, Instance, InstanceEdit, ProbeResult, SizeJobs, Sized_,
};
use crate::process::{RunState, RunStatus, Runtime};
use crate::profiles::LaunchProfile;
use crate::settings::{Bootstrap, LibrarySettings, UiSettings};
use crate::workflows::{LibraryScan, WorkflowMeta};
use crate::shared_models::{
    ApplyMode, InstanceFileInfo, InstanceFileState, InstanceShared, RootScan, SharedSettings,
};

/// Настройки, прочитанные при старте: тема, язык, состояние рейла,
/// плюс системная локаль и пути для раздела «О приложении».
#[tauri::command]
#[specta::specta]
async fn load_bootstrap(app: tauri::AppHandle) -> Result<Bootstrap, AppError> {
    settings::load(&app)
}

#[tauri::command]
#[specta::specta]
async fn save_settings(app: tauri::AppHandle, settings: UiSettings) -> Result<(), AppError> {
    settings::save(&app, &settings)
}

// ------------------------------------------------------------ реестр

#[tauri::command]
#[specta::specta]
async fn list_instances(app: tauri::AppHandle) -> Result<Vec<Instance>, AppError> {
    instances::list(&app)
}

/// Проверяет выбранную папку и заодно предлагает имя, порт и цвет.
///
/// Проверка и предложения приходят одним ответом: экран добавления
/// показывает их вместе, и разбивать это на три вызова незачем.
#[tauri::command]
#[specta::specta]
async fn probe_folder(app: tauri::AppHandle, path: String) -> Result<ProbeResult, AppError> {
    instances::probe(&app, &WindowsPortable, &path)
}

#[tauri::command]
#[specta::specta]
async fn suggest_accent(app: tauri::AppHandle) -> Result<Accent, AppError> {
    instances::suggest_accent(&app)
}

#[tauri::command]
#[specta::specta]
async fn add_instance(
    app: tauri::AppHandle,
    path: String,
    edit: InstanceEdit,
) -> Result<Instance, AppError> {
    // Папка проверяется заново, а не берётся из ответа probe_folder:
    // между экраном выбора и сохранением её могли переименовать.
    let probe = <WindowsPortable as crate::discovery::InstanceDiscovery>::probe(
        &WindowsPortable,
        std::path::Path::new(&path),
    )?;
    instances::add(&app, probe, edit, None)
}

#[tauri::command]
#[specta::specta]
async fn update_instance(
    app: tauri::AppHandle,
    id: String,
    edit: InstanceEdit,
) -> Result<Instance, AppError> {
    instances::update(&app, &id, edit)
}

/// Убирает инстанс из реестра. Папка на диске остаётся нетронутой.
#[tauri::command]
#[specta::specta]
async fn remove_instance(app: tauri::AppHandle, id: String) -> Result<(), AppError> {
    instances::remove(&app, &id)
}

// ---------------------------------------------------- мастер установки

#[tauri::command]
#[specta::specta]
async fn probe_archive(path: String) -> Result<ArchiveInfo, AppError> {
    installer::probe_archive(&path)
}

#[tauri::command]
#[specta::specta]
async fn check_targets(
    info: ArchiveInfo,
    targets: Vec<InstallTarget>,
) -> Result<Vec<TargetCheck>, AppError> {
    Ok(installer::check_targets(&info, &targets))
}

#[tauri::command]
#[specta::specta]
async fn archive_history(app: tauri::AppHandle) -> Result<Vec<ArchiveRecord>, AppError> {
    installer::history::list(&app)
}

#[tauri::command]
#[specta::specta]
async fn forget_archive(app: tauri::AppHandle, path: String) -> Result<(), AppError> {
    installer::history::forget(&app, &path)
}

/// Разворачивает архив в цели и регистрирует их.
///
/// Команда `async`, поэтому минуты распаковки не блокируют главный поток.
/// Прогресс идёт событиями: возвращать его в ответе нечем, ответ один.
#[tauri::command]
#[specta::specta]
async fn run_install(
    app: tauri::AppHandle,
    lock: tauri::State<'_, InstallLock>,
    cancel: tauri::State<'_, InstallCancel>,
    info: ArchiveInfo,
    targets: Vec<InstallTarget>,
) -> Result<Vec<Instance>, AppError> {
    let _guard = lock.acquire()?;
    cancel.reset();

    // Первым делом, до любой работы: от клика до этого события проходит
    // только время IPC, и экран перестаёт молчать сразу. Без него первые
    // секунды выглядели зависанием — проверки, открытие архива и разворот
    // словаря LZMA2 идут молча.
    let first_name = targets.first().map(|t| t.name.clone()).unwrap_or_default();
    let _ = installer::InstallProgress::stage(
        installer::InstallPhase::Preparing,
        1,
        targets.len() as u32,
        &first_name,
    )
    .emit(&app);

    // Проверки повторяются перед самой работой: между экраном целей
    // и запуском место на диске могло кончиться, а папка — появиться.
    let blocking: Vec<AppError> = installer::check_targets(&info, &targets)
        .into_iter()
        .flat_map(|c| c.errors)
        .collect();
    if let Some(first) = blocking.into_iter().next() {
        return Err(first);
    }

    // Минуты распаковки уходят в отдельный поток, а не на воркер асинхронного
    // рантайма: иначе `cancel_install` и остальные команды соревнуются
    // за тот же воркер всё это время.
    let emitter = app.clone();
    let work_info = info.clone();
    let work_targets = targets.clone();
    let cancel_flag = cancel.share();
    let outcome = tauri::async_runtime::spawn_blocking(move || {
        installer::run(&work_info, &work_targets, &cancel_flag, |progress| {
            let _ = progress.emit(&emitter);
        })
    })
    .await
    .map_err(|e| AppError::because("installer.extractFailed", e))?;
    outcome?;

    // Регистрация перепроверяет каждую цель и запускает `python --version`
    // на каждую — это ещё пара секунд после того, как файлы кончились.
    // Прежде экран стоял на сотне процентов с подписью от последнего файла.
    let _ = installer::InstallProgress::stage(
        installer::InstallPhase::Registering,
        targets.len() as u32,
        targets.len() as u32,
        &first_name,
    )
    .emit(&app);

    installer::history::remember(&app, &info)?;
    register_targets(&app, &info, &targets)
}

/// Регистрирует распакованные цели, проставляя источник.
fn register_targets(
    app: &tauri::AppHandle,
    info: &ArchiveInfo,
    targets: &[InstallTarget],
) -> Result<Vec<Instance>, AppError> {
    let source = instances::InstallSource {
        archive_path: info.path.clone(),
        archive_label: info.label.clone(),
        installed_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as f64)
            .unwrap_or(0.0),
    };

    let mut created = Vec::new();
    for target in targets {
        let probe = <WindowsPortable as crate::discovery::InstanceDiscovery>::probe(
            &WindowsPortable,
            std::path::Path::new(&target.path),
        )?;
        let edit = InstanceEdit {
            name: target.name.clone(),
            description: target.description.clone(),
            accent: target.accent,
            preferred_port: target.preferred_port,
        };
        created.push(instances::add(app, probe, edit, Some(source.clone()))?);
    }
    Ok(created)
}

/// Просит мастер остановиться. Проверяется между файлами, поэтому
/// отменённая установка не оставляет полураспакованного дерева.
#[tauri::command]
#[specta::specta]
async fn cancel_install(cancel: tauri::State<'_, InstallCancel>) -> Result<(), AppError> {
    cancel.request();
    Ok(())
}


// ------------------------------------------------------ запуск сборки

/// Событие с очередной строкой лога запущенного инстанса.
///
/// Имя с префиксом `Run`, чтобы не столкнуться с `SpikeLog` спайка:
/// tauri-specta выводит имя события из имени структуры, и одинаковые
/// имена разъехались бы молча.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
#[serde(rename_all = "camelCase")]
pub struct RunLog {
    pub instance_id: String,
    pub line: crate::process::LogLine,
}

/// Событие смены состояния. Приходит и тогда, когда пользователь ничего
/// не делал: падение и самоперезапуск обязаны быть видны сразу.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct RunChanged(pub RunStatus);

/// Профили запуска инстанса, разобранные из его `.bat` прямо сейчас.
#[tauri::command]
#[specta::specta]
async fn instance_profiles(
    app: tauri::AppHandle,
    id: String,
) -> Result<Vec<LaunchProfile>, AppError> {
    let instance = instances::list(&app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", &id))?;
    Ok(run::profiles_of(&instance))
}

#[tauri::command]
#[specta::specta]
async fn run_statuses(runtime: tauri::State<'_, Runtime>) -> Result<Vec<RunStatus>, AppError> {
    Ok(runtime.statuses())
}

/// Весь накопленный лог инстанса.
///
/// Нужен, чтобы вернувшись на экран инстанса увидеть старт целиком:
/// события догоняют только то, что происходит при открытом экране.
#[tauri::command]
#[specta::specta]
async fn run_log(
    runtime: tauri::State<'_, Runtime>,
    id: String,
) -> Result<Vec<crate::process::LogLine>, AppError> {
    Ok(runtime
        .get(&id)
        .map(|cell| cell.lock().unwrap().log.snapshot())
        .unwrap_or_default())
}

// ------------------------------------------------- библиотека воркфлоу

/// Куда положить свежую библиотеку, если пользователь не выбрал сам.
///
/// Рядом с корнем общих моделей — они обычно лежат на просторном диске,
/// и держать воркфлоу там же логично. Жёсткой связи нет: библиотека
/// работает и без общих моделей, поэтому при их отсутствии просто молчим.
#[tauri::command]
#[specta::specta]
async fn suggest_library_path(app: tauri::AppHandle) -> Result<Option<String>, AppError> {
    let shared = settings::load_shared(&app)?;
    Ok(shared
        .roots
        .first()
        .and_then(|r| std::path::Path::new(&r.path).parent().map(|p| p.to_path_buf()))
        .map(|p| p.join("workflows").display().to_string()))
}

#[tauri::command]
#[specta::specta]
async fn load_library_settings(app: tauri::AppHandle) -> Result<LibrarySettings, AppError> {
    settings::load_library(&app)
}

#[tauri::command]
#[specta::specta]
async fn save_library_settings(
    app: tauri::AppHandle,
    library: LibrarySettings,
) -> Result<(), AppError> {
    settings::save_library(&app, &library)
}

/// Читает библиотеку целиком.
///
/// В блокирующем потоке: обход дерева на паре сотен воркфлоу — тысячи
/// обращений к диску плюс разбор каждого JSON ради списка нод.
#[tauri::command]
#[specta::specta]
async fn scan_library(path: String) -> Result<LibraryScan, AppError> {
    tauri::async_runtime::spawn_blocking(move || {
        workflows::scan_library(std::path::Path::new(&path))
    })
    .await
    .map_err(|e| AppError::because("workflows.scanFailed", e))
}

/// Кладёт файл с диска в библиотеку.
///
/// Не воркфлоу — отказ с объяснением: библиотека обязана оставаться
/// библиотекой воркфлоу, а не свалкой JSON.
#[tauri::command]
#[specta::specta]
async fn add_workflow_file(
    library: String,
    source: String,
    rel: Option<String>,
    overwrite: bool,
) -> Result<String, AppError> {
    let content = std::fs::read_to_string(&source)
        .map_err(|e| AppError::because("workflows.readFailed", e))?;

    if workflows::node_types(&content).is_none() {
        return Err(AppError::with("workflows.notAWorkflow", "path", &source));
    }

    let name = rel.unwrap_or_else(|| {
        std::path::Path::new(&source)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "workflow.json".to_string())
    });

    let target = std::path::Path::new(&library).join(&name);
    if target.exists() && !overwrite {
        return Err(AppError::with("workflows.nameTaken", "path", &name));
    }
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AppError::because("workflows.writeFailed", e))?;
    }
    std::fs::write(&target, content)
        .map_err(|e| AppError::because("workflows.writeFailed", e))?;

    Ok(name)
}

/// Правит запись манифеста: избранное, теги, заметка.
#[tauri::command]
#[specta::specta]
async fn set_workflow_meta(
    library: String,
    rel: String,
    meta: WorkflowMeta,
) -> Result<(), AppError> {
    let root = std::path::Path::new(&library);
    let (mut manifest, _) = workflows::read_manifest(root);
    manifest.items.insert(rel, meta);
    workflows::write_manifest(root, &manifest)
}

/// Убирает потерянную запись из манифеста.
///
/// Только запись: файлов эта команда не касается вовсе — она и вызывается
/// лишь тогда, когда файла уже нет.
#[tauri::command]
#[specta::specta]
async fn forget_workflow(library: String, rel: String) -> Result<(), AppError> {
    let root = std::path::Path::new(&library);
    let (mut manifest, _) = workflows::read_manifest(root);
    manifest.items.remove(&rel);
    workflows::write_manifest(root, &manifest)
}

/// Воркфлоу, лежащие в сборке.
///
/// У запущенной спрашиваем по API, у остановленной читаем папку. Разница
/// не косметическая: у запущенной ответ учитывает то, что она сохранила
/// минуту назад, а у остановленной другого источника и нет.
#[tauri::command]
#[specta::specta]
async fn instance_workflows(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
) -> Result<Vec<String>, AppError> {
    let instance = find_instance(&app, &id)?;
    let port = running_port(&runtime, &id);

    tauri::async_runtime::spawn_blocking(move || match port {
        Some(port) => Ok(comfy_api::Client::new(port)
            .list_workflows()?
            .into_iter()
            .map(|f| f.path)
            .collect()),
        None => Ok(local_workflow_names(&instance)),
    })
    .await
    .map_err(|e| AppError::because("workflows.scanFailed", e))?
}

/// Забирает воркфлоу из сборки в библиотеку. Исходный остаётся на месте.
#[tauri::command]
#[specta::specta]
async fn pull_workflow(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    rel: String,
    library: String,
    overwrite: bool,
) -> Result<String, AppError> {
    let instance = find_instance(&app, &id)?;
    let port = running_port(&runtime, &id);

    tauri::async_runtime::spawn_blocking(move || {
        let content = match port {
            Some(port) => comfy_api::Client::new(port).read_workflow(&rel)?,
            None => {
                let path = local_workflows_dir(&instance).join(&rel);
                std::fs::read_to_string(&path)
                    .map_err(|e| AppError::because("workflows.readFailed", e))?
            }
        };

        if workflows::node_types(&content).is_none() {
            return Err(AppError::with("workflows.notAWorkflow", "path", &rel));
        }

        let target = std::path::Path::new(&library).join(&rel);
        if target.exists() && !overwrite {
            return Err(AppError::with("workflows.nameTaken", "path", &rel));
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::because("workflows.writeFailed", e))?;
        }
        std::fs::write(&target, content)
            .map_err(|e| AppError::because("workflows.writeFailed", e))?;

        // Помним, откуда взяли: через полгода это единственный способ
        // понять, почему воркфлоу требует именно этих нод.
        let root = std::path::Path::new(&library);
        let (mut manifest, _) = workflows::read_manifest(root);
        let entry = manifest.items.entry(rel.clone()).or_default();
        entry.source_instance_id = Some(id.clone());
        if entry.added_at.is_none() {
            entry.added_at = Some(now_ms());
        }
        workflows::write_manifest(root, &manifest)?;

        Ok(rel)
    })
    .await
    .map_err(|e| AppError::because("workflows.writeFailed", e))?
}

/// Кладёт воркфлоу из библиотеки в сборку.
///
/// У запущенной — через API с `overwrite=false`, и **409 возвращается как
/// развилка**, а не как ошибка: молча затирать чужой воркфлоу нельзя.
/// У остановленной — файлом, с той же проверкой существования.
#[tauri::command]
#[specta::specta]
async fn push_workflow(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    library: String,
    rel: String,
    overwrite: bool,
) -> Result<PushOutcome, AppError> {
    let instance = find_instance(&app, &id)?;
    if !instance.available {
        return Err(AppError::with("instances.missing", "path", &instance.path));
    }
    let port = running_port(&runtime, &id);

    tauri::async_runtime::spawn_blocking(move || {
        let content = std::fs::read_to_string(std::path::Path::new(&library).join(&rel))
            .map_err(|e| AppError::because("workflows.readFailed", e))?;

        match port {
            Some(port) => {
                match comfy_api::Client::new(port).upload_workflow(&rel, &content, overwrite)? {
                    comfy_api::UploadOutcome::Written => Ok(PushOutcome::Written),
                    comfy_api::UploadOutcome::Conflict => Ok(PushOutcome::Conflict),
                }
            }
            None => {
                let dir = local_workflows_dir(&instance);
                let target = dir.join(&rel);
                if target.exists() && !overwrite {
                    return Ok(PushOutcome::Conflict);
                }
                // Папки может не быть вовсе: ComfyUI создаёт её лениво,
                // при первом сохранении. Создаём сами.
                if let Some(parent) = target.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| AppError::because("workflows.writeFailed", e))?;
                }
                std::fs::write(&target, content)
                    .map_err(|e| AppError::because("workflows.writeFailed", e))?;
                Ok(PushOutcome::Written)
            }
        }
    })
    .await
    .map_err(|e| AppError::because("workflows.writeFailed", e))?
}

/// Совместимость воркфлоу со всеми зарегистрированными сборками.
///
/// Считается для всех разом: пользователь выбирает, куда класть, и сравнить
/// он должен на одном экране, а не обходя инстансы по одному.
#[tauri::command]
#[specta::specta]
async fn workflow_compat(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    rel: String,
    nodes: Vec<String>,
) -> Result<Vec<InstanceCompat>, AppError> {
    let instances = instances::list(&app)?;
    let cache_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| AppError::because("workflows.scanFailed", e))?;

    let ports: Vec<(String, Option<u16>)> =
        instances.iter().map(|i| (i.id.clone(), running_port(&runtime, &i.id))).collect();

    tauri::async_runtime::spawn_blocking(move || {
        let mut out = Vec::new();
        for (instance, (_, port)) in instances.iter().zip(ports) {
            // Три состояния, и третье нельзя выдавать за первое: сборка,
            // о которой мы ничего не знаем, — это «неизвестно», а не
            // «всё хорошо». Зелёная галочка без оснований хуже её отсутствия.
            let (available, source) = match port {
                Some(port) => match comfy_api::Client::new(port).object_info_keys() {
                    Ok(keys) => {
                        comfy_api::cache::write(&cache_dir, &instance.id, &keys);
                        (Some(keys), CompatSource::Live)
                    }
                    Err(_) => (None, CompatSource::Unknown),
                },
                None => match comfy_api::cache::read(&cache_dir, &instance.id) {
                    Some(snapshot) => (Some(snapshot.nodes), CompatSource::Cached),
                    None => (None, CompatSource::Unknown),
                },
            };

            out.push(InstanceCompat {
                instance_id: instance.id.clone(),
                source,
                missing: available
                    .as_ref()
                    .map(|keys| workflows::missing_nodes(&nodes, keys))
                    .unwrap_or_default(),
                // Спрашивать по HTTP незачем даже у запущенной сборки:
                // ComfyUI хранит воркфлоу файлами, и ответ одинаково даёт
                // файловая система в обоих состояниях.
                present: local_workflows_dir(instance).join(&rel).is_file(),
            });
        }
        Ok(out)
    })
    .await
    .map_err(|e| AppError::because("workflows.scanFailed", e))?
}

/// Откуда взяты сведения о нодах сборки.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum CompatSource {
    /// Спросили у работающей сборки прямо сейчас.
    Live,
    /// По снимку с последнего запуска.
    Cached,
    /// Сборка не запущена и ни разу не запускалась при нас.
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InstanceCompat {
    pub instance_id: String,
    pub source: CompatSource,
    /// Пусто при `Unknown` — и это не «всё на месте», а «неизвестно».
    /// Различать обязан интерфейс, а не читатель.
    pub missing: Vec<String>,
    /// Этот воркфлоу уже лежит в сборке.
    ///
    /// Считается, а не запоминается. Прежде интерфейс знал только о наших
    /// собственных нажатиях в текущем сеансе и после перезахода показывал
    /// «добавить» у сборки, где файл уже был.
    pub present: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum PushOutcome {
    Written,
    /// Имя занято. Развилка, а не ошибка: спрашиваем пользователя.
    Conflict,
}

/// Порт работающей сборки, если она работает.
fn running_port(runtime: &Runtime, id: &str) -> Option<u16> {
    runtime
        .get(id)
        .and_then(|cell| {
            let running = cell.lock().unwrap();
            matches!(running.status.state, RunState::Running).then_some(running.status.port)
        })
        .flatten()
}

/// Папка воркфлоу остановленной сборки, по её первому профилю.
fn local_workflows_dir(instance: &Instance) -> std::path::PathBuf {
    let root = std::path::Path::new(&instance.path);
    match run::profiles_of(instance).first() {
        Some(profile) => profiles::workflows_dir(profile, root),
        // Профилей нет вовсе — берём умолчание ComfyUI.
        None => root.join("ComfyUI").join("user").join("default").join("workflows"),
    }
}

/// Имена воркфлоу остановленной сборки. Папки может не быть — это пусто,
/// а не ошибка.
fn local_workflow_names(instance: &Instance) -> Vec<String> {
    let dir = local_workflows_dir(instance);
    let scan = workflows::scan_library(&dir);
    scan.items.into_iter().filter(|i| !i.lost).map(|i| i.path).collect()
}

fn now_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0)
}

// ------------------------------------------------------- общие модели

#[tauri::command]
#[specta::specta]
async fn load_shared_settings(app: tauri::AppHandle) -> Result<SharedSettings, AppError> {
    settings::load_shared(&app)
}

#[tauri::command]
#[specta::specta]
async fn save_shared_settings(
    app: tauri::AppHandle,
    shared: SharedSettings,
) -> Result<(), AppError> {
    settings::save_shared(&app, &shared)
}

/// Сканирует папку и возвращает найденные категории.
///
/// Обход дерева в блокирующем потоке: на общей папке в сотни гигабайт это
/// десятки тысяч обращений к метаданным, и держать на них главный поток
/// нельзя — интерфейс замрёт ровно тогда, когда пользователь ждёт ответа.
#[tauri::command]
#[specta::specta]
async fn scan_shared_root(path: String) -> Result<RootScan, AppError> {
    tauri::async_runtime::spawn_blocking(move || {
        shared_models::scan_root(std::path::Path::new(&path))
    })
    .await
    .map_err(|e| AppError::because("shared.scanFailed", e))
}

/// YAML, который получится при текущих настройках.
///
/// Предпросмотр не косметика: пользователь должен видеть, что именно
/// попадёт в конфиг, до того как это попадёт в его сборку.
#[tauri::command]
#[specta::specta]
async fn preview_shared_yaml(shared: SharedSettings) -> Result<String, AppError> {
    tauri::async_runtime::spawn_blocking(move || shared_models::render_settings(&shared).yaml)
        .await
        .map_err(|e| AppError::because("shared.scanFailed", e))
}

/// Создаёт недостающие стандартные подпапки в общем корне.
#[tauri::command]
#[specta::specta]
async fn create_shared_folders(path: String, names: Vec<String>) -> Result<RootScan, AppError> {
    tauri::async_runtime::spawn_blocking(move || {
        let root = std::path::Path::new(&path);
        for name in &names {
            // Имя приходит из нашего же списка предложений, но проверить
            // дешевле, чем однажды создать папку на уровень выше корня.
            if name.contains(['/', '\\', ':']) || name.starts_with('.') {
                continue;
            }
            let _ = std::fs::create_dir_all(root.join(name));
        }
        shared_models::scan_root(root)
    })
    .await
    .map_err(|e| AppError::because("shared.scanFailed", e))
}

/// Что лежит в `extra_model_paths.yaml` инстанса.
#[tauri::command]
#[specta::specta]
async fn inspect_instance_config(
    app: tauri::AppHandle,
    id: String,
) -> Result<InstanceFileInfo, AppError> {
    let instance = find_instance(&app, &id)?;
    Ok(shared_models::inspect_instance_file(std::path::Path::new(&instance.path)))
}

/// Подключает инстанс к общим моделям.
///
/// `confirm_overwrite` относится только к режиму «файл в инстансе» и только
/// к случаю, когда там уже лежит чужой файл. Без согласия команда отказывает
/// кодом `shared.foreignConfig`, и фронт показывает экран сравнения.
#[tauri::command]
#[specta::specta]
async fn connect_shared(
    app: tauri::AppHandle,
    id: String,
    apply_mode: ApplyMode,
    confirm_overwrite: bool,
) -> Result<Option<String>, AppError> {
    let instance = find_instance(&app, &id)?;
    let settings = settings::load_shared(&app)?;
    let rendered = shared_models::render_settings(&settings);

    if rendered.empty {
        return Err(AppError::new("shared.noRoots"));
    }

    let mut backup = None;
    if apply_mode == ApplyMode::InstanceFile {
        let root = std::path::Path::new(&instance.path);
        let info = shared_models::inspect_instance_file(root);
        if info.state == InstanceFileState::Foreign && !confirm_overwrite {
            return Err(AppError::with("shared.foreignConfig", "path", info.path));
        }
        backup = shared_models::write_instance_file(root, &rendered.yaml, now_secs())?;
    }

    instances::set_shared(&app, &id, InstanceShared { enabled: true, apply_mode })?;
    Ok(backup)
}

/// Отключает инстанс от общих моделей.
///
/// В режиме «файл в инстансе» убирает наш файл и возвращает на место
/// сохранённую копию чужого. Модели в общей папке не трогаются никогда.
#[tauri::command]
#[specta::specta]
async fn disconnect_shared(app: tauri::AppHandle, id: String) -> Result<(), AppError> {
    let instance = find_instance(&app, &id)?;

    if instance.shared.apply_mode == ApplyMode::InstanceFile {
        shared_models::remove_instance_file(std::path::Path::new(&instance.path))?;
    }

    instances::set_shared(
        &app,
        &id,
        InstanceShared { enabled: false, apply_mode: instance.shared.apply_mode },
    )
}

fn find_instance(app: &tauri::AppHandle, id: &str) -> Result<Instance, AppError> {
    instances::list(app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", id))
}

/// Секунды эпохи — метка в имени резервной копии.
fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Готовит общие модели к запуску инстанса.
///
/// Возвращает путь конфига для `--extra-model-paths-config`, если инстанс
/// подключён в режиме флага. В режиме «файл в инстансе» флаг не нужен:
/// файл уже лежит в папке сборки и подхватывается ComfyUI сам.
fn prepare_shared(
    app: &tauri::AppHandle,
    instance: &Instance,
    without_shared: bool,
) -> Result<Option<String>, AppError> {
    if without_shared || !instance.shared.enabled {
        return Ok(None);
    }

    let settings = settings::load_shared(app)?;
    let rendered = shared_models::render_settings(&settings);

    // Проверка до запуска, а не после: внешний диск отключают, и узнавать
    // об этом из «model not found» посреди работы пользователь не должен.
    if let Some(path) = rendered.unavailable.first() {
        return Err(AppError::with("shared.rootUnavailable", "path", path));
    }
    if rendered.empty {
        return Ok(None);
    }

    if instance.shared.apply_mode == ApplyMode::InstanceFile {
        // Файл мог устареть: пользователь завёл в общей папке новую
        // категорию, а конфиг остался прежним. Обновляем — но только свой.
        // Чужой на этом месте означает, что его положили после нас,
        // и трогать его при обычном запуске мы права не имеем.
        let root = std::path::Path::new(&instance.path);
        if shared_models::inspect_instance_file(root).state == InstanceFileState::Ours {
            shared_models::write_config(
                &shared_models::instance_config_path(root),
                &rendered.yaml,
            )?;
        }
        return Ok(None);
    }

    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::because("shared.writeFailed", e))?;
    let path = shared_models::flag_config_path(&dir);
    shared_models::write_config(&path, &rendered.yaml)?;
    Ok(Some(path.display().to_string()))
}

/// Запускает инстанс.
///
/// `without_shared` — согласие запуститься без общих моделей.
/// Недоступный корень не может просто игнорироваться: пользователь,
/// державший модели на внешнем диске, получил бы «model not found» посреди
/// работы и решил бы, что сломалось приложение. Поэтому первый вызов
/// отказывает кодом `shared.rootUnavailable`, фронт показывает выбор,
/// и повторный вызов приходит уже с согласием.
#[tauri::command]
#[specta::specta]
async fn start_instance(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    profile_id: Option<String>,
    without_shared: bool,
) -> Result<RunStatus, AppError> {
    if runtime.is_busy(&id) {
        return Err(AppError::new("run.alreadyRunning"));
    }

    let instance = instances::list(&app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", &id))?;
    if !instance.available {
        return Err(AppError::with("instances.missing", "path", &instance.path));
    }

    let all = run::profiles_of(&instance);
    let profile = profile_id
        .as_deref()
        .and_then(|want| all.iter().find(|p| p.id == want))
        .or_else(|| all.first())
        .ok_or_else(|| AppError::new("run.noProfiles"))?
        .clone();

    let shared_config = prepare_shared(&app, &instance, without_shared)?;

    let emitter = app.clone();
    let on_line = std::sync::Arc::new({
        let id = id.clone();
        move |line: crate::process::LogLine| {
            let _ = RunLog { instance_id: id.clone(), line }.emit(&emitter);
        }
    });

    let exit_app = app.clone();
    let exit_id = id.clone();
    let outcome = run::start(&instance, &profile, shared_config.as_deref(), on_line, move |exit| {
        finish(&exit_app, &exit_id, exit);
    })?;

    runtime.insert(&id, outcome.cell.clone());
    let _ = RunChanged(outcome.status.clone()).emit(&app);

    // Готовность ждём в фоне: команда обязана вернуться сразу, иначе
    // интерфейс не покажет ни строчки до конца холодного старта.
    let ready_app = app.clone();
    let ready_cell = outcome.cell.clone();
    let port = outcome.status.port.unwrap_or_default();
    std::thread::spawn(move || {
        let keep = {
            let cell = ready_cell.clone();
            move || {
                matches!(cell.lock().unwrap().status.state, RunState::Starting)
            }
        };
        match crate::process::wait_ready(port, crate::process::READY_TIMEOUT, keep) {
            Ok(secs) => {
                let mut running = ready_cell.lock().unwrap();
                if running.status.state == RunState::Starting {
                    running.status.state = RunState::Running;
                    running.status.ready_secs = Some(secs);
                    let _ = RunChanged(running.status.clone()).emit(&ready_app);
                }
            }
            Err(_) => {
                let mut running = ready_cell.lock().unwrap();
                if running.status.state == RunState::Starting {
                    running.status.state = RunState::Crashed;
                    let _ = RunChanged(running.status.clone()).emit(&ready_app);
                }
            }
        }
    });

    Ok(outcome.status)
}

/// Разбирает, чем кончился процесс, и сообщает наверх.
fn finish(app: &tauri::AppHandle, id: &str, exit: run::Exit) {
    let runtime = app.state::<Runtime>();
    let Some(cell) = runtime.get(id) else { return };
    let mut running = cell.lock().unwrap();

    match exit {
        run::Exit::Requested => {
            running.status.state = RunState::Stopped;
            running.status.pid = None;
        }
        run::Exit::Crashed(code) => {
            running.status.state = RunState::Crashed;
            running.status.exit_code = code;
            running.status.pid = None;
        }
        run::Exit::Detached => {
            // Сервер жив, но не наш. Управлять им мы больше не можем,
            // и делать вид, что можем, нельзя.
            running.status.state = RunState::Detached;
            running.status.pid = None;
        }
    }
    let _ = RunChanged(running.status.clone()).emit(app);
}

#[tauri::command]
#[specta::specta]
async fn stop_instance(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
) -> Result<(), AppError> {
    let cell = runtime
        .get(&id)
        .ok_or_else(|| AppError::new("run.notRunning"))?;
    let _ = RunChanged(RunStatus {
        state: RunState::Stopping,
        ..cell.lock().unwrap().status.clone()
    })
    .emit(&app);
    run::stop(&cell)
}

/// Считает размер инстанса на диске.
///
/// Команда `async`, поэтому выполняется не в главном потоке и интерфейс
/// не замирает на все минуты обхода. `None` означает, что подсчёт уже идёт.
#[tauri::command]
#[specta::specta]
async fn measure_instance_size(
    app: tauri::AppHandle,
    jobs: tauri::State<'_, SizeJobs>,
    id: String,
) -> Result<Option<Sized_>, AppError> {
    instances::measure_size(&app, &jobs, &id)
}

/// Реальная установка для спайка. Захардкожена намеренно: реестр появится в Фазе 1.
const INSTANCE_DIR: &str =
    r"d:\program_files\comfyui\ComfyUI_windows_portable_nvidia\ComfyUI_windows_portable";
const PORT: u16 = 8188;

/// Скрывает окно консоли у дочернего процесса. Без него при каждом запуске
/// поверх интерфейса всплывал бы чёрный терминал.
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[derive(Default)]
struct SpikeState {
    child: Mutex<Option<Child>>,
}

/// Событие с строкой лога спайка.
///
/// Имя с префиксом Spike не для красоты: tauri-specta выводит имя
/// события из имени структуры, и совпадение с LogLine из process.rs
/// роняло экспорт типов.
///
/// Типы генерируются в `src/bindings.ts`: описывать одну и ту же модель
/// на Rust и на TypeScript руками — верный способ получить молчаливое
/// расхождение, а этих моделей дальше будет много.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
struct SpikeLog {
    stream: String,
    text: String,
}

/// Сервер поднялся и готов принимать запросы.
///
/// Автопрогон не встраивает вкладку сам: прямоугольник знает только фронт,
/// у которого есть `ResizeObserver`. Захардкоженный размер в Rust дал бы
/// вкладку не по месту.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
struct SpikeReady {
    port: u16,
    secs: u32,
}

/// Запускает ComfyUI и стримит его вывод событиями `comfy-log`.
///
/// Ключевые флаги: `--disable-auto-launch` не даёт открыться браузеру
/// (в cli_args.py он применяется после `--windows-standalone-build`
/// и всегда побеждает), `--port` фиксирует порт.
/// Команды объявлены `async` намеренно.
///
/// Синхронная команда Tauri выполняется в главном потоке. Для `wait_ready`
/// это заморозило бы интерфейс на все минуты холодного старта, а для
/// `embed_comfy` дало бы взаимную блокировку: `add_child` изнутри ставит
/// задачу в главный поток и ждёт её результата.
#[tauri::command]
#[specta::specta]
async fn start_comfy(
    app: tauri::AppHandle,
    state: tauri::State<'_, SpikeState>,
) -> Result<u16, AppError> {
    spawn_comfy(&app, &state)
}

fn spawn_comfy(app: &tauri::AppHandle, state: &SpikeState) -> Result<u16, AppError> {
    if state.child.lock().unwrap().is_some() {
        return Err(AppError::new("comfy.alreadyRunning"));
    }

    let python = format!(r"{INSTANCE_DIR}\python_embeded\python.exe");
    let main_py = format!(r"{INSTANCE_DIR}\ComfyUI\main.py");

    let mut cmd = Command::new(&python);
    cmd.args([
        "-s",
        &main_py,
        "--windows-standalone-build",
        "--port",
        &PORT.to_string(),
        "--disable-auto-launch",
    ])
    .current_dir(INSTANCE_DIR)
    // Без этого stdout буферизуется блоками при перенаправлении в пайп,
    // и первые минуты старта выглядят как зависание.
    .env("PYTHONUNBUFFERED", "1")
    .env("PYTHONIOENCODING", "utf-8")
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = cmd
        .spawn()
        .map_err(|e| AppError::because("comfy.spawnFailed", e))?;

    // ComfyUI пишет основную часть старта в stderr, а не в stdout,
    // поэтому читаем оба потока.
    if let Some(out) = child.stdout.take() {
        pump(app.clone(), out, "stdout");
    }
    if let Some(err) = child.stderr.take() {
        pump(app.clone(), err, "stderr");
    }

    *state.child.lock().unwrap() = Some(child);
    Ok(PORT)
}

/// Читает поток построчно в отдельном треде и шлёт каждую строку во фронт.
///
/// Первая строка дополнительно печатается в терминал: по ней видно,
/// действительно ли стриминг живой, или вывод пришёл пачкой в конце.
fn pump<R: Read + Send + 'static>(app: tauri::AppHandle, stream: R, name: &'static str) {
    std::thread::spawn(move || {
        let started = Instant::now();
        let mut first = true;
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            let Ok(text) = line else { break };
            if first {
                first = false;
                report(&format!(
                    "первая строка {name} через {:.1} с: {}",
                    started.elapsed().as_secs_f32(),
                    text.chars().take(60).collect::<String>()
                ));
            }
            let _ = SpikeLog { stream: name.to_string(), text }.emit(&app);
        }
    });
}

/// Печатает факт спайка в терминал `tauri dev` с приметным префиксом,
/// чтобы результаты было видно среди логов сборки.
fn report(msg: &str) {
    println!("[СПАЙК] {msg}");
}

/// Опрашивает `/system_stats`, пока сервер не ответит.
///
/// Реализовано на голом TcpStream осознанно: тянуть HTTP-клиент ради
/// одного запроса в спайк — лишние минуты компиляции.
#[tauri::command]
#[specta::specta]
async fn wait_ready(port: u16, timeout_secs: u32) -> Result<u32, AppError> {
    wait_ready_inner(port, timeout_secs)
}

fn wait_ready_inner(port: u16, timeout_secs: u32) -> Result<u32, AppError> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs.into());
    let started = Instant::now();

    while Instant::now() < deadline {
        if probe(port) {
            return Ok(started.elapsed().as_secs() as u32);
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    Err(AppError::with("comfy.readyTimeout", "secs", timeout_secs))
}

fn probe(port: u16) -> bool {
    let addr = format!("127.0.0.1:{port}");
    let Ok(mut sock) = TcpStream::connect_timeout(
        &addr.parse().expect("валидный адрес"),
        Duration::from_millis(500),
    ) else {
        return false;
    };
    let _ = sock.set_read_timeout(Some(Duration::from_millis(1500)));

    let req = format!("GET /system_stats HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
    if sock.write_all(req.as_bytes()).is_err() {
        return false;
    }
    let mut head = [0u8; 32];
    match sock.read(&mut head) {
        Ok(n) => String::from_utf8_lossy(&head[..n]).contains("200"),
        Err(_) => false,
    }
}

/// Главная проверка фазы: ComfyUI внутри нашего окна.
///
/// `<iframe>` здесь получил бы 403 — origin_only_middleware режет всё
/// с `Sec-Fetch-Site: cross-site`. Дочерний вебвью грузит страницу как
/// навигацию верхнего уровня, и middleware пропускает без единого
/// послабления в настройках сервера.
#[tauri::command]
#[specta::specta]
async fn embed_comfy(
    app: tauri::AppHandle,
    port: u16,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) -> Result<(), AppError> {
    embed_inner(&app, port, x, y, w, h)
}

fn embed_inner(
    app: &tauri::AppHandle,
    port: u16,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) -> Result<(), AppError> {
    let embed_failed = |e: tauri::Error| AppError::because("webview.embedFailed", e);

    let window = app
        .get_window("main")
        .ok_or_else(|| AppError::because("webview.embedFailed", "нет окна main"))?;

    if let Some(existing) = app.get_webview("comfy") {
        existing
            .set_position(LogicalPosition::new(x, y))
            .map_err(embed_failed)?;
        existing
            .set_size(LogicalSize::new(w, h))
            .map_err(embed_failed)?;
        // Возврат на экран инстанса: вкладку показываем и ставим на место
        // одним действием, иначе она мигнёт на старом прямоугольнике.
        existing.show().map_err(embed_failed)?;
        return Ok(());
    }

    let url = format!("http://127.0.0.1:{port}")
        .parse()
        .map_err(|_| AppError::because("webview.embedFailed", "плохой URL"))?;

    let probe_app = app.clone();
    let title_app = app.clone();

    let builder = tauri::webview::WebviewBuilder::new("comfy", WebviewUrl::External(url))
        // Иначе Tauri перехватит системный дроп, и перетаскивание картинок
        // и воркфлоу на холст ComfyUI перестанет работать.
        .disable_drag_drop_handler()
        .on_page_load(move |view, payload| {
            report(&format!("вкладка загрузила {}", payload.url()));
            let _ = SpikeLog {
                stream: "webview".into(),
                text: format!("страница загружена: {}", payload.url()),
            }
            .emit(&probe_app);
            // Заголовок — единственный канал обратно из чужого origin:
            // наш IPC там не доступен. Кладём в него начало текста страницы,
            // чтобы увидеть, отдал ли сервер интерфейс или 403.
            let _ = view.eval(
                "document.title = 'CPO|' + document.title + '|' \
                 + (document.body ? document.body.innerText.slice(0, 90) : '(нет body)')",
            );
        })
        .on_document_title_changed(move |_view, title| {
            if let Some(rest) = title.strip_prefix("CPO|") {
                // Главный результат фазы: что реально отдал сервер вкладке.
                // Если бы origin-middleware отбила запрос, здесь было бы 403.
                report(&format!("вкладка видит: {rest}"));
                let _ = SpikeLog {
                    stream: "webview".into(),
                    text: format!("вебвью видит: {rest}"),
                }
                .emit(&title_app);
            }
        });

    window
        .add_child(
            builder,
            LogicalPosition::new(x, y),
            LogicalSize::new(w, h),
        )
        .map_err(embed_failed)?;

    Ok(())
}

/// Прячет вкладку, не останавливая сервер.
///
/// Уход с экрана инстанса в любой другой раздел обязан скрыть дочерний
/// вебвью: он нативное окно поверх нашего HTML и иначе закроет собой
/// открытый раздел. Процесс при этом продолжает работать — останавливает
/// его только явная команда.
#[tauri::command]
#[specta::specta]
async fn hide_comfy(app: tauri::AppHandle) -> Result<(), AppError> {
    if let Some(view) = app.get_webview("comfy") {
        view.hide()
            .map_err(|e| AppError::because("webview.embedFailed", e))?;
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
async fn stop_comfy(
    app: tauri::AppHandle,
    state: tauri::State<'_, SpikeState>,
) -> Result<(), AppError> {
    if let Some(view) = app.get_webview("comfy") {
        let _ = view.close();
    }
    if let Some(mut child) = state.child.lock().unwrap().take() {
        // На Windows послать SIGINT чужому процессу нельзя, поэтому
        // в Фазе 2 здесь будет taskkill /T /F. Для спайка достаточно kill.
        let _ = child.kill();
        let _ = child.wait();
    }
    Ok(())
}

/// Прогоняет спайк целиком без участия человека.
///
/// Включается переменной `CPO_SPIKE=1`. Нужен потому, что проверить
/// результат кликом по кнопке можно только руками, а решение фазы
/// хочется получать воспроизводимо и в логе.
fn autorun(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        report("автопрогон включён (CPO_SPIKE=1)");
        let state = app.state::<SpikeState>();

        let port = match spawn_comfy(&app, &state) {
            Ok(p) => {
                report(&format!("процесс запущен, порт {p}"));
                p
            }
            Err(e) => {
                report(&format!("ПРОВАЛ: не удалось запустить: {}", e.code));
                return;
            }
        };

        let secs = match wait_ready_inner(port, 300) {
            Ok(secs) => {
                report(&format!("сервер готов за {secs} с"));
                secs
            }
            Err(e) => {
                report(&format!("ПРОВАЛ: {}", e.code));
                return;
            }
        };

        // Встраивание отдаём фронту: прямоугольник знает он.
        if let Err(e) = (SpikeReady { port, secs }).emit(&app) {
            report(&format!("ПРОВАЛ: не удалось сообщить о готовности: {e}"));
        }
    });
}

/// Единый список команд и событий: и обработчик вызовов, и генератор типов
/// берутся отсюда, поэтому разойтись они не могут.
fn specta_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            load_bootstrap,
            save_settings,
            list_instances,
            probe_folder,
            suggest_accent,
            add_instance,
            update_instance,
            remove_instance,
            measure_instance_size,
            probe_archive,
            check_targets,
            archive_history,
            forget_archive,
            run_install,
            cancel_install,
            instance_profiles,
            suggest_library_path,
            load_library_settings,
            save_library_settings,
            scan_library,
            add_workflow_file,
            set_workflow_meta,
            forget_workflow,
            instance_workflows,
            pull_workflow,
            push_workflow,
            workflow_compat,
            load_shared_settings,
            save_shared_settings,
            scan_shared_root,
            preview_shared_yaml,
            create_shared_folders,
            inspect_instance_config,
            connect_shared,
            disconnect_shared,
            run_statuses,
            run_log,
            start_instance,
            stop_instance,
            start_comfy,
            wait_ready,
            embed_comfy,
            hide_comfy,
            stop_comfy
        ])
        .events(tauri_specta::collect_events![
            SpikeLog,
            SpikeReady,
            InstallProgress,
            RunLog,
            RunChanged
        ])
}

/// Выгружает типы в `src/bindings.ts`.
///
/// Вызывается и из дев-сборки, и из теста. Тест важнее: он генерирует
/// типы без запуска окна, поэтому работает в CI и не требует ни дисплея,
/// ни дев-сервера Vite.
#[cfg(debug_assertions)]
fn export_bindings(builder: &tauri_specta::Builder<tauri::Wry>) {
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("не удалось выгрузить типы в bindings.ts");
}

#[cfg(test)]
mod tests {
    /// Держит `src/bindings.ts` в согласии с сигнатурами команд.
    /// Ломается ровно тогда, когда изменился контракт, — и это хорошо.
    #[test]
    fn bindings_up_to_date() {
        super::export_bindings(&super::specta_builder());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    #[cfg(debug_assertions)]
    export_bindings(&builder);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(SpikeState::default())
        .manage(SizeJobs::default())
        .manage(InstallLock::default())
        .manage(InstallCancel::default())
        .manage(Runtime::default())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            // Обязательно: без mount_events типизированные события
            // не доедут до фронта.
            builder.mount_events(app);

            // Job Object до всего остального: сборки, запущенные до его
            // установки, переживут падение приложения и оставят занятой
            // видеопамять.
            if let Err(e) = supervise::windows::install_job_object() {
                eprintln!("[CPO] job object не создан: {e}. Дочерние процессы могут пережить приложение.");
            }

            if std::env::var("CPO_SPIKE").as_deref() == Ok("1") {
                autorun(app.handle().clone());
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("не удалось запустить приложение");
}
