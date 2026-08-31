import { defineConfig } from 'vitepress';
import { fileURLToPath } from 'node:url';
import { join } from 'node:path';

// The repository root: the app's styles are read from here directly, with
// no copy in this package.
const repoRoot = fileURLToPath(new URL('../../..', import.meta.url));

export default defineConfig({
  title: 'ComfyUI Portable Organizer — дизайн',
  description: 'Стайлгайд и макеты экранов по сценариям',
  lang: 'ru',
  cleanUrls: true,
  appearance: false, // the page theme is our own (--page-*)

  themeConfig: {
    nav: [
      { text: 'Стайлгайд', link: '/styleguide/' },
      { text: 'Экраны', link: '/screens/' },
    ],
    sidebar: {
      '/styleguide/': [
        {
          text: 'Стайлгайд',
          items: [
            { text: 'Палитра', link: '/styleguide/palette' },
            { text: 'Типографика', link: '/styleguide/typography' },
            { text: 'Рейл навигации', link: '/styleguide/nav-rail' },
            { text: 'Карточка инстанса', link: '/styleguide/instance-card' },
            { text: 'Значки', link: '/styleguide/icons' },
            { text: 'Действия', link: '/styleguide/actions' },
            { text: 'Прокрутка', link: '/styleguide/scroll' },
            { text: 'Поля ввода', link: '/styleguide/inputs' },
            { text: 'Уведомления', link: '/styleguide/notifications' },
            { text: 'Пустой реестр', link: '/styleguide/empty-registry' },
            { text: 'Шаги мастера', link: '/styleguide/wizard-steps' },
            { text: 'Категории моделей', link: '/styleguide/model-categories' },
            { text: 'Совместимость воркфлоу', link: '/styleguide/workflow-compat' },
          ],
        },
      ],
      '/screens/': [
        {
          text: 'Инстансы',
          items: [
            { text: 'Пустой реестр', link: '/screens/empty-registry' },
            { text: 'Список сборок', link: '/screens/instance-list' },
            { text: 'Экран сборки: вкладки', link: '/screens/instance-tabs' },
            { text: 'Инстанс стартует', link: '/screens/instance-starting' },
            { text: 'Инстанс работает', link: '/screens/instance-running' },
            { text: 'Сообщения: тост и баннер', link: '/screens/toast-and-banner' },
            { text: 'Аргументы запуска', link: '/screens/launch-args' },
            { text: 'Подключение общих моделей', link: '/screens/shared-models' },
            { text: 'Забор воркфлоу в библиотеку', link: '/screens/workflow-collect' },
            { text: 'Две версии рядом', link: '/screens/two-versions' },
          ],
        },
        {
          text: 'Добавление',
          items: [
            { text: 'Две дорожки', link: '/screens/two-paths' },
            { text: 'Папка уже есть', link: '/screens/existing-folder' },
            { text: 'Архив', link: '/screens/archive' },
            { text: 'Назначения', link: '/screens/mappings' },
            { text: 'Общие ресурсы', link: '/screens/shared-resources' },
            { text: 'Распаковка', link: '/screens/unpacking' },
            { text: 'Готово', link: '/screens/done' },
          ],
        },
        {
          text: 'Настройки',
          items: [
            { text: 'Настройки: внешний вид', link: '/screens/settings-appearance' },
            { text: 'Настройки: общие модели', link: '/screens/settings-shared-models' },
            { text: 'Общие модели на мониторе 1920×1080', link: '/screens/settings-shared-models-hd' },
            { text: 'Настройки: библиотека воркфлоу', link: '/screens/settings-workflow-library' },
            { text: 'Настройки: отчёт по диску', link: '/screens/settings-disk-report' },
            { text: 'Настройки: архивы установщика', link: '/screens/settings-installer-archives' },
            { text: 'Выход при работающих сборках', link: '/screens/exit-with-running-instances' },
            { text: 'О приложении', link: '/screens/about' },
          ],
        },
        {
          text: 'Общее',
          items: [
            { text: 'Правило прокрутки', link: '/screens/scroll-rule' },
            { text: 'Полнота покрытия', link: '/screens/coverage' },
          ],
        },
      ],
    },
    // The outline column is removed rather than hidden: `aside: false` drops
    // the `.has-aside` class from VPDoc, and with it the theme's hard
    // `.content-container { max-width: 688px }`, which is attached to that
    // class alone. The frames need room for `.win` (min-width 940px).
    aside: false,
    socialLinks: [],
  },

  vite: {
    server: {
      fs: { allow: [repoRoot] },
    },
  },
});
