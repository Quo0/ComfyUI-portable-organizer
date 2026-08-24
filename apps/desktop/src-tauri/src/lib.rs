// Copyright (C) 2026 Andrew Blokhin
// SPDX-License-Identifier: GPL-3.0-only

//! Точка сборки приложения: команды Tauri тонкими обёртками над модулями.
//!
//! Здесь же — список команд и событий для `tauri-specta`, из которого
//! генерируется `src/bindings.ts`.

// Модули публичные: по ним ходят примеры в examples/, которыми
// проверяется распаковка на реальном архиве.
pub mod comfy_api;
pub mod discovery;
pub mod duplicates;
pub mod error;
pub mod installer;
pub mod migrate;
pub mod instances;
pub mod ports;
pub mod process;
pub mod profiles;
pub mod run;
pub mod settings;
pub mod shared_models;
pub mod supervise;
pub mod tray;
pub mod update;
pub mod webview;
pub mod workflows;

use serde::{Deserialize, Serialize};
use tauri::Manager;
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
use crate::migrate::ModelsScan;
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
            accent: target.accent.clone(),
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

/// Итоговая команда запуска — та, что реально уйдёт системе.
///
/// Показывается в редакторе аргументов: `--port` и `--disable-auto-launch`
/// дописываются нами, и без предпросмотра пользователь спорил бы
/// с невидимым.
#[tauri::command]
#[specta::specta]
async fn preview_command(
    app: tauri::AppHandle,
    id: String,
    profile_id: String,
) -> Result<Vec<String>, AppError> {
    let instance = instances::list(&app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", &id))?;

    let all = run::profiles_of(&instance);
    let profile = all
        .iter()
        .find(|p| p.id == profile_id)
        .ok_or_else(|| AppError::new("run.noProfiles"))?;

    // Порт показываем предпочитаемый: настоящий выдаётся при старте,
    // и обещать конкретное число заранее нельзя.
    let mut line = vec![profile.python_path.clone()];
    line.extend(profiles::apply_runtime_args(
        &profile.args,
        instance.preferred_port,
        None,
    ));
    Ok(line)
}

/// Сохраняет свой профиль запуска.
#[tauri::command]
#[specta::specta]
async fn save_custom_profile(
    app: tauri::AppHandle,
    id: String,
    profile: instances::CustomProfile,
) -> Result<Instance, AppError> {
    instances::save_profile(&app, &id, profile)
}

#[tauri::command]
#[specta::specta]
async fn remove_custom_profile(
    app: tauri::AppHandle,
    id: String,
    profile_id: String,
) -> Result<Instance, AppError> {
    instances::remove_profile(&app, &id, &profile_id)
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

// -------------------------------------------- перенос моделей в общую

/// Модели этой сборки и что из них уже есть в общей папке.
#[tauri::command]
#[specta::specta]
async fn scan_instance_models(
    app: tauri::AppHandle,
    id: String,
) -> Result<ModelsScan, AppError> {
    let (models, shared) = migrate_paths(&app, &id)?;
    tauri::async_runtime::spawn_blocking(move || migrate::scan(&models, &shared))
        .await
        .map_err(|e| AppError::because("migrate.readFailed", e))
}

/// Переносит выбранные модели в общую папку.
///
/// Выбор приходит парами «категория и модель», как и у уборки: тумблер
/// на экране стоит у каждой модели, а не у категории целиком.
///
/// Отказывает у работающей сборки: забирать файлы из-под работающего
/// ComfyUI нельзя — он держит их открытыми и уже разобрал пути при старте.
#[tauri::command]
#[specta::specta]
async fn migrate_models(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    cancel: tauri::State<'_, migrate::MigrateCancel>,
    id: String,
    items: Vec<(String, String)>,
) -> Result<migrate::MigrateOutcome, AppError> {
    if running_port(&runtime, &id).is_some() {
        return Err(AppError::new("migrate.instanceRunning"));
    }
    let (models, shared) = migrate_paths(&app, &id)?;
    cancel.reset();

    // Место проверяется до начала, а не на середине: узнать о нехватке
    // на девятнадцатом гигабайте из двадцати — худшее из возможного.
    let scan = migrate::scan(&models, &shared);
    let need: f64 = scan
        .categories
        .iter()
        .flat_map(|c| {
            c.entries.iter().filter(|e| {
                e.same_name.is_none()
                    && items.iter().any(|(cat, name)| cat == &c.folder && name == &e.name)
            })
        })
        .map(|e| e.size_bytes)
        .sum();
    if !migrate::enough_space(&shared, need) {
        return Err(AppError::with("migrate.noSpace", "path", shared.display()));
    }

    // Перенос десятков гигабайт уходит в отдельный поток, а не на воркер
    // асинхронного рантайма: иначе отмена соревнуется за тот же воркер.
    let emitter = app.clone();
    let flag = cancel.share();
    tauri::async_runtime::spawn_blocking(move || {
        migrate::move_all(&models, &shared, &items, &flag, |progress| {
            let _ = progress.emit(&emitter);
        })
    })
    .await
    .map_err(|e| AppError::because("migrate.writeFailed", e))
}

#[tauri::command]
#[specta::specta]
async fn cancel_migrate(cancel: tauri::State<'_, migrate::MigrateCancel>) -> Result<(), AppError> {
    cancel.cancel();
    Ok(())
}

/// Убирает из сборки то, что уже лежит в общей папке.
///
/// Три условия проверяются здесь, а не только на экране: сборка
/// остановлена, подключена к общим моделям, и каждый элемент заново
/// признан дубликатом внутри `remove_duplicates`. Удаление — не то место,
/// где можно доверять входным данным.
#[tauri::command]
#[specta::specta]
async fn remove_duplicate_models(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    items: Vec<(String, String)>,
) -> Result<migrate::CleanupOutcome, AppError> {
    if running_port(&runtime, &id).is_some() {
        return Err(AppError::new("migrate.instanceRunning"));
    }
    let instance = find_instance(&app, &id)?;
    if !instance.shared.enabled {
        // Не подключена — удаление лишило бы её моделей вовсе.
        // Это не уборка, а поломка.
        return Err(AppError::new("migrate.notConnected"));
    }

    let (models, shared) = migrate_paths(&app, &id)?;
    tauri::async_runtime::spawn_blocking(move || {
        migrate::remove_duplicates(&models, &shared, &items)
    })
    .await
    .map_err(|e| AppError::because("migrate.removeFailed", e))
}

/// Папка моделей сборки и общий корень.
fn migrate_paths(
    app: &tauri::AppHandle,
    id: &str,
) -> Result<(std::path::PathBuf, std::path::PathBuf), AppError> {
    let instance = find_instance(app, id)?;
    if !instance.available {
        return Err(AppError::with("instances.missing", "path", &instance.path));
    }

    let shared = migrate::first_root(&settings::load_shared(app)?)
        .ok_or_else(|| AppError::new("shared.noRoots"))?;

    let root = std::path::Path::new(&instance.path);
    let models = match run::profiles_of(&instance).first() {
        Some(profile) => profiles::models_dir(profile, root),
        None => root.join("ComfyUI").join("models"),
    };
    Ok((models, shared))
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
/// Принимает и `.json`, и `.png`: картинка из папки `output` носит граф
/// с собой, и «перетащить картинку» — самый частый способ вернуться
/// к удачной генерации. В библиотеку в обоих случаях ложится `.json` —
/// именно он потом уедет в инстанс.
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
    let path = std::path::Path::new(&source);
    let is_png = path
        .extension()
        .map(|e| e.eq_ignore_ascii_case("png"))
        .unwrap_or(false);

    let content = if is_png {
        let bytes = std::fs::read(path).map_err(|e| AppError::because("workflows.readFailed", e))?;
        workflows::workflow_from_png(&bytes)
            .ok_or_else(|| AppError::with("workflows.noGraphInImage", "path", &source))?
    } else {
        std::fs::read_to_string(path).map_err(|e| AppError::because("workflows.readFailed", e))?
    };

    if workflows::node_types(&content).is_none() {
        return Err(AppError::with("workflows.notAWorkflow", "path", &source));
    }

    let name = rel.unwrap_or_else(|| {
        let stem = path
            .file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "workflow".to_string());
        if is_png {
            format!("{stem}.json")
        } else {
            path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "workflow.json".to_string())
        }
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

/// Кладёт в библиотеку граф, пришедший текстом.
///
/// Тот же конец пути, что и у файла, только источник другой: воркфлоу
/// чаще присылают текстом — в чате, на форуме, — чем файлом, и сохранять
/// присланное в файл только ради того, чтобы тут же выбрать его в диалоге,
/// — лишний круг.
///
/// Имя приходит из поля ввода, поэтому проверяется здесь, а не доверяется:
/// оно попадает в путь. Перезаписи нет и тут — занятое имя это отказ,
/// а не замена: заменить значило бы затереть одну работу другой.
#[tauri::command]
#[specta::specta]
async fn add_workflow_text(
    library: String,
    name: String,
    content: String,
) -> Result<String, AppError> {
    let rel = workflows::file_name_from_input(&name)
        .ok_or_else(|| AppError::with("workflows.badName", "name", &name))?;

    if workflows::node_types(&content).is_none() {
        return Err(AppError::with("workflows.notAWorkflow", "path", &rel));
    }

    let target = std::path::Path::new(&library).join(&rel);
    if target.exists() {
        return Err(AppError::with("workflows.nameTaken", "path", &rel));
    }
    std::fs::write(&target, content).map_err(|e| AppError::because("workflows.writeFailed", e))?;

    Ok(rel)
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
    library: String,
) -> Result<Vec<workflows::InstanceWorkflow>, AppError> {
    let instance = find_instance(&app, &id)?;
    let port = running_port(&runtime, &id);

    tauri::async_runtime::spawn_blocking(move || {
        let client = port.map(comfy_api::Client::new);
        let names: Vec<String> = match &client {
            Some(client) => client.list_workflows()?.into_iter().map(|f| f.path).collect(),
            None => local_workflow_names(&instance),
        };

        let root = std::path::Path::new(&library);
        let dir = local_workflows_dir(&instance);

        let mut out = Vec::with_capacity(names.len());
        for path in names {
            // Читаем только то, чьё имя в библиотеке занято. У остальных
            // сверять не с чем, а список бывает в сотни файлов — и у
            // запущенной сборки каждое чтение это запрос по HTTP.
            let twin = root.join(&path);
            let verdict = if library.is_empty() || !twin.is_file() {
                None
            } else {
                let mine = match &client {
                    Some(client) => client.read_workflow(&path).ok(),
                    None => std::fs::read_to_string(dir.join(&path)).ok(),
                };
                match (mine, std::fs::read_to_string(&twin).ok()) {
                    (Some(a), Some(b)) if workflows::same_workflow(&a, &b) => {
                        Some(workflows::LibraryMatch::Same)
                    }
                    // Не прочиталось — считаем разошедшимися. Так кнопка
                    // остаётся рабочей: «не смогли сверить» не повод
                    // объявлять чужую работу уже сохранённой.
                    _ => Some(workflows::LibraryMatch::Diverged),
                }
            };
            out.push(workflows::InstanceWorkflow { path, library: verdict });
        }

        Ok(out)
    })
    .await
    .map_err(|e| AppError::because("workflows.scanFailed", e))?
}

/// Папка воркфлоу сборки — для кнопки «показать в проводнике».
///
/// Существование проверяем здесь: у остановленной сборки, которая ещё
/// ничего не сохраняла, папки нет вовсе, и звать проводник не с чем.
#[tauri::command]
#[specta::specta]
async fn instance_workflows_dir(
    app: tauri::AppHandle,
    id: String,
) -> Result<workflows::InstanceWorkflowsDir, AppError> {
    let instance = find_instance(&app, &id)?;
    let dir = local_workflows_dir(&instance);
    Ok(workflows::InstanceWorkflowsDir {
        available: dir.is_dir(),
        path: dir.to_string_lossy().into_owned(),
    })
}

/// Переносит воркфлоу из сборки в библиотеку: в сборке его не остаётся.
///
/// Порядок здесь — единственная защита от потери чужой работы, и он тот же,
/// что у переноса моделей: пишем копию, **читаем её обратно и сверяем**,
/// и только потом убираем исходник. Пока копия не проверена, оригинал
/// должен оставаться на руках.
///
/// Перезаписи нет вовсе. Раньше на занятое имя был вопрос «заменить?»,
/// и он был безобиден, пока забор был копированием: при любом ответе
/// воркфлоу оставался в сборке. У переноса цена ответа другая — «заменить»
/// затирало бы одну работу другой, не оставляя копии ни той, ни другой.
///
/// Вместо замены — `target`: забрать под свободным именем. Занятое имя
/// перестало быть тупиком, но разошедшиеся версии остаются двумя разными
/// файлами, а не одним поверх другого.
///
/// У запущенной сборки исходник убирает она сама, своим API: папка
/// воркфлоу принадлежит ей, и о правках со стороны она не знает.
#[tauri::command]
#[specta::specta]
async fn pull_workflow(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    rel: String,
    library: String,
    target: Option<String>,
) -> Result<String, AppError> {
    let instance = find_instance(&app, &id)?;
    let port = running_port(&runtime, &id);

    tauri::async_runtime::spawn_blocking(move || {
        // `rel` — где лежит в сборке, `dest` — под каким именем ляжет
        // в библиотеку. Совпадают всегда, кроме забора разошедшейся версии.
        let dest = target.unwrap_or_else(|| rel.clone());

        let local = local_workflows_dir(&instance).join(&rel);
        let content = match port {
            Some(port) => comfy_api::Client::new(port).read_workflow(&rel)?,
            None => std::fs::read_to_string(&local)
                .map_err(|e| AppError::because("workflows.readFailed", e))?,
        };

        if workflows::node_types(&content).is_none() {
            return Err(AppError::with("workflows.notAWorkflow", "path", &rel));
        }

        let target = std::path::Path::new(&library).join(&dest);
        if target.exists() {
            return Err(AppError::with("workflows.nameTaken", "path", &dest));
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::because("workflows.writeFailed", e))?;
        }
        std::fs::write(&target, &content)
            .map_err(|e| AppError::because("workflows.writeFailed", e))?;

        // Сверка, а не доверие к успешному `write`: дальше идёт удаление,
        // и оно должно опираться на прочитанное с диска, а не на то,
        // что запись не вернула ошибку.
        let written = std::fs::read_to_string(&target)
            .map_err(|e| AppError::because("workflows.verifyFailed", e))?;
        if written != content {
            let _ = std::fs::remove_file(&target);
            return Err(AppError::new("workflows.verifyFailed"));
        }

        // Помним, откуда взяли: через полгода это единственный способ
        // понять, почему воркфлоу требует именно этих нод.
        let root = std::path::Path::new(&library);
        let (mut manifest, _) = workflows::read_manifest(root);
        let entry = manifest.items.entry(dest.clone()).or_default();
        entry.source_instance_id = Some(id.clone());
        if entry.added_at.is_none() {
            entry.added_at = Some(now_ms());
        }
        workflows::write_manifest(root, &manifest)?;

        // Копия на месте и проверена — теперь можно убирать исходник.
        // Сбой на этом шаге не теряет ничего: воркфлоу окажется и в сборке,
        // и в библиотеке, а сверка при следующем чтении списка признает их
        // одним и тем же и погасит кнопку.
        match port {
            Some(port) => comfy_api::Client::new(port).delete_workflow(&rel)?,
            None => std::fs::remove_file(&local)
                .map_err(|e| AppError::because("workflows.removeFailed", e))?,
        }

        Ok(dest)
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

/// Согласия пользователя, без которых запуск отказывается идти.
///
/// Обе развилки устроены одинаково: первый вызов приходит с пустыми
/// согласиями и получает отказ с кодом, фронт раскрывает выбор на месте,
/// повторный вызов приходит уже с ответом. Модалку сюда положить нельзя
/// (дисциплина z-order), а у тоста не бывает кнопок.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct StartOptions {
    /// Запускаться, даже если общий корень моделей недоступен.
    pub without_shared: bool,
    /// Запускаться, даже если другая сборка уже работает.
    pub allow_multiple: bool,
}

/// Запускает инстанс.
///
/// Недоступный общий корень не может просто игнорироваться: пользователь,
/// державший модели на внешнем диске, получил бы «model not found» посреди
/// работы и решил бы, что сломалось приложение.
///
/// Вторая сборка на той же видеокарте — это отказ по нехватке видеопамяти
/// уже в очереди генерации, и понять его со стороны ComfyUI невозможно.
#[tauri::command]
#[specta::specta]
async fn start_instance(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    profile_id: Option<String>,
    options: StartOptions,
) -> Result<RunStatus, AppError> {
    start_inner(&app, &runtime, id, profile_id, options)
}

fn start_inner(
    app: &tauri::AppHandle,
    runtime: &Runtime,
    id: String,
    profile_id: Option<String>,
    options: StartOptions,
) -> Result<RunStatus, AppError> {
    if runtime.is_busy(&id) {
        return Err(AppError::new("run.alreadyRunning"));
    }

    if !options.allow_multiple {
        if let Some(other) = other_running(app, runtime, &id) {
            return Err(AppError::with("run.otherRunning", "name", other));
        }
    }

    let instance = instances::list(app)?
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

    let shared_config = prepare_shared(app, &instance, options.without_shared)?;

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
    // Дата последнего запуска — справочная, и провал её записи запуску
    // не помеха: сборка уже стартовала.
    let _ = instances::mark_started(app, &id);
    let _ = RunChanged(outcome.status.clone()).emit(app);

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

/// Имя другой работающей сборки, если такая есть.
///
/// Имя, а не идентификатор: сообщение показывается пользователю, и «i17550…»
/// ему ничего не скажет. Инстанс мог исчезнуть из реестра — тогда
/// довольствуемся идентификатором, лишь бы не промолчать.
fn other_running(app: &tauri::AppHandle, runtime: &Runtime, id: &str) -> Option<String> {
    let busy = tray::busy(runtime);
    let other = busy.into_iter().find(|other| other != id)?;
    let name = instances::list(app)
        .ok()
        .and_then(|all| all.into_iter().find(|i| i.id == other).map(|i| i.name));
    Some(name.unwrap_or(other))
}

/// Разбирает, чем кончился процесс, и сообщает наверх.
fn finish(app: &tauri::AppHandle, id: &str, exit: run::Exit) {
    let runtime = app.state::<Runtime>();
    let Some(cell) = runtime.get(id) else { return };
    let mut running = cell.lock().unwrap();
    let detached = matches!(exit, run::Exit::Detached);

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

    // Вкладку закрывает тот, кто знает про конец процесса. Порт умер
    // вместе с ним, и оставленная вкладка показала бы страницу ошибки
    // WebView2 вместо интерфейса. Исключение — `Detached`: там сервер
    // на порту жив, просто управляет им уже не наш хэндл, и отбирать
    // у пользователя работающий интерфейс не за что.
    if !detached {
        webview::close(app, id);
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
    stop_inner(&app, &runtime, &id)
}

fn stop_inner(app: &tauri::AppHandle, runtime: &Runtime, id: &str) -> Result<(), AppError> {
    let cell = runtime
        .get(id)
        .ok_or_else(|| AppError::new("run.notRunning"))?;
    let _ = RunChanged(RunStatus {
        state: RunState::Stopping,
        ..cell.lock().unwrap().status.clone()
    })
    .emit(app);
    run::stop(&cell)
}

/// Забирает под своё управление сервер, перезапустившийся сам.
///
/// ComfyUI-Manager после установки нод гасит сервер и поднимает новый
/// процесс. Наш хэндл теряется, состояние становится `Detached`, и дальше
/// приложение умеет только смотреть: PID нового процесса ему неизвестен.
///
/// Находим владельца порта по таблице соединений и записываем его PID.
/// После этого работает всё обычное — и остановка, и вкладка.
#[tauri::command]
#[specta::specta]
async fn adopt_instance(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
) -> Result<RunStatus, AppError> {
    let cell = runtime
        .get(&id)
        .ok_or_else(|| AppError::new("run.notRunning"))?;

    let port = {
        let running = cell.lock().unwrap();
        if running.status.state != RunState::Detached {
            return Err(AppError::new("run.notDetached"));
        }
        running.status.port.ok_or_else(|| AppError::new("run.notRunning"))?
    };

    // Порт мог освободиться, пока пользователь читал сообщение.
    if !crate::process::probe(port) {
        return Err(AppError::new("run.notRunning"));
    }

    let pid = supervise::windows::pid_listening_on(port)
        .ok_or_else(|| AppError::with("run.ownerUnknown", "port", port))?;

    let status = {
        let mut running = cell.lock().unwrap();
        running.status.state = RunState::Running;
        running.status.pid = Some(pid);
        running.status.exit_code = None;
        running.status.clone()
    };
    let _ = RunChanged(status.clone()).emit(&app);
    Ok(status)
}

/// Останавливает и поднимает инстанс заново тем же профилем.
///
/// Делается в Rust, а не двумя вызовами с фронта: `stop` возвращается,
/// как только освободился порт, а состояние в `Stopped` переводит поток
/// ожидания процесса — и старт, посланный сразу следом, наткнулся бы
/// на `run.alreadyRunning`. Здесь мы просто дожидаемся конца перехода.
#[tauri::command]
#[specta::specta]
async fn restart_instance(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
) -> Result<RunStatus, AppError> {
    let profile_id = runtime
        .get(&id)
        .and_then(|cell| cell.lock().unwrap().status.profile_id.clone());

    stop_inner(&app, &runtime, &id)?;

    // Ждём, пока поток ожидания процесса разберётся, чем тот кончился.
    // Порт к этому моменту уже отпущен: за этим следит `run::stop`.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while runtime.is_busy(&id) && std::time::Instant::now() < deadline {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // Перезапуск — продолжение того, что уже шло: спрашивать заново про
    // общие модели и про соседнюю сборку не за что, эти согласия
    // пользователь уже дал, когда запускал.
    start_inner(
        &app,
        &runtime,
        id,
        profile_id,
        StartOptions { without_shared: false, allow_multiple: true },
    )
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

// -------------------------------------------------- встроенная вкладка

/// Показывает вкладку инстанса, создавая её при первом вызове.
///
/// Команды встраивания обязаны быть `async`. Синхронная команда Tauri
/// выполняется в главном потоке, а `add_child` изнутри ставит задачу
/// в тот же поток и ждёт её результата — получается взаимная блокировка
/// без единой ошибки в логе.
#[tauri::command]
#[specta::specta]
async fn show_comfy(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    id: String,
    rect: webview::Rect,
) -> Result<(), AppError> {
    // Порт берём из состояния запуска, а не с фронта: он выдан нами
    // и меняется при каждом старте.
    let port = runtime
        .get(&id)
        .and_then(|cell| cell.lock().unwrap().status.port)
        .ok_or_else(|| AppError::new("run.notRunning"))?;
    webview::show(&app, &id, port, rect)
}

/// Переставляет вкладку вслед за областью контента.
#[tauri::command]
#[specta::specta]
async fn place_comfy(
    app: tauri::AppHandle,
    id: String,
    rect: webview::Rect,
) -> Result<(), AppError> {
    webview::place(&app, &id, rect)
}

/// Прячет все вкладки: уход в другой раздел, открытие консоли логов.
#[tauri::command]
#[specta::specta]
async fn hide_comfy(app: tauri::AppHandle) -> Result<(), AppError> {
    webview::hide_all(&app);
    Ok(())
}

#[tauri::command]
#[specta::specta]
async fn reload_comfy(app: tauri::AppHandle, id: String) -> Result<(), AppError> {
    webview::reload(&app, &id)
}

// -------------------------------------------------- отчёт о дубликатах

/// Строит отчёт о дублирующихся моделях по всем сборкам сразу.
///
/// Команда **ничего не делает с файлами** и делать не будет: уборка
/// дублей живёт отдельной командой, на своём экране и со своим перечнем.
#[tauri::command]
#[specta::specta]
async fn scan_duplicates(
    app: tauri::AppHandle,
    cancel: tauri::State<'_, duplicates::ScanCancel>,
) -> Result<duplicates::DuplicatesReport, AppError> {
    cancel.reset();

    let mut places = Vec::new();
    for instance in instances::list(&app)? {
        if !instance.available {
            // Недоступную папку молча пропустить нельзя: отчёт выглядел бы
            // полным. Кладём её как место с несуществующим путём — сканер
            // сам отнесёт её в пропущенные.
            places.push(duplicates::Place {
                name: instance.name.clone(),
                models_dir: std::path::PathBuf::from(&instance.path),
            });
            continue;
        }
        let profiles = run::profiles_of(&instance);
        let Some(profile) = profiles.first() else { continue };
        places.push(duplicates::Place {
            name: instance.name.clone(),
            models_dir: profiles::models_dir(profile, std::path::Path::new(&instance.path)),
        });
    }

    // Общая папка — такое же место: модель, лежащая и там, и в сборке,
    // это дубль ровно в том же смысле.
    let shared = settings::load_shared(&app)?;
    if let Some(root) = migrate::first_root(&shared) {
        places.push(duplicates::Place {
            name: root.display().to_string(),
            models_dir: root,
        });
    }

    let emitter = app.clone();
    Ok(duplicates::scan(&places, &cancel, move |progress| {
        let _ = progress.emit(&emitter);
    }))
}

#[tauri::command]
#[specta::specta]
async fn cancel_duplicates_scan(
    cancel: tauri::State<'_, duplicates::ScanCancel>,
) -> Result<(), AppError> {
    cancel.cancel();
    Ok(())
}

// ------------------------------------------------------ трей и выход

/// Подписи меню трея под текущий язык.
///
/// Меню трея нативное, до `t()` ему не дотянуться, а переводить строки
/// в Rust правилами проекта запрещено. Поэтому текст приходит с фронта —
/// и приходит заново при каждой смене языка.
#[tauri::command]
#[specta::specta]
async fn set_tray_labels(app: tauri::AppHandle, labels: tray::TrayLabels) -> Result<(), AppError> {
    tray::set_labels(&app, &labels);
    Ok(())
}

/// Инстансы, которые закрытие приложения унесёт с собой.
#[tauri::command]
#[specta::specta]
async fn busy_instances(runtime: tauri::State<'_, Runtime>) -> Result<Vec<String>, AppError> {
    Ok(tray::busy(&runtime))
}

/// Остановить всё и выйти.
#[tauri::command]
#[specta::specta]
async fn stop_all_and_quit(app: tauri::AppHandle) -> Result<(), AppError> {
    tray::stop_all(&app);
    app.exit(0);
    Ok(())
}

/// Свернуть в трей, оставив серверы работать.
#[tauri::command]
#[specta::specta]
async fn hide_to_tray(app: tauri::AppHandle) -> Result<(), AppError> {
    webview::hide_all(&app);
    if let Some(window) = app.get_window("main") {
        window
            .hide()
            .map_err(|e| AppError::because("webview.embedFailed", e))?;
    }
    Ok(())
}

// ------------------------------------------------------- обновление

/// Спрашивает, вышла ли новая версия. `None` — установлена последняя.
///
/// Ошибку возвращает как есть, а глушит её вызывающий: автоматическая
/// проверка при старте молчит о сетевых сбоях, ручная — показывает.
/// Различить это в Rust нечем, зато на фронте видно, кто нажал кнопку.
#[tauri::command]
#[specta::specta]
async fn check_update(app: tauri::AppHandle) -> Result<Option<update::UpdateInfo>, AppError> {
    update::check(&app).await
}

/// Ставит обновление и перезапускает приложение.
///
/// Развилка «работают сборки» устроена так же, как гард мульти-запуска:
/// первый вызов приходит с `stop_running: false` и получает отказ с кодом,
/// фронт раскрывает выбор на месте, повторный вызов приходит уже с ответом.
/// Молча гасить чужую очередь генерации нельзя — инсталлятор Windows
/// закроет нас принудительно, а Job Object унесёт с собой все сборки.
#[tauri::command]
#[specta::specta]
async fn install_update(
    app: tauri::AppHandle,
    runtime: tauri::State<'_, Runtime>,
    stop_running: bool,
) -> Result<(), AppError> {
    let busy = tray::busy(&runtime);
    if !busy.is_empty() {
        if !stop_running {
            return Err(AppError::with("update.instancesRunning", "names", busy_names(&app, &busy)));
        }
        tray::stop_all(&app);
    }

    update::install(&app).await
}

/// Имена работающих сборок через запятую — для сообщения пользователю.
///
/// Имена, а не идентификаторы: «i17550…» не скажет ему ничего. Пропавшая
/// из реестра сборка довольствуется идентификатором, лишь бы не промолчать.
fn busy_names(app: &tauri::AppHandle, ids: &[String]) -> String {
    let all = instances::list(app).unwrap_or_default();
    ids.iter()
        .map(|id| {
            all.iter()
                .find(|i| &i.id == id)
                .map(|i| i.name.clone())
                .unwrap_or_else(|| id.clone())
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Папка результатов генерации у инстанса.
///
/// `None` означает «папки ещё нет»: до первой генерации ComfyUI её
/// не создаёт, и открывать нечего. Создавать её за пользователя мы
/// не будем — внутри чужой установки не появляется ничего по нашей воле.
#[tauri::command]
#[specta::specta]
async fn instance_output_dir(
    app: tauri::AppHandle,
    id: String,
    profile_id: Option<String>,
) -> Result<Option<String>, AppError> {
    let instance = instances::list(&app)?
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| AppError::with("instances.notFound", "id", &id))?;

    let all = run::profiles_of(&instance);
    let profile = profile_id
        .as_deref()
        .and_then(|want| all.iter().find(|p| p.id == want))
        .or_else(|| all.first())
        .ok_or_else(|| AppError::new("run.noProfiles"))?;

    let dir = profiles::output_dir(profile, std::path::Path::new(&instance.path));
    Ok(dir.is_dir().then(|| dir.display().to_string()))
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
            preview_command,
            save_custom_profile,
            remove_custom_profile,
            scan_instance_models,
            migrate_models,
            cancel_migrate,
            remove_duplicate_models,
            suggest_library_path,
            load_library_settings,
            save_library_settings,
            scan_library,
            add_workflow_file,
            add_workflow_text,
            set_workflow_meta,
            forget_workflow,
            instance_workflows,
            instance_workflows_dir,
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
            restart_instance,
            adopt_instance,
            show_comfy,
            place_comfy,
            hide_comfy,
            reload_comfy,
            instance_output_dir,
            scan_duplicates,
            cancel_duplicates_scan,
            set_tray_labels,
            busy_instances,
            stop_all_and_quit,
            hide_to_tray,
            check_update,
            install_update
        ])
        .events(tauri_specta::collect_events![
            InstallProgress,
            migrate::MigrateProgress,
            RunLog,
            RunChanged,
            duplicates::DupProgress,
            tray::QuitRequested,
            update::UpdateProgress
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
        // Первым по требованию плагина: второй экземпляр обязан узнать
        // о первом раньше, чем успеет что-нибудь создать. Второе окно
        // означало бы второй Job Object и вторую запись в тот же реестр.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            tray::reveal(app);
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        // Апдейтер спрашивает манифест только по нашей команде: прав
        // на прямой вызов плагина у вебвью нет, и гард «работают сборки»
        // обойти со стороны фронта нечем.
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(SizeJobs::default())
        .manage(InstallLock::default())
        .manage(InstallCancel::default())
        .manage(migrate::MigrateCancel::default())
        .manage(duplicates::ScanCancel::default())
        .manage(Runtime::default())
        .manage(tray::TrayItems::default())
        .invoke_handler(builder.invoke_handler())
        .on_window_event(tray::on_window_event)
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

            if let Err(e) = tray::install(app.handle()) {
                // Без трея приложение работает, просто «свернуть» станет
                // некуда. Ронять запуск из-за этого незачем.
                eprintln!("[CPO] трей не создан: {e}");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("не удалось запустить приложение");
}
