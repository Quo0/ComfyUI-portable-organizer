import { defineConfig, type DefaultTheme } from 'vitepress';

// Сайт пользовательской документации. Английский — язык, на котором люди
// ищут ответы про ComfyUI, поэтому он лежит в корне; русский переведён
// целиком и живёт под `/ru/`. Китайский и испанский пока заглушки: их
// адреса зафиксированы заранее, чтобы перевод, добавленный позже,
// не ломал ссылки, которыми уже поделились.
//
// Пакет намеренно ничего не знает про apps/desktop: сборка сайта обязана
// оставаться зелёной, когда Rust-часть временно не компилируется.

const REPO = 'https://github.com/Quo0/ComfyUI-portable-organizer';

// Навигация задаётся по локали целиком, а не переводом заголовков поверх
// общей раскладки: ссылки у переведённой локали ведут внутрь неё
// (`/ru/guide/`), и подмешивать сюда корневые пути нечем.
const navEn: DefaultTheme.NavItem[] = [
  { text: 'Guide', link: '/guide/', activeMatch: '/guide/' },
  { text: 'Reference', link: '/reference/', activeMatch: '/reference/' },
  { text: 'Download', link: '/download' },
];

const sidebarEn: DefaultTheme.Sidebar = {
  '/guide/': [
    {
      text: 'Getting started',
      items: [
        { text: 'What this is', link: '/guide/' },
        { text: 'Install the app', link: '/guide/install-app' },
        { text: 'SmartScreen warning', link: '/guide/smartscreen' },
      ],
    },
    {
      text: 'Builds',
      items: [
        { text: 'Add a build', link: '/guide/add-build' },
        { text: 'Install from an archive', link: '/guide/install-from-archive' },
        { text: 'Launch profiles', link: '/guide/profiles' },
        { text: 'Ports and conflicts', link: '/guide/ports' },
      ],
    },
    {
      text: 'Shared content',
      items: [
        { text: 'Shared models', link: '/guide/shared-models' },
        { text: 'Workflow library', link: '/guide/workflows' },
      ],
    },
    {
      text: 'Housekeeping',
      items: [
        { text: 'Updating', link: '/guide/updating' },
        { text: 'Uninstalling', link: '/guide/uninstall' },
        { text: 'Known limitations', link: '/guide/limitations' },
      ],
    },
  ],
  '/reference/': [
    {
      text: 'Reference',
      items: [
        { text: 'Overview', link: '/reference/' },
        { text: 'Flags we add', link: '/reference/flags' },
        { text: 'instances.json', link: '/reference/instances-json' },
        { text: 'Launch profiles', link: '/reference/launch-profile' },
        { text: 'Generated YAML', link: '/reference/extra-model-paths' },
      ],
    },
  ],
};

const navRu: DefaultTheme.NavItem[] = [
  { text: 'Руководство', link: '/ru/guide/', activeMatch: '/ru/guide/' },
  { text: 'Справочник', link: '/ru/reference/', activeMatch: '/ru/reference/' },
  { text: 'Скачать', link: '/ru/download' },
];

const sidebarRu: DefaultTheme.Sidebar = {
  '/ru/guide/': [
    {
      text: 'С чего начать',
      items: [
        { text: 'Что это такое', link: '/ru/guide/' },
        { text: 'Установка приложения', link: '/ru/guide/install-app' },
        { text: 'Предупреждение SmartScreen', link: '/ru/guide/smartscreen' },
      ],
    },
    {
      text: 'Сборки',
      items: [
        { text: 'Добавить сборку', link: '/ru/guide/add-build' },
        { text: 'Установка из архива', link: '/ru/guide/install-from-archive' },
        { text: 'Профили запуска', link: '/ru/guide/profiles' },
        { text: 'Порты и конфликты', link: '/ru/guide/ports' },
      ],
    },
    {
      text: 'Общее хозяйство',
      items: [
        { text: 'Общие модели', link: '/ru/guide/shared-models' },
        { text: 'Библиотека воркфлоу', link: '/ru/guide/workflows' },
      ],
    },
    {
      text: 'Обслуживание',
      items: [
        { text: 'Обновление', link: '/ru/guide/updating' },
        { text: 'Удаление', link: '/ru/guide/uninstall' },
        { text: 'Известные ограничения', link: '/ru/guide/limitations' },
      ],
    },
  ],
  '/ru/reference/': [
    {
      text: 'Справочник',
      items: [
        { text: 'Обзор', link: '/ru/reference/' },
        { text: 'Какие флаги мы добавляем', link: '/ru/reference/flags' },
        { text: 'instances.json', link: '/ru/reference/instances-json' },
        { text: 'Профили запуска', link: '/ru/reference/launch-profile' },
        { text: 'Генерируемый YAML', link: '/ru/reference/extra-model-paths' },
      ],
    },
  ],
};

