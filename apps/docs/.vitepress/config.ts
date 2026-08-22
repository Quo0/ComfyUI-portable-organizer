import { defineConfig } from 'vitepress';

// Сайт пользовательской документации. Содержимое английское — это язык,
// на котором люди ищут ответы про ComfyUI, — а раскладка локалей заведена
// сразу: перевод, добавленный позже, не должен ломать ссылки, которыми
// уже поделились.
//
// Пакет намеренно ничего не знает про apps/desktop: сборка сайта обязана
// оставаться зелёной, когда Rust-часть временно не компилируется.
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
    root: { label: 'English', lang: 'en' },
    // Переводов ещё нет, и страницы под этими путями честно об этом
    // говорят вместо того, чтобы отдавать 404. Пути зафиксированы сейчас,
    // потому что менять их потом — ломать чужие закладки.
    //
    // Ключ локали — это имя папки: `zh` даёт `/zh/`, а не `/zh-Hans/`.
    // Само письмо объявлено в `lang`, потому что традиционного у нас нет
    // и не планируется.
    ru: { label: 'Русский', lang: 'ru' },
    zh: { label: '简体中文', lang: 'zh-Hans' },
    es: { label: 'Español', lang: 'es' },
  },

  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/', activeMatch: '/guide/' },
      { text: 'Reference', link: '/reference/', activeMatch: '/reference/' },
      { text: 'Download', link: '/download' },
    ],

    sidebar: {
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
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/Quo0/ComfyUI-portable-organizer' },
    ],

    editLink: {
      pattern:
        'https://github.com/Quo0/ComfyUI-portable-organizer/edit/master/apps/docs/:path',
      text: 'Edit this page on GitHub',
    },

    footer: {
      message: 'Not affiliated with the ComfyUI project.',
      copyright: 'MIT-licensed. Models and workflows stay yours.',
    },

    search: { provider: 'local' },
  },
});
