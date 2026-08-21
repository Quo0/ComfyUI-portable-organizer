# Типографика

<!-- US-UI-02 · NFR-250 · NFR-260 -->

Системные гарнитуры Windows — не экономия, а то, чем приложение будет
отрисовано на самом деле.

Приложение существует только под Windows, поэтому стек начинается
с `Segoe UI Variable`, а моноширинный — с `Cascadia Mono`. Макет
и итоговая сборка совпадут посимвольно. `Microsoft YaHei` в стеке
обязателен: без него китайский падает в случайный фолбэк и выглядит
чужеродно. Базовый кегль 13&nbsp;px — инструмент, где в список должно
помещаться много инстансов, а в консоль много строк.

## Шкала

<ThemePair>
  <div class="scale">
    <div class="scale-row"><span>xl · 22</span><div class="t-xl">Общие модели</div></div>
    <div class="scale-row"><span>lg · 18</span><div class="t-lg">Мастер установки</div></div>
    <div class="scale-row"><span>md · 15</span><div class="t-md">SDXL — стабильная</div></div>
    <div class="scale-row"><span>base · 13</span><div class="t-base">Инстанс подключён к общей папке моделей</div></div>
    <div class="scale-row"><span>sm · 12</span><div class="t-sm">Требуется перезапуск, чтобы изменение вступило в силу</div></div>
    <div class="scale-row"><span>label · 11</span><div class="t-label">Профиль запуска</div></div>
    <div class="scale-row"><span>mono · 12</span><div class="t-mono">D:\AI\SDXL · :8188 · 52.3 GB</div></div>
    <div class="scale-row"><span>CJK</span><div class="t-base">共享模型存储 · 工作流库 · 安装向导</div></div>
  </div>
</ThemePair>

## Консоль запуска · моноширинный в деле

Строка индикатора загрузки моделей **перерисовывается на месте**, а не
добавляется новой — иначе за один старт лог распухает на десятки тысяч
строк (`US-RUN-03/AC-3`).

<ThemePair>
<div class="console">Total VRAM 24564 MB, total RAM 65451 MB
pytorch version: 2.13.0+cu130
Set vram state to: NORMAL_VRAM
<span class="dim">Loading model  ████████████░░░░░░░  61%  4.2/6.9 GB</span>
Starting server on 127.0.0.1:8188</div>
</ThemePair>

## Проверка на длинных строках · подписи разделов

<div class="longform">
  <div class="lf-head">Проверка на длинных строках · подписи разделов</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Workflow library · Installation wizard · About the app</span></div>
    <div class="lf-row"><b>RU</b><span>Библиотека воркфлоу · Мастер установки · О приложении</span></div>
    <div class="lf-row"><b>ES</b><span>Biblioteca de flujos de trabajo · Asistente de instalación · Acerca de la aplicación</span></div>
    <div class="lf-row"><b>ZH</b><span>工作流库 · 安装向导 · 关于应用</span></div>
  </div>
</div>
