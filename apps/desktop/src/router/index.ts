import { createRouter, createWebHashHistory } from 'vue-router';

// Настройки, добавление инстанса и редактор аргументов — отдельные роуты,
// а не модалки: дочерний вебвью с ComfyUI это нативное окно поверх нашего
// HTML, и перекрыть его наш интерфейс физически не может.
export const router = createRouter({
  // Хеш, а не история: у окна приложения нет сервера, который отдал бы
  // index.html по произвольному пути после перезагрузки вебвью.
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/instances' },
    {
      path: '/instances',
      name: 'instances',
      component: () => import('../views/InstancesView.vue'),
    },
    // Добавление — отдельный роут, а не модалка: см. дисциплину z-order.
    {
      path: '/instances/add',
      name: 'instance-add',
      component: () => import('../views/AddInstanceView.vue'),
    },
    {
      path: '/instances/:id',
      name: 'instance',
      component: () => import('../views/InstanceView.vue'),
      props: true,
    },
    {
      path: '/install',
      name: 'install',
      component: () => import('../views/InstallView.vue'),
    },
    {
      path: '/install/wizard',
      name: 'install-wizard',
      component: () => import('../views/InstallWizardView.vue'),
    },
    {
      path: '/workflows',
      name: 'workflows',
      component: () => import('../views/WorkflowsView.vue'),
    },
    {
      path: '/settings',
      component: () => import('../views/SettingsView.vue'),
      children: [
        { path: '', redirect: '/settings/appearance' },
        {
          path: 'appearance',
          name: 'settings-appearance',
          component: () => import('../views/settings/AppearanceView.vue'),
        },
        {
          path: 'shared-models',
          name: 'settings-shared-models',
          component: () => import('../views/settings/SharedModelsView.vue'),
        },
        {
          path: 'workflow-library',
          name: 'settings-workflow-library',
          component: () => import('../views/settings/WorkflowLibraryView.vue'),
        },
      ],
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/AboutView.vue'),
    },
    // Спайк Фазы 0. В рейле его нет намеренно: это стенд, а не раздел.
    // Живёт здесь до Фазы 3 как единственный способ проверить, что
    // встроенная вкладка переезжает вслед за сворачиванием рейла.
    {
      path: '/dev/spike',
      name: 'dev-spike',
      component: () => import('../views/DevSpikeView.vue'),
    },
    { path: '/:pathMatch(.*)*', redirect: '/instances' },
  ],
});
