//! Обращения к HTTP-API работающего инстанса ComfyUI.
//!
//! **Ходить сюда обязан Rust, а не наш фронт.** `fetch` из нашего вебвью
//! на `127.0.0.1:<port>` — cross-site, и `origin_only_middleware`
//! (`server.py:159-197`) такие запросы режет. Это тот же барьер, из-за
//! которого в Фазе 0 отвергли `<iframe>`; обойти его можно только
//! `--enable-cors-header`, который выключает защиту целиком — отвергнуто
//! там же.
//!
//! Клиент — `ureq` без TLS: ходим исключительно на петлю, шифровать нечего,
//! а синхронный вызов ложится на правило «логика в обычных функциях,
//! команды — тонкие async-обёртки».
//!
//! `process::wait_ready` намеренно оставлен на голом `TcpStream`: один
//! крошечный опрос раз в полсекунды, переписывать работающее незачем.

use std::collections::BTreeSet;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// Ответ ComfyUI на запрос листинга при `full_info=true`.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoteFile {
    /// Путь относительно запрошенной папки, прямыми слэшами —
    /// так его отдаёт `get_file_info` (`app/user_manager.py:29`).
    pub path: String,
    pub size: f64,
    /// Миллисекунды эпохи.
    pub modified: f64,
}

/// Чем кончилась заливка.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum UploadOutcome {
    Written,
    /// Файл с таким именем уже есть, а перезаписывать не разрешали.
    /// Не ошибка, а развилка: спрашиваем пользователя.
    Conflict,
}

pub struct Client {
    base: String,
    agent: ureq::Agent,
}

impl Client {
    pub fn new(port: u16) -> Self {
        // Таймаут скромный, но не нулевой: сервер на петле отвечает
        // мгновенно, а вот повисший в старте — не отвечает вовсе,
        // и ждать его вечно нельзя.
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .build()
            .into();
        Self { base: format!("http://127.0.0.1:{port}"), agent }
    }

    /// Список воркфлоу сборки.
    ///
    /// Рекурсивный вариант v1, а не `/v2/userdata`: пользователи раскладывают
    /// воркфлоу по подпапкам, и один рекурсивный вызов честнее обхода
    /// по уровням. `full_info` даёт размер и время правки.
    ///
    /// **404 означает «папки ещё нет», а не ошибку.** ComfyUI создаёт
    /// `user/default/workflows` лениво, при первом сохранении, и в свежей
    /// сборке её попросту не существует.
    pub fn list_workflows(&self) -> Result<Vec<RemoteFile>, AppError> {
        let url = format!("{}/userdata?dir=workflows&recurse=true&full_info=true", self.base);
        let body = match self.get(&url) {
            Ok(body) => body,
            Err(AppError { code, .. }) if code == "comfy.notFound" => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };

        let files: Vec<RemoteFile> = serde_json::from_str(&body)
            .map_err(|e| AppError::because("comfy.badResponse", e))?;

        // Посторонние файлы в папке воркфлоу встречаются: ComfyUI кладёт
        // туда и служебное. В список воркфлоу они не идут.
        Ok(files.into_iter().filter(|f| f.path.ends_with(".json")).collect())
    }

    pub fn read_workflow(&self, rel: &str) -> Result<String, AppError> {
        self.get(&format!("{}/userdata/{}", self.base, encode(&format!("workflows/{rel}"))))
    }

    /// Заливает воркфлоу в сборку.
    ///
    /// `overwrite = false` даёт **409** при совпадении имени
    /// (`app/user_manager.py:397`). Это и есть механизм, которым мы
    /// не даём молча затереть чужой воркфлоу.
    pub fn upload_workflow(
        &self,
        rel: &str,
        content: &str,
        overwrite: bool,
    ) -> Result<UploadOutcome, AppError> {
        let url = format!(
            "{}/userdata/{}?overwrite={overwrite}",
            self.base,
            encode(&format!("workflows/{rel}"))
        );

        match self.agent.post(&url).send(content) {
            Ok(_) => Ok(UploadOutcome::Written),
            Err(ureq::Error::StatusCode(409)) => Ok(UploadOutcome::Conflict),
            Err(e) => Err(AppError::because("comfy.uploadFailed", e)),
        }
    }

