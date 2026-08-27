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

Сведения о выпуске не удалось прочитать, пока собирался сайт. Возьмите
самый свежий инсталлятор на странице релизов:

<a class="download-button" :href="data.allReleases">Открыть страницу релизов</a>

</div>

## Перед запуском

Windows покажет предупреждение **«Система Windows защитила ваш компьютер»**.
Так и должно быть: приложение не подписано сертификатом разработчика.
[Как выглядит это окно и что с ним делать →](/ru/guide/smartscreen)

<div v-if="data.available && data.checksums">

Если хотите убедиться, что файл именно наш, сверьте его с
<a :href="data.checksums">SHA256SUMS.txt</a>. В PowerShell:

```powershell
Get-FileHash '.\ComfyUI Portable Organizer_0.1.0_x64-setup.exe' -Algorithm SHA256
```

Раз подписи на файле нет, контрольная сумма — единственный способ его
проверить.

</div>

## Другие версии

- <a :href="data.allReleases">Все выпуски</a> — включая предрелизные,
  на которые кнопка выше не указывает никогда.
- [Список изменений](https://github.com/Quo0/ComfyUI-portable-organizer/blob/master/CHANGELOG.md)

После установки приложение само проверяет обновления и предлагает их
в разделе **О программе**. Эта проверка — единственное, что оно вообще
отправляет за пределы вашего компьютера, и выключить её можно. Подробнее —
[Обновление](/ru/guide/updating).
