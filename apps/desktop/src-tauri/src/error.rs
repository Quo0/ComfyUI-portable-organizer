//! Ошибки, пересекающие границу Rust → фронт.
//!
//! Бэкенд не переводит ничего. Он отдаёт код и подстановки, а текст
//! собирает фронт из `errors.<code>`. Иначе переводы размазались бы
//! по двум языкам и двум слоям, а `pnpm i18n:check` видел бы только половину.
//!
//! Неизвестный фронту код показывается как сам код: интерфейс не должен
//! падать или показывать пустоту из-за пропущенного ключа.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AppError {
    /// Ключ перевода без префикса: `settings.saveFailed` → `errors.settings.saveFailed`.
    pub code: String,
    /// Подстановки в сообщение. Только строки: числа всё равно форматирует
    /// фронт по правилам локали, а не Rust.
    pub params: HashMap<String, String>,
}

impl AppError {
    pub fn new(code: &str) -> Self {
        Self { code: code.to_string(), params: HashMap::new() }
    }

    /// Ошибка с одной подстановкой — подавляющее большинство случаев.
    pub fn with(code: &str, key: &str, value: impl ToString) -> Self {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());
        Self { code: code.to_string(), params }
    }

    /// Техническая причина от нижележащего слоя.
    ///
    /// Текст причины не переводится и не должен: это сообщение ОС или
    /// парсера, и подменять его выдумкой значит лишить пользователя
    /// единственной зацепки при разборе.
    pub fn because(code: &str, cause: impl ToString) -> Self {
        Self::with(code, "reason", cause)
    }
}