    /// Множество классов нод, доступных этой сборке.
    ///
    /// Ответ — многомегабайтный JSON со схемами всех нод, а нужны из него
    /// только ключи верхнего уровня. Разбираем целиком (иного способа нет),
    /// но наружу отдаём один набор имён: именно он кэшируется и сравнивается.
    pub fn object_info_keys(&self) -> Result<BTreeSet<String>, AppError> {
        let body = self.get_large(&format!("{}/object_info", self.base))?;
        let value: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| AppError::because("comfy.badResponse", e))?;

        let map = value
            .as_object()
            .ok_or_else(|| AppError::new("comfy.badResponse"))?;

        Ok(map.keys().cloned().collect())
    }

    fn get(&self, url: &str) -> Result<String, AppError> {
        self.read(url, 8 * 1024 * 1024)
    }

    /// Отдельный предел для `/object_info`: у сборки с полусотней пакетов
    /// нод ответ переваливает за десятки мегабайт, и умолчание ureq
    /// обрезало бы его молча.
    fn get_large(&self, url: &str) -> Result<String, AppError> {
        self.read(url, 256 * 1024 * 1024)
    }

    fn read(&self, url: &str, limit: u64) -> Result<String, AppError> {
        match self.agent.get(url).call() {
            Ok(mut res) => res
                .body_mut()
                .with_config()
                .limit(limit)
                .read_to_string()
                .map_err(|e| AppError::because("comfy.badResponse", e)),
            Err(ureq::Error::StatusCode(404)) => Err(AppError::new("comfy.notFound")),
            Err(e) => Err(AppError::because("comfy.unreachable", e)),
        }
    }
}

/// Снимок доступных нод инстанса.
///
/// У остановленной сборки спросить не у кого, а ответ «неизвестно» на каждый
/// вопрос о совместимости бесполезен. Поэтому при каждом успешном старте
/// кладём набор классов рядом, и для остановленной отвечаем по нему,
/// честно помечая ответ как данные последнего запуска.
///
/// Кэш производный: потеря безболезненна, восстановится при первом же
/// старте. Отсюда `app_local_data_dir`, а не папка данных — при чистом
/// удалении приложения его не жалко.
pub mod cache {
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Snapshot {
        pub taken_at: f64,
        pub nodes: BTreeSet<String>,
    }

    fn path(dir: &Path, instance_id: &str) -> PathBuf {
        // Имя инстанса в путь не идёт: оно произвольное и меняется.
        // Идентификатор наш, из реестра, и безопасен как имя файла.
        dir.join("nodes").join(format!("{instance_id}.json"))
    }

    pub fn write(dir: &Path, instance_id: &str, nodes: &BTreeSet<String>) {
        let snapshot = Snapshot {
            taken_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as f64)
                .unwrap_or(0.0),
            nodes: nodes.clone(),
        };
        let file = path(dir, instance_id);
        // Молча: не сумели записать кэш — потеряли удобство, а не данные.
        if let Some(parent) = file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(text) = serde_json::to_string(&snapshot) {
            let _ = std::fs::write(file, text);
        }
    }

    pub fn read(dir: &Path, instance_id: &str) -> Option<Snapshot> {
        let text = std::fs::read_to_string(path(dir, instance_id)).ok()?;
        serde_json::from_str(&text).ok()
    }
}

/// Процентное кодирование сегмента пути.
///
/// Слэш обязан превратиться в `%2F`: путь идёт одним сегментом URL,
/// а ComfyUI разворачивает его обратно (`app/user_manager.py:88`).
/// Своя реализация вместо зависимости — набор символов крошечный,
/// а тянуть ради него ещё один крейт незачем.
fn encode(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for byte in path.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char)
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}