export default defineConfig({
  // Репозиторий публикуется на GitHub Pages как проектный сайт, то есть
  // живёт в подпапке. Без base все ссылки на ассеты уедут в корень домена.
  base: '/ComfyUI-portable-organizer/',

  title: 'ComfyUI Portable Organizer',
  description: 'Manage several portable ComfyUI builds from one window',
  cleanUrls: true,
  lastUpdated: true,

  head: [['link', { rel: 'icon', href: '/ComfyUI-portable-organizer/favicon.svg' }]],

  locales: {
    root: {
      label: 'English',
      lang: 'en',
      themeConfig: {
        nav: navEn,
        sidebar: sidebarEn,
        editLink: {
          pattern: `${REPO}/edit/master/apps/docs/:path`,
          text: 'Edit this page on GitHub',
        },
        footer: {
          message: 'Not affiliated with the ComfyUI project.',
          copyright: 'MIT-licensed. Models and workflows stay yours.',
        },
      },
    },

    ru: {
      label: 'Русский',
      lang: 'ru',
      description: 'Несколько портабл-сборок ComfyUI в одном окне',
      themeConfig: {
        nav: navRu,
        sidebar: sidebarRu,
        // Строки самой темы: без них у переведённых страниц остаются
        // английские «On this page» и «Previous page» вперемешку с текстом.
        outline: { label: 'На этой странице' },
        docFooter: { prev: 'Предыдущая страница', next: 'Следующая страница' },
        lastUpdated: { text: 'Обновлено' },
        returnToTopLabel: 'Наверх',
        sidebarMenuLabel: 'Разделы',
        darkModeSwitchLabel: 'Оформление',
        lightModeSwitchTitle: 'Светлая тема',
        darkModeSwitchTitle: 'Тёмная тема',
        langMenuLabel: 'Сменить язык',
        skipToContentLabel: 'Перейти к содержимому',
        editLink: {
          pattern: `${REPO}/edit/master/apps/docs/:path`,
          text: 'Предложить правку на GitHub',
        },
        footer: {
          message: 'Проект не связан с командой ComfyUI.',
          copyright: 'Лицензия MIT. Модели и воркфлоу остаются вашими.',
        },
        notFound: {
          title: 'СТРАНИЦА НЕ НАЙДЕНА',
          quote:
            'Такой страницы нет. Проверьте адрес — или начните с руководства.',
          linkLabel: 'на главную',
          linkText: 'Вернуться на главную',
        },
      },
    },

    // Переводов ещё нет, и страницы под этими путями честно об этом
    // говорят вместо того, чтобы отдавать 404. Пути зафиксированы сейчас,
    // потому что менять их потом — ломать чужие закладки.
    //
    // Ключ локали — это имя папки: `zh` даёт `/zh/`, а не `/zh-Hans/`.
    // Само письмо объявлено в `lang`, потому что традиционного у нас нет
    // и не планируется.
    zh: { label: '简体中文', lang: 'zh-Hans' },
    es: { label: 'Español', lang: 'es' },
  },

  themeConfig: {
    socialLinks: [{ icon: 'github', link: REPO }],

    search: {
      provider: 'local',
      options: {
        locales: {
          ru: {
            translations: {
              button: { buttonText: 'Поиск', buttonAriaLabel: 'Поиск' },
              modal: {
                displayDetails: 'Показать подробности',
                resetButtonTitle: 'Очистить запрос',
                backButtonTitle: 'Закрыть поиск',
                noResultsText: 'Ничего не найдено по запросу',
                footer: {
                  selectText: 'выбрать',
                  selectKeyAriaLabel: 'Enter',
                  navigateText: 'перейти',
                  navigateUpKeyAriaLabel: 'Стрелка вверх',
                  navigateDownKeyAriaLabel: 'Стрелка вниз',
                  closeText: 'закрыть',
                  closeKeyAriaLabel: 'Esc',
                },
              },
            },
          },
        },
      },
    },
  },
});
