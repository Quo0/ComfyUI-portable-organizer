//! Errors that cross the Rust → frontend boundary.
//!
//! The backend translates nothing. It returns a code and interpolations, and
//! the frontend assembles the text from `errors.<code>`. Otherwise the
//! translations would be smeared across two languages and two layers, and
//! `pnpm i18n:check` would see only half of them.
//!
//! A code the frontend does not know is shown as the code itself: the UI must
//! not crash or show blankness because of a missing key.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AppError {
    /// The translation key without its prefix: `settings.saveFailed` → `errors.settings.saveFailed`.
    pub code: String,
    /// Interpolations for the message. Strings only: numbers are formatted by
    /// the frontend according to the locale's rules anyway, not by Rust.
    pub params: HashMap<String, String>,
}

impl AppError {
    pub fn new(code: &str) -> Self {
        Self { code: code.to_string(), params: HashMap::new() }
    }

    /// An error with a single interpolation — the vast majority of cases.
    pub fn with(code: &str, key: &str, value: impl ToString) -> Self {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());
        Self { code: code.to_string(), params }
    }

    /// A technical cause from a lower layer.
    ///
    /// The cause text is not translated and must not be: it is a message from
    /// the OS or a parser, and replacing it with an invention would deprive the
    /// user of their only foothold while investigating.
    pub fn because(code: &str, cause: impl ToString) -> Self {
        Self::with(code, "reason", cause)
    }
}
