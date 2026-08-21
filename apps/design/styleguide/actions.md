# Действия

<!-- US-RUN-01 · US-REG-05 · NFR-300 -->

Главное действие нейтральное: фирменный цвет спорил бы с акцентами инстансов.

Кнопка главного действия залита не фирменным цветом, а нейтральным
чернильным — потому что цвет в этом приложении уже занят: его выбирает
пользователь каждому инстансу. Опасные действия обозначены контуром,
а не заливкой: удаление из реестра не должно выглядеть привлекательнее
запуска.

<ThemePair>
  <div class="stack" style="gap:var(--space-4)">
    <div class="row">
      <span class="btn primary">Запустить</span>
      <span class="btn secondary">Открыть папку</span>
      <span class="btn ghost">Отмена</span>
      <span class="btn danger">Убрать из реестра</span>
    </div>
    <div class="row">
      <span class="btn primary lg">Установить ComfyUI</span>
      <span class="btn secondary"><i class="spin"></i>Останавливается</span>
      <span class="btn secondary" aria-disabled="true">Запустить</span>
    </div>
    <div class="row">
      <span class="split">
        <span class="btn primary">Запустить</span>
        <span class="btn primary">▾</span>
      </span>
      <span class="t-label">сплит-кнопка выбора профиля</span>
    </div>
  </div>
</ThemePair>

Выпадающий список профилей у сплит-кнопки раскрывается
**только пока инстанс не запущен**. После старта область
контента занимает нативное окно ComfyUI, и всплыть поверх него наш список
физически не может — поэтому здесь он показан закрытым.

## Проверка на длинных строках · подписи кнопок

<div class="longform">
  <div class="lf-head">Проверка на длинных строках · подписи кнопок</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Launch · Open folder · Remove from registry</span></div>
    <div class="lf-row"><b>RU</b><span>Запустить · Открыть папку · Убрать из реестра</span></div>
    <div class="lf-row"><b>ES</b><span>Iniciar · Abrir carpeta · Quitar del registro</span></div>
    <div class="lf-row"><b>ZH</b><span>启动 · 打开文件夹 · 从注册表移除</span></div>
  </div>
</div>
