//! Запуск и остановка дочерних процессов.
//!
//! Второй из двух трейтов, где живёт платформозависимость. Контракт узкий
//! намеренно: всё, что выше, знает только «запустить», «убить дерево»
//! и гарантию «дети умирают вместе с родителем».
//!
//! Гарантия не косметическая. Python с загруженной моделью держит
//! видеопамять, и осиротевший процесс после падения приложения делает
//! видеокарту непригодной до перезагрузки — при этом окна у него нет,
//! и найти его пользователю нечем.

pub mod windows;

use std::collections::HashMap;
use std::process::Child;

use crate::error::AppError;

pub struct SpawnRequest {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub env: HashMap<String, String>,
}

pub trait ProcessSupervisor: Send + Sync {
    /// Запускает процесс с перехваченными stdout и stderr.
    fn spawn(&self, request: &SpawnRequest) -> Result<Child, AppError>;

    /// Убивает процесс вместе со всем его поддеревом.
    ///
    /// Дерево, а не один процесс: портабл-сборка запускает python, тот —
    /// свои воркеры, и убийство головы оставило бы их жить.
    fn kill_tree(&self, pid: u32) -> Result<(), AppError>;
}
