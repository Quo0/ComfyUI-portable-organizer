import { defineConfig, type DefaultTheme } from 'vitepress';

// Сайт пользовательской документации. Английский — язык, на котором люди
// ищут ответы про ComfyUI, поэтому он лежит в корне; русский и испанский
// переведены целиком и живут под `/ru/` и `/es/`. Китайский пока
// заглушка: его адрес зафиксирован заранее, чтобы перевод, добавленный
// позже, не ломал ссылки, которыми уже поделились.
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

const navEs: DefaultTheme.NavItem[] = [
  { text: 'Guía', link: '/es/guide/', activeMatch: '/es/guide/' },
  { text: 'Referencia', link: '/es/reference/', activeMatch: '/es/reference/' },
  { text: 'Descargar', link: '/es/download' },
];

const sidebarEs: DefaultTheme.Sidebar = {
  '/es/guide/': [
    {
      text: 'Para empezar',
      items: [
        { text: 'Qué es esto', link: '/es/guide/' },
        { text: 'Instalar la aplicación', link: '/es/guide/install-app' },
        { text: 'El aviso de SmartScreen', link: '/es/guide/smartscreen' },
      ],
    },
    {
      text: 'Instalaciones',
      items: [
        { text: 'Añadir una instalación', link: '/es/guide/add-build' },
        { text: 'Instalar desde un archivo', link: '/es/guide/install-from-archive' },
        { text: 'Perfiles de arranque', link: '/es/guide/profiles' },
        { text: 'Puertos y conflictos', link: '/es/guide/ports' },
      ],
    },
    {
      text: 'Contenido compartido',
      items: [
        { text: 'Modelos compartidos', link: '/es/guide/shared-models' },
        { text: 'Biblioteca de flujos', link: '/es/guide/workflows' },
      ],
    },
    {
      text: 'Mantenimiento',
      items: [
        { text: 'Actualizar', link: '/es/guide/updating' },
        { text: 'Desinstalar', link: '/es/guide/uninstall' },
        { text: 'Limitaciones conocidas', link: '/es/guide/limitations' },
      ],
    },
  ],
  '/es/reference/': [
    {
      text: 'Referencia',
      items: [
        { text: 'Resumen', link: '/es/reference/' },
        { text: 'Opciones que añadimos', link: '/es/reference/flags' },
        { text: 'instances.json', link: '/es/reference/instances-json' },
        { text: 'Perfiles de arranque', link: '/es/reference/launch-profile' },
        { text: 'El YAML generado', link: '/es/reference/extra-model-paths' },
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

    es: {
      label: 'Español',
      lang: 'es',
      description: 'Varias instalaciones portables de ComfyUI en una ventana',
      themeConfig: {
        nav: navEs,
        sidebar: sidebarEs,
        outline: { label: 'En esta página' },
        docFooter: { prev: 'Página anterior', next: 'Página siguiente' },
        lastUpdated: { text: 'Actualizado el' },
        returnToTopLabel: 'Volver arriba',
        sidebarMenuLabel: 'Secciones',
        darkModeSwitchLabel: 'Apariencia',
        lightModeSwitchTitle: 'Cambiar al tema claro',
        darkModeSwitchTitle: 'Cambiar al tema oscuro',
        langMenuLabel: 'Cambiar de idioma',
        skipToContentLabel: 'Ir al contenido',
        editLink: {
          pattern: `${REPO}/edit/master/apps/docs/:path`,
          text: 'Editar esta página en GitHub',
        },
        footer: {
          message: 'Sin relación con el proyecto ComfyUI.',
          copyright: 'Licencia MIT. Tus modelos y tus flujos siguen siendo tuyos.',
        },
        notFound: {
          title: 'PÁGINA NO ENCONTRADA',
          quote:
            'Esta página no existe. Revisa la dirección — o empieza por la guía.',
          linkLabel: 'ir al inicio',
          linkText: 'Volver al inicio',
        },
      },
    },

    // Перевода ещё нет, и страница под этим путём честно об этом говорит
    // вместо того, чтобы отдавать 404. Путь зафиксирован заранее, потому
    // что менять его потом — ломать чужие закладки.
    //
    // Ключ локали — это имя папки: `zh` даёт `/zh/`, а не `/zh-Hans/`.
    // Само письмо объявлено в `lang`, потому что традиционного у нас нет
    // и не планируется.
    zh: { label: '简体中文', lang: 'zh-Hans' },
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
          es: {
            translations: {
              button: { buttonText: 'Buscar', buttonAriaLabel: 'Buscar' },
              modal: {
                displayDetails: 'Mostrar detalles',
                resetButtonTitle: 'Borrar la búsqueda',
                backButtonTitle: 'Cerrar la búsqueda',
                noResultsText: 'Sin resultados para',
                footer: {
                  selectText: 'seleccionar',
                  selectKeyAriaLabel: 'Intro',
                  navigateText: 'navegar',
                  navigateUpKeyAriaLabel: 'Flecha arriba',
                  navigateDownKeyAriaLabel: 'Flecha abajo',
                  closeText: 'cerrar',
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
