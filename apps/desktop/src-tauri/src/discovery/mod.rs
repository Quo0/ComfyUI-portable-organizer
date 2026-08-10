//! Распознавание установки ComfyUI на диске.
//!
//! Один из двух трейтов, где живёт платформозависимость. Всё, что выше,
//! работает с `Probe` и не знает ни про `python_embeded`, ни про `.bat`.
//! Когда дойдёт до Linux, появится вторая реализация, а вызывающий код
//! не изменится.

pub mod windows_portable;

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// Найденный вариант запуска. В Фазе 1 это только факт наличия файла:
/// разбор `.bat` в редактируемый профиль — задача Фазы 2.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FoundProfile {
    /// Путь относительно корня инстанса: `run_nvidia_gpu.bat`,
    /// `advanced\run_nvidia_gpu_disable_api_nodes.bat`.
    pub id: String,
    /// Имя файла без расширения. Не переводится: это имя файла.
    pub name: String,
    /// Из папки `advanced\` — такие варианты стоит показывать отдельно.
    pub advanced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Probe {
    /// Канонический путь. Сравнение инстансов на дубликат идёт по нему,
    /// а не по тому, что ввёл пользователь.
    pub path: String,
    pub comfy_version: Option<String>,
    pub python_version: Option<String>,
    pub profiles: Vec<FoundProfile>,
}

pub trait InstanceDiscovery: Send + Sync {
    /// Проверяет папку и собирает всё, что о ней известно.
    ///
    /// Ошибка обязана называть, чего не хватило: «папка не подошла» без
    /// причины заставляет пользователя гадать, а типичная причина —
    /// выбран уровень выше или ниже нужного.
    fn probe(&self, path: &Path) -> Result<Probe, AppError>;
}
