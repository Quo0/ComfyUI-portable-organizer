//! Сборка запуска воедино: профиль → порт → процесс → готовность.
//!
//! Здесь же живёт стейт-машина: `Stopped → Starting → Running → Stopping →
//! Stopped`, плюс два боковых исхода — `Crashed`, когда процесс ушёл сам,
//! и `Detached`, когда сервер на порту жив, а управляет им уже не он.

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::error::AppError;
use crate::instances::Instance;
use crate::ports;
use crate::process::{
    self, LogLine, RunState, RunStatus, Running,
};
use crate::profiles::{self, LaunchProfile};
use crate::supervise::{windows::WindowsSupervisor, ProcessSupervisor, SpawnRequest};

/// Профили инстанса: разобранные из `.bat` плюс собранные пользователем.
///
/// Разбор читается каждый раз заново, а не берётся из реестра: пользователь
/// мог поправить `.bat` руками, и показывать ему устаревший разбор — врать.
///
/// Свой профиль хранит только имя и аргументы, всё остальное берёт
/// у базового — здесь и сейчас, по той же причине.
pub fn profiles_of(instance: &Instance) -> Vec<LaunchProfile> {
    let root = Path::new(&instance.path);
    let base: Vec<LaunchProfile> = instance
        .profiles
        .iter()
        .map(|found| profiles::parse_bat(root, &found.id, found.advanced))
        .collect();

    let mut all = base.clone();
    for custom in &instance.custom_profiles {
        // Базовый `.bat` мог исчезнуть вместе с обновлением сборки.
        // Тихо подставлять другой нельзя: запустится не то, что просили.
        let Some(source) = base.iter().find(|p| p.id == custom.base_id) else {
            continue;
        };
        all.push(LaunchProfile {
            id: custom.id.clone(),
            name: custom.name.clone(),
            args: custom.args.clone(),
            ..source.clone()
        });
    }
    all
}

/// Что делать дальше после того, как процесс завершился.
pub enum Exit {
    /// Мы сами просили остановиться.
    Requested,
    /// Ушёл сам, и сервер на порту не отвечает.
    Crashed(Option<i32>),
    /// Ушёл сам, но кто-то держит порт: почти наверняка ComfyUI-Manager
    /// переподнял сервер после установки нод.
    Detached,
}

pub struct StartOutcome {
    pub status: RunStatus,
    pub cell: Arc<Mutex<Running>>,
}

/// Готовит команду и запускает процесс.
///
/// Возвращается сразу после спавна: ждать готовности здесь нельзя, иначе
/// вызывающий не увидит ни строчки лога до самого конца холодного старта.
/// `shared_config` — путь к нашему `extra_model_paths.yaml` для режима
/// «флаг». В режиме «файл в инстансе» его нет: файл уже лежит в папке
/// сборки и подхватывается ComfyUI сам.
pub fn start(
    instance: &Instance,
    profile: &LaunchProfile,
    shared_config: Option<&str>,
    on_line: Arc<dyn Fn(LogLine) + Send + Sync>,
    on_exit: impl FnOnce(Exit) + Send + 'static,
) -> Result<StartOutcome, AppError> {
    let port = ports::pick(instance.preferred_port)
        .ok_or_else(|| AppError::new("run.noFreePort"))?;

    let args = profiles::apply_runtime_args(&profile.args, port, shared_config);
    let request = SpawnRequest {
        program: profile.python_path.clone(),
        args,
        cwd: profile.cwd.clone(),
        env: profile.env.clone(),
    };

    let mut child = WindowsSupervisor.spawn(&request)?;
    let pid = child.id();

    let status = RunStatus {
        instance_id: instance.id.clone(),
        state: RunState::Starting,
        port: Some(port),
        pid: Some(pid),
        started_at: Some(process::now_ms()),
        ready_secs: None,
        exit_code: None,
        profile_id: Some(profile.id.clone()),
    };

    let cell = Arc::new(Mutex::new(Running {
        status: status.clone(),
        log: process::LogBuffer::default(),
        stopping: false,
    }));

    // ComfyUI пишет основную часть старта в stderr, а не в stdout,
    // поэтому читаем оба потока — каждый в своём треде, иначе один будет
    // ждать другого и лог поедет.
    if let Some(stream) = child.stdout.take() {
        spawn_pump("stdout", stream, cell.clone(), on_line.clone());
    }
    if let Some(stream) = child.stderr.take() {
        spawn_pump("stderr", stream, cell.clone(), on_line.clone());
    }

    // Ожидание завершения — в своём потоке: `wait` блокирует, а мы обязаны
    // узнать о падении сразу, а не когда пользователь нажмёт «Остановить».
    let watch = cell.clone();
    std::thread::spawn(move || {
        let code = child.wait().ok().and_then(|s| s.code());
        let requested = watch.lock().unwrap().stopping;
        let exit = if requested {
            Exit::Requested
        } else if process::detached_after_exit(port) {
            Exit::Detached
        } else {
            Exit::Crashed(code)
        };
        on_exit(exit);
    });

    Ok(StartOutcome { status, cell })
}

/// Заводит тред чтения одного из потоков процесса.
fn spawn_pump<R: std::io::Read + Send + 'static>(
    name: &'static str,
    stream: R,
    cell: Arc<Mutex<Running>>,
    sink: Arc<dyn Fn(LogLine) + Send + Sync>,
) {
    std::thread::spawn(move || {
        process::pump(stream, |text, replaces| {
            let line = process::push_line(&cell, name, text, replaces);
            sink(line);
        });
    });
}

/// Останавливает инстанс и дожидается освобождения порта.
///
/// Ждать обязательно: процесс успевает умереть раньше, чем система
/// отпустит его сокет, и немедленный перезапуск натыкается на собственный
/// прошлый порт.
pub fn stop(cell: &Arc<Mutex<Running>>) -> Result<(), AppError> {
    let (pid, port) = {
        let mut running = cell.lock().unwrap();
        running.stopping = true;
        running.status.state = RunState::Stopping;
        (running.status.pid, running.status.port)
    };

    let Some(pid) = pid else {
        return Err(AppError::new("run.notRunning"));
    };
    WindowsSupervisor.kill_tree(pid)?;

    if let Some(port) = port {
        ports::wait_released(port, Duration::from_secs(10));
    }
    Ok(())
}
