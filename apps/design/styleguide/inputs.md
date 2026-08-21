# Поля ввода

<!-- US-REG-01 · US-REG-02 · US-SHARED-04 -->

Путь, имя, порт, цвет — четыре вещи, которые задаются при каждом добавлении инстанса.

Выбор акцентного цвета — единственное поле с проверкой контраста:
пользователь волен взять любой, но предлагать заведомо нечитаемый нельзя
`US-UI-01/AC-8`. Поэтому набор ограничен восемью, каждый из
которых проходит порог на обоих основаниях.

<ThemePair>
  <div class="stack" style="gap:var(--space-4)">
    <div class="field">
      <label>Папка инстанса</label>
      <div class="path-row">
        <div class="input mono"><span>D:\AI\ComfyUI_SDXL</span></div>
        <span class="btn secondary">Обзор</span>
      </div>
      <div class="hint">Регистрация ничего не изменит внутри папки</div>
    </div>
    <div class="field">
      <label>Имя</label>
      <div class="input">SDXL стабильная</div>
    </div>
    <div class="field">
      <label>Имя</label>
      <div class="input bad"></div>
      <div class="hint bad">Имя не может быть пустым</div>
    </div>
    <div class="row" style="gap:var(--space-6); align-items:flex-start">
      <div class="field">
        <label>Порт</label>
        <div class="input num mono">8188</div>
      </div>
      <div class="field" style="flex:1">
        <label>Акцентный цвет</label>
        <div class="picker">
          <i style="background:var(--accent-ember)"></i>
          <i style="background:var(--accent-amber)"></i>
          <i style="background:var(--accent-moss)"></i>
          <i class="on" style="background:var(--accent-teal)"></i>
          <i style="background:var(--accent-azure)"></i>
          <i style="background:var(--accent-indigo)"></i>
          <i style="background:var(--accent-orchid)"></i>
          <i style="background:var(--accent-rose)"></i>
        </div>
      </div>
    </div>
    <div class="toggle-row">
      <span class="toggle"></span>
      <div>
        <div class="t-base">Скачивать новые модели в общую папку</div>
        <div class="hint">Модель, загруженная в одном инстансе, станет видна всем</div>
      </div>
    </div>
    <div class="toggle-row">
      <span class="toggle off"></span>
      <div>
        <div class="t-base">Записывать файл настроек внутрь инстанса</div>
        <div class="hint">Нужно, если сборка иногда запускается мимо приложения</div>
      </div>
    </div>
  </div>
</ThemePair>

## Проверка на длинных строках · подписи полей

<div class="longform">
  <div class="lf-head">Проверка на длинных строках · подписи полей</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Download new models to the shared folder</span></div>
    <div class="lf-row"><b>RU</b><span>Скачивать новые модели в общую папку</span></div>
    <div class="lf-row"><b>ES</b><span>Descargar los modelos nuevos en la carpeta compartida</span></div>
    <div class="lf-row"><b>ZH</b><span>将新模型下载到共享文件夹</span></div>
  </div>
</div>
