# Уведомления

<!-- US-UI-03 · US-UI-05 -->

Успех исчезает сам. Ошибка ждёт, пока её прочитают.

Разное поведение не косметика, а требование: успешную операцию
подтверждают и забывают, а сообщение об ошибке нужно дочитать, развернуть
и скопировать вместе с кодом `US-UI-03/AC-3…AC-5`. Код ошибки
показывается всегда — по нему пользователь ищет ответ в документации,
даже если перевода сообщения нет.

<ThemePair>
  <div class="toasts">
    <div class="toast ok">
      <i></i>
      <div class="toast-in">
        <div class="toast-head"><b>Инстанс добавлен</b><span class="close">✕</span></div>
        <p>SDXL стабильная — профили запуска распознаны</p>
        <div class="toast-life"><i></i></div>
      </div>
    </div>
    <div class="toast err">
      <i></i>
      <div class="toast-in">
        <div class="toast-head"><b>Не удалось записать настройки</b><span class="close">✕</span></div>
        <p>Папка инстанса доступна только для чтения. Общие модели можно подключить вторым способом — без записи в папку.</p>
        <div class="disclosure"><span>Детали</span><code>ERR_INSTANCE_READONLY</code><span style="margin-left:auto">Копировать</span></div>
      </div>
    </div>
  </div>
</ThemePair>

### Прогресс — только там, где операция действительно долгая

Постоянного индикатора на каждое действие нет: почти всё в приложении
мгновенно, и мелькающая шкала только шумит `US-UI-05/AC-6`.
Детерминантный показывает доли, когда объём работы известен;
индетерминантный — только когда он неизвестен, и без ложной шкалы.

<ThemePair>
  <div class="stack" style="gap:var(--space-5)">
    <div class="prog">
      <div class="prog-head"><span>Распаковка · D:\AI\Flux</span><span class="count">1 / 2</span></div>
      <div class="track"><i style="width:64%"></i></div>
      <div class="prog-file">python_embeded\Lib\site-packages\torch\_inductor\kernel\mm.py</div>
    </div>
    <div class="prog">
      <div class="prog-head"><span>Обход общей папки моделей</span><span class="count">412 ГБ</span></div>
      <div class="track indet"><i></i></div>
      <div class="prog-file">длительность заранее неизвестна</div>
    </div>
  </div>
</ThemePair>

## Проверка на длинных строках · тексты уведомлений

<div class="longform">
  <div class="lf-head">Проверка на длинных строках · тексты уведомлений</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Could not write settings — the instance folder is read-only</span></div>
    <div class="lf-row"><b>RU</b><span>Не удалось записать настройки — папка инстанса доступна только для чтения</span></div>
    <div class="lf-row"><b>ES</b><span>No se pudo escribir la configuración: la carpeta de la instancia es de solo lectura</span></div>
    <div class="lf-row"><b>ZH</b><span>无法写入设置——实例文件夹为只读</span></div>
  </div>
</div>
