//! Правило «своё остаётся во вкладке, чужое уходит в браузер».
//!
//! Проверяется чистая часть — решение по URL. Само встраивание проверить
//! отсюда нельзя: для него нужно окно, и потому оно живёт в списке ручных
//! проверок. Но ошибиться легче всего именно в правиле: слишком узкое
//! отправит в браузер сам ComfyUI, слишком широкое оставит во вкладке
//! страницу входа, где нет ни адресной строки, ни кнопки «назад».
//!
//! Запуск: cargo run --example check_webview

use cpo_desktop_lib::webview::{internal, label};
use tauri::Url;

fn main() {
    let mut failures = 0;
    const PORT: u16 = 8188;

    let mut case = |what: &str, url: &str, want: bool| {
        let parsed = Url::parse(url).expect("валидный URL");
        let got = internal(&parsed, PORT);
        let ok = got == want;
        println!("{} {what} — {url}", if ok { "  OK  " } else { "ПРОВАЛ" });
        failures += u32::from(!ok);
    };

    case("сам интерфейс остаётся во вкладке", "http://127.0.0.1:8188", true);
    case("страница внутри сборки остаётся", "http://127.0.0.1:8188/templates", true);
    case("запрос к API остаётся", "http://127.0.0.1:8188/api/userdata", true);

    // Скачивание и «Сохранить изображение» в WebView2 идут через эти схемы.
    // Запрет превратил бы экспорт из ComfyUI в тишину.
    case("blob остаётся", "blob:http://127.0.0.1:8188/9f1c", true);
    case("data остаётся", "data:text/plain,hi", true);
    case("about:blank остаётся", "about:blank", true);

    case("документация уходит в браузер", "https://docs.comfy.org/", false);
    case("вход в API-ноды уходит в браузер", "https://platform.comfy.org/login", false);
    // Соседний инстанс — тоже чужой сервер: своя вкладка у него своя.
    case("другой порт на локалхосте — чужой", "http://127.0.0.1:8189/", false);
    // Тот же порт, но другой хост: одинаковое начало строки обмануть
    // не должно.
    case("другой хост с тем же портом — чужой", "http://example.com:8188/", false);
    // Ровно наш адрес в начале строки, а хост чужой: до `@` стоит имя
    // пользователя. Сравнение по префиксу это пропускало.
    case(
        "наш адрес как имя пользователя — чужой",
        "http://127.0.0.1:8188@example.com/",
        false,
    );
    // Схема другая — значит и сервер не тот, что мы поднимали.
    case("https на тот же адрес — чужой", "https://127.0.0.1:8188/", false);

    let id = "i1755000000000-2";
    let ok = label(id) == format!("comfy-{id}");
    println!("{} метка вкладки — {}", if ok { "  OK  " } else { "ПРОВАЛ" }, label(id));
    failures += u32::from(!ok);

    println!("\nПроверок провалено: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
