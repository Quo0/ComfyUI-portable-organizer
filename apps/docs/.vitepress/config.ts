import { defineConfig, type DefaultTheme } from 'vitepress';

// Сайт пользовательской документации. Английский — язык, на котором люди
// ищут ответы про ComfyUI, поэтому он лежит в корне; русский, испанский
// и китайский переведены целиком и живут под `/ru/`, `/es/` и `/zh/`.
// Заглушек больше нет ни под одним путём.
//
// Пакет намеренно ничего не знает про apps/desktop: сборка сайта обязана
// оставаться зелёной, когда Rust-часть временно не компилируется.

const REPO = 'https://github.com/Quo0/ComfyUI-portable-organizer';

// Сайт живёт в подпапке GitHub Pages, и подвал темы выводится через
// `v-html`. Разметку оттуда VitePress не разбирает и пути в ней не
// переписывает — в отличие от ссылок в markdown. Значит, ссылка на
// лицензию в подвале обязана нести префикс сама, иначе уедет в корень
// домена. Отсюда общая константа: `base` и подвал не должны разъехаться.
const BASE = '/ComfyUI-portable-organizer/';

// Навигация задаётся по локали целиком, а не переводом заголовков поверх
// общей раскладки: ссылки у переведённой локали ведут внутрь неё
// (`/ru/guide/`), и подмешивать сюда корневые пути нечем.
const navEn: DefaultTheme.NavItem[] = [
  { text: 'Guide', link: '/guide/', activeMatch: '/guide/' },
  { text: 'Reference', link: '/reference/', activeMatch: '/reference/' },
  { text: 'Download', link: '/download' },
  { text: 'License', link: '/license' },
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
  { text: 'Лицензия', link: '/ru/license' },
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
  { text: 'Licencia', link: '/es/license' },
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

const navZh: DefaultTheme.NavItem[] = [
  { text: '指南', link: '/zh/guide/', activeMatch: '/zh/guide/' },
  { text: '参考', link: '/zh/reference/', activeMatch: '/zh/reference/' },
  { text: '下载', link: '/zh/download' },
  { text: '许可证', link: '/zh/license' },
];

const sidebarZh: DefaultTheme.Sidebar = {
  '/zh/guide/': [
    {
      text: '从这里开始',
      items: [
        { text: '这是什么', link: '/zh/guide/' },
        { text: '安装应用', link: '/zh/guide/install-app' },
        { text: 'SmartScreen 提示', link: '/zh/guide/smartscreen' },
      ],
    },
    {
      text: '整合包',
      items: [
        { text: '添加整合包', link: '/zh/guide/add-build' },
        { text: '从压缩包安装', link: '/zh/guide/install-from-archive' },
        { text: '启动配置', link: '/zh/guide/profiles' },
        { text: '端口与冲突', link: '/zh/guide/ports' },
      ],
    },
    {
      text: '共享内容',
      items: [
        { text: '共享模型', link: '/zh/guide/shared-models' },
        { text: '工作流库', link: '/zh/guide/workflows' },
      ],
    },
    {
      text: '日常维护',
      items: [
        { text: '更新应用', link: '/zh/guide/updating' },
        { text: '卸载', link: '/zh/guide/uninstall' },
        { text: '已知限制', link: '/zh/guide/limitations' },
      ],
    },
  ],
  '/zh/reference/': [
    {
      text: '参考',
      items: [
        { text: '总览', link: '/zh/reference/' },
        { text: '我们添加的参数', link: '/zh/reference/flags' },
        { text: 'instances.json', link: '/zh/reference/instances-json' },
        { text: '启动配置', link: '/zh/reference/launch-profile' },
        { text: '生成的 YAML', link: '/zh/reference/extra-model-paths' },
      ],
    },
  ],
};

export default defineConfig({
  // Репозиторий публикуется на GitHub Pages как проектный сайт, то есть
  // живёт в подпапке. Без base все ссылки на ассеты уедут в корень домена.
  base: BASE,

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
          copyright: `Free software under the <a href="${BASE}license">GNU GPL v3</a>. Models and workflows stay yours.`,
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
          copyright: `Свободная программа под <a href="${BASE}ru/license">GNU GPL v3</a>. Модели и воркфлоу остаются вашими.`,
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
          copyright: `Software libre bajo la <a href="${BASE}es/license">GNU GPL v3</a>. Tus modelos y tus flujos siguen siendo tuyos.`,
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

    // Ключ локали — это имя папки: `zh` даёт `/zh/`, а не `/zh-Hans/`.
    // Само письмо объявлено в `lang`, потому что традиционного у нас нет
    // и не планируется.
    zh: {
      label: '简体中文',
      lang: 'zh-Hans',
      description: '在一个窗口里管理多个 ComfyUI 整合包',
      themeConfig: {
        nav: navZh,
        sidebar: sidebarZh,
        outline: { label: '本页目录' },
        docFooter: { prev: '上一页', next: '下一页' },
        lastUpdated: { text: '最后更新于' },
        returnToTopLabel: '返回顶部',
        sidebarMenuLabel: '目录',
        darkModeSwitchLabel: '外观',
        lightModeSwitchTitle: '切换到浅色主题',
        darkModeSwitchTitle: '切换到深色主题',
        langMenuLabel: '切换语言',
        skipToContentLabel: '跳到正文',
        editLink: {
          pattern: `${REPO}/edit/master/apps/docs/:path`,
          text: '在 GitHub 上编辑此页',
        },
        footer: {
          message: '本项目与 ComfyUI 项目无关。',
          copyright: `自由软件，采用 <a href="${BASE}zh/license">GNU GPL v3</a>。模型和工作流始终属于你。`,
        },
        notFound: {
          title: '页面不存在',
          quote: '这个页面找不到。检查一下地址——或者从指南开始看。',
          linkLabel: '回到首页',
          linkText: '返回首页',
        },
      },
    },
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
          zh: {
            translations: {
              button: { buttonText: '搜索', buttonAriaLabel: '搜索' },
              modal: {
                displayDetails: '显示详情',
                resetButtonTitle: '清除搜索内容',
                backButtonTitle: '关闭搜索',
                noResultsText: '没有找到相关结果',
                footer: {
                  selectText: '选择',
                  selectKeyAriaLabel: '回车',
                  navigateText: '切换',
                  navigateUpKeyAriaLabel: '上箭头',
                  navigateDownKeyAriaLabel: '下箭头',
                  closeText: '关闭',
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
