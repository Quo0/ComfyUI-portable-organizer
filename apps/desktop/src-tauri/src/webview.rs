//! Встроенная вкладка: дочерний вебвью с интерфейсом ComfyUI.
//!
//! Дочерний вебвью — нативное окно **поверх** нашего HTML. Отсюда всё
//! остальное: автолейаута у него нет, прямоугольник считает фронт,
//! перекрыть его нашей разметкой физически невозможно, а две видимые
//! вкладки лягут одна на другую.
//!
//! `<iframe>` здесь получил бы 403: `origin_only_middleware` ComfyUI
//! режет всё с `Sec-Fetch-Site: cross-site`. Дочерний вебвью грузит
//! страницу как навигацию верхнего уровня — middleware пропускает
//! без единого послабления в настройках сервера.

use tauri::{LogicalPosition, LogicalSize, Manager, Url, WebviewUrl};

use crate::error::AppError;

/// Префикс метки. По нему находятся все наши вкладки среди вебвью окна.
const PREFIX: &str = "comfy-";

/// Метка вебвью инстанса.
///
/// Идентификатор инстанса — `i<миллисекунды>[-N]` (`instances::new_id`),
/// то есть латиница, цифры и дефис. Экранировать нечего.
pub fn label(id: &str) -> String {
    format!("{PREFIX}{id}")
}

/// Прямоугольник области контента в логических пикселях.
///
/// Приходит из `getBoundingClientRect()` слота на фронте: CSS-пиксели
/// и логические — одно и то же, поэтому масштабирование экрана
/// пересчитывать не надо.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

fn failed(e: impl ToString) -> AppError {
    AppError::because("webview.embedFailed", e)
}

/// Показывает вкладку инстанса, создавая её при первом вызове.
///
/// Прочие вкладки прячутся **в этом же вызове**. Переключение между двумя
/// работающими сборками идёт через один и тот же компонент Vue — меняется
/// только параметр роута, — и порядок «покажи новую, спрячь старую»,
/// отданный фронту, дал бы кадр с двумя вебвью поверх друг друга.
pub fn show(app: &tauri::AppHandle, id: &str, port: u16, rect: Rect) -> Result<(), AppError> {
    let want = label(id);
    hide_others(app, Some(&want));

    if let Some(existing) = app.get_webview(&want) {
        existing
            .set_position(LogicalPosition::new(rect.x, rect.y))
            .map_err(failed)?;
        existing
            .set_size(LogicalSize::new(rect.w, rect.h))
            .map_err(failed)?;
        // Показ и постановка на место одним действием: иначе вкладка
        // мигнёт на прямоугольнике, где её оставили в прошлый раз.
        existing.show().map_err(failed)?;
        let _ = existing.set_focus();
        return Ok(());
    }

    let window = app
        .get_window("main")
        .ok_or_else(|| failed("нет окна main"))?;

    let url = format!("http://127.0.0.1:{port}")
        .parse()
        .map_err(|_| failed("плохой URL"))?;

    let nav_app = app.clone();
    let new_window_app = app.clone();

    let builder = tauri::webview::WebviewBuilder::new(want.clone(), WebviewUrl::External(url))
        // Иначе Tauri перехватит системный дроп, и перетаскивание картинок
        // и воркфлоу на холст ComfyUI перестанет работать.
        .disable_drag_drop_handler()
        .on_navigation(move |url| {
            if internal(url, port) {
                return true;
            }
            // Окна без адресной строки и кнопки «назад» пользователь
            // не ждёт: ссылка на документацию уходит в его браузер.
            open_external(&nav_app, url.as_str());
            false
        })
        // Отдельный механизм: `target="_blank"` и `window.open`
        // приходят сюда, а не в `on_navigation`. Отказываем всегда,
        // включая свой origin, — отдельного окна под попап у нас нет,
        // а положить его поверх встроенной области невозможно.
        .on_new_window(move |url, _features| {
            open_external(&new_window_app, url.as_str());
            tauri::webview::NewWindowResponse::Deny
        });

    window
        .add_child(
            builder,
            LogicalPosition::new(rect.x, rect.y),
            LogicalSize::new(rect.w, rect.h),
        )
        .map_err(failed)?;

    if let Some(view) = app.get_webview(&want) {
        let _ = view.set_focus();
    }
    Ok(())
}

/// Своя ли это навигация.
///
/// Кроме собственного адреса пропускаем `about:`, `blob:` и `data:`:
/// на них держатся скачивания и «Сохранить изображение» в WebView2,
/// и запрет превратил бы экспорт из ComfyUI в тишину.
///
/// Сравниваются разобранные части, а не начало строки. Префикс обманывается
/// парой способов сразу: `http://127.0.0.1:8188@example.com/` начинается
/// ровно с нашего адреса, а хост у него чужой — там это имя пользователя.
pub fn internal(url: &Url, port: u16) -> bool {
    if matches!(url.scheme(), "about" | "blob" | "data") {
        return true;
    }
    url.scheme() == "http"
        && url.host_str() == Some("127.0.0.1")
        && url.port() == Some(port)
        && url.username().is_empty()
}

fn open_external(app: &tauri::AppHandle, url: &str) {
    use tauri_plugin_opener::OpenerExt;
    if let Err(e) = app.opener().open_url(url, None::<&str>) {
        eprintln!("[CPO] не удалось открыть ссылку во внешнем браузере: {e}");
    }
}

/// Переставляет уже созданную вкладку. Нет вкладки — тихо ничего:
/// `ResizeObserver` срабатывает и до её создания.
pub fn place(app: &tauri::AppHandle, id: &str, rect: Rect) -> Result<(), AppError> {
    let Some(view) = app.get_webview(&label(id)) else {
        return Ok(());
    };
    view.set_position(LogicalPosition::new(rect.x, rect.y))
        .map_err(failed)?;
    view.set_size(LogicalSize::new(rect.w, rect.h))
        .map_err(failed)?;
    Ok(())
}

/// Прячет все вкладки: уход в другой раздел или открытие консоли логов.
///
/// Процессы при этом продолжают работать, а несохранённый граф остаётся
/// в живой странице — останавливает сборку только явная команда.
pub fn hide_all(app: &tauri::AppHandle) {
    hide_others(app, None);
}

fn hide_others(app: &tauri::AppHandle, keep: Option<&str>) {
    for (name, view) in app.webviews() {
        if name.starts_with(PREFIX) && Some(name.as_str()) != keep {
            let _ = view.hide();
        }
    }
}

/// Закрывает вкладку инстанса.
///
/// Зовётся, когда процесс кончился: порт умирает вместе с ним, и живая
/// вкладка показала бы страницу ошибки WebView2 вместо интерфейса.
/// При рестарте порт вдобавок может смениться — вкладку обязательно
/// создавать заново, а не переиспользовать.
pub fn close(app: &tauri::AppHandle, id: &str) {
    if let Some(view) = app.get_webview(&label(id)) {
        let _ = view.close();
    }
}

/// Перезагружает страницу вкладки.
///
/// Дешёвый ответ на «интерфейс не догрузился» и на «воркфлоу добавлен,
/// но в списке его нет»: запущенный ComfyUI сам список не перечитывает.
pub fn reload(app: &tauri::AppHandle, id: &str) -> Result<(), AppError> {
    let view = app
        .get_webview(&label(id))
        .ok_or_else(|| AppError::new("webview.noTab"))?;
    view.eval("location.reload()").map_err(failed)
}
