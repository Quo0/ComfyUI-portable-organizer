//! Реализация супервизора под Windows.
//!
//! Ключевая часть — Job Object с `KILL_ON_JOB_CLOSE`. Приложение кладёт
//! **само себя** в job один раз при старте, а членство в job наследуется
//! потомками. Поэтому специального обращения с каждым дочерним процессом
//! не нужно: он попадает в тот же job автоматически, а закрытие последнего
//! хэндла — то есть смерть нашего процесса, штатная или аварийная —
//! забирает с собой всех.
//!
//! План предполагал `CREATE_SUSPENDED` → `AssignProcessToJobObject` →
//! `ResumeThread` для каждого потомка. От этого пути пришлось отказаться
//! по конкретной причине: `std::process::Command` не отдаёт хэндл главного
//! потока, а без него возобновлять нечего. Городить ради этого собственный
//! `CreateProcessW` со всей обвязкой пайпов значит переписать половину
//! `std::process` — при том что наследование job даёт ровно ту же гарантию.

use std::collections::HashMap;
use std::process::{Child, Command, Stdio};

use crate::error::AppError;

use super::{ProcessSupervisor, SpawnRequest};

/// Скрывает окно консоли у дочернего процесса. Без него при каждом запуске
/// поверх интерфейса всплывал бы чёрный терминал.
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub struct WindowsSupervisor;

impl ProcessSupervisor for WindowsSupervisor {
    fn spawn(&self, request: &SpawnRequest) -> Result<Child, AppError> {
        let mut cmd = Command::new(&request.program);
        cmd.args(&request.args)
            .current_dir(&request.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // stdin закрыт намеренно: `pause` в конце .bat и любой input()
            // внутри иначе повиснут в ожидании клавиши, которую нажать негде.
            .stdin(Stdio::null());

        for (key, value) in &request.env {
            cmd.env(key, value);
        }
        apply_python_env(&mut cmd, &request.env);

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        cmd.spawn()
            .map_err(|e| AppError::because("run.spawnFailed", e))
    }

    fn kill_tree(&self, pid: u32) -> Result<(), AppError> {
        // На Windows нельзя послать чужому процессу SIGINT, а `Child::kill`
        // убивает только голову дерева. taskkill /T обходит поддерево целиком.
        let mut cmd = Command::new("taskkill");
        cmd.args(["/PID", &pid.to_string(), "/T", "/F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        match cmd.status() {
            // 128 — процесса уже нет. Это не ошибка: цель достигнута.
            Ok(status) if status.success() || status.code() == Some(128) => Ok(()),
            Ok(status) => Err(AppError::with(
                "run.stopFailed",
                "reason",
                format!("taskkill вернул {}", status.code().unwrap_or(-1)),
            )),
            Err(e) => Err(AppError::because("run.stopFailed", e)),
        }
    }
}

/// Переменные, без которых лог приходит пачкой в конце.
///
/// Проверено спайком Фазы 0: без `PYTHONUNBUFFERED` stdout при
/// перенаправлении в пайп буферизуется блоками, и первые минуты старта
/// выглядят как зависание. Пользовательские `set` из `.bat` не затираем —
/// если человек задал своё, у него была причина.
fn apply_python_env(cmd: &mut Command, existing: &HashMap<String, String>) {
    if !existing.contains_key("PYTHONUNBUFFERED") {
        cmd.env("PYTHONUNBUFFERED", "1");
    }
    if !existing.contains_key("PYTHONIOENCODING") {
        cmd.env("PYTHONIOENCODING", "utf-8");
    }
}

/// Кладёт текущий процесс в job, из которого потомки не выберутся.
///
/// Вызывается один раз при старте приложения. Хэндл job намеренно
/// «утекает»: пока он открыт, job жив, а закрывается он вместе с процессом —
/// именно в этот момент система и убивает всех потомков.
#[cfg(windows)]
pub fn install_job_object() -> Result<(), String> {
    use std::mem::{size_of, zeroed};
    use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject,
        JobObjectExtendedLimitInformation, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    };
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    // SAFETY: все вызовы — обычный Win32 без хитростей с временем жизни;
    // структура инициализируется нулями, как того требует документация.
    unsafe {
        let job = CreateJobObjectW(std::ptr::null(), std::ptr::null());
        if job.is_null() {
            return Err("CreateJobObjectW вернул null".into());
        }

        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = zeroed();
        info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

        let ok = SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            std::ptr::addr_of!(info).cast(),
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        );
        if ok == 0 {
            return Err("SetInformationJobObject не сработал".into());
        }

        if AssignProcessToJobObject(job, GetCurrentProcess()) == 0 {
            return Err("AssignProcessToJobObject не сработал".into());
        }
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn install_job_object() -> Result<(), String> {
    Ok(())
}
