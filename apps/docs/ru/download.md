---
title: Скачать
---

<script setup>
// Загрузчик данных один на весь сайт: сведения о релизе не переводятся,
// а второй запрос к GitHub на этапе сборки не нужен.
import { data } from '../download.data.ts';

const sizeMb = data.size ? (data.size / 1024 / 1024).toFixed(1) : '';
const date = data.publishedAt
  ? new Date(data.publishedAt).toLocaleDateString('ru-RU', {
      year: 'numeric', month: 'long', day: 'numeric',
    })
  : '';
</script>

# Скачать

<div v-if="data.available">

**Версия {{ data.version }}** · выпущена {{ date }} · {{ sizeMb }} МБ

<a class="download-button" :href="data.url">Скачать {{ data.name }}</a>

Windows 10 или 11, 64 бита. Права администратора не нужны — инсталлятор
кладёт приложение в ваш профиль пользователя.

</div>
<div v-else>

Сведения о выпуске не удалось прочитать на этапе сборки сайта. Возьмите
самый свежий инсталлятор на странице релизов:

<a class="download-button" :href="data.allReleases">Открыть страницу релизов</a>

</div>

## Перед запуском

Windows покажет предупреждение **«Система Windows защитила ваш компьютер»**.
Так и должно быть: приложение не подписано сертификатом разработчика.
[Как выглядит это окно и что с ним делать →](/ru/guide/smartscreen)

<div v-if="data.available && data.checksums">

Если хотите убедиться, что файл именно наш, проверьте его:
<a :href="data.checksums">SHA256SUMS.txt</a>. В PowerShell:

```powershell
Get-FileHash '.\ComfyUI Portable Organizer_0.1.0_x64-setup.exe' -Algorithm SHA256
```

Контрольная сумма — единственный способ проверить файл, именно потому что
подписи на нём нет.

</div>

## Другие версии

- <a :href="data.allReleases">Все выпуски</a> — включая предрелизные,
  на которые кнопка выше не указывает никогда.
- [Список изменений](https://github.com/Quo0/ComfyUI-portable-organizer/blob/master/CHANGELOG.md)

После установки приложение само проверяет обновления и предлагает их
в разделе **О программе**. Эта проверка — единственное, что оно вообще
отправляет за пределы вашего компьютера, и её можно выключить. Подробнее —
[Обновление](/ru/guide/updating).

<style>
.download-button {
  display: inline-block;
  margin: 1rem 0;
  padding: 0.6rem 1.4rem;
  border-radius: 20px;
  background: var(--vp-c-brand-3);
  color: var(--vp-c-white);
  font-weight: 600;
  text-decoration: none;
}
.download-button:hover {
  background: var(--vp-c-brand-2);
}
</style>
