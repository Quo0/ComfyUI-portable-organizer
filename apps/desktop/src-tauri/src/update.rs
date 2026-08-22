//! Обновление приложения: проверка, загрузка, установка.
//!
//! Единственное, что приложение отправляет наружу, — запрос манифеста
//! `latest.json` с текущим номером версии в адресе. Поэтому проверка
//! отключается настройкой, а автоматическая проверка при старте молчит
//! о сетевых сбоях: без сети приложение обязано работать обычным образом,
//! а не встречать пользователя ошибкой, к которой он не имеет отношения.
//!
//! **Установка никогда не идёт молча.** На Windows инсталлятор закрывает
//! приложение принудительно, а дочерние процессы живут в Job Object
//! с `KILL_ON_JOB_CLOSE` и уходят вместе с нами. Обновление посреди
//! генерации стоило бы пользователю очереди и минут холодного старта,
//! поэтому решение о судьбе работающих сборок принимает он, а не мы.
//!
//! Подпись обновления проверяет сам плагин по `pubkey` из конфигурации:
//! не сошлась — установка не начинается. Это не подпись кода и на
//! SmartScreen не влияет, механизмы разные.

use serde::{Deserialize, Serialize};
use tauri_plugin_updater::UpdaterExt;
use tauri_specta::Event;

use crate::error::AppError;

/// Найденная новая версия.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub version: String,
    /// Установленная сейчас. Рядом с новой, чтобы экран не собирал
    /// пару из двух источников.
    pub current_version: String,
    /// Тело релиза: секция `CHANGELOG.md` этой версии. Не переводится —
    /// это текст выпуска, а не строка интерфейса.
    pub notes: Option<String>,
    /// Миллисекунды эпохи, как и остальные даты в реестре: форматирует их
    /// фронт по правилам локали, а не Rust.
    pub date: Option<f64>,
}

/// Ход загрузки. Событием, а не ответом команды: ответ у команды один,
/// а установка идёт десятки секунд.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgress {
    pub downloaded: f64,
    /// `None` — сервер не прислал длину. Тогда полоса индетерминантная.
    pub total: Option<f64>,
}

/// Спрашивает манифест. `None` — установлена последняя версия.
pub async fn check(app: &tauri::AppHandle) -> Result<Option<UpdateInfo>, AppError> {
    let current = app.package_info().version.to_string();

    let updater = app
        .updater()
        .map_err(|e| AppError::because("update.checkFailed", e))?;

    let found = updater
        .check()
        .await
        .map_err(|e| AppError::because("update.checkFailed", e))?;

    Ok(found.map(|update| UpdateInfo {
        version: update.version.clone(),
        current_version: current,
        notes: update.body.clone(),
        date: update
            .date
            .map(|d| d.unix_timestamp() as f64 * 1000.0),
    }))
}

/// Скачивает и ставит обновление, после чего перезапускает приложение.
///
/// Проверка повторяется здесь, а не берётся из ответа `check`: объект
/// обновления живёт внутри плагина и через границу IPC не проходит,
/// а между экраном и нажатием проходит время.
pub async fn install(app: &tauri::AppHandle) -> Result<(), AppError> {
    let updater = app
        .updater()
        .map_err(|e| AppError::because("update.installFailed", e))?;

    let update = updater
        .check()
        .await
        .map_err(|e| AppError::because("update.checkFailed", e))?
        .ok_or_else(|| AppError::new("update.notAvailable"))?;

    let emitter = app.clone();
    let mut downloaded: u64 = 0;
    update
        .download_and_install(
            move |chunk, total| {
                downloaded += chunk as u64;
                let _ = UpdateProgress {
                    downloaded: downloaded as f64,
                    total: total.map(|t| t as f64),
                }
                .emit(&emitter);
            },
            || {},
        )
        .await
        .map_err(|e| AppError::because("update.installFailed", e))?;

    // Сюда доходим только на тех платформах, где инсталлятор не забрал
    // управление себе. Перезапуск не возвращается.
    app.restart()
}
