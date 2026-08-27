import { nextTick } from 'vue';
import { createRouter, createWebHashHistory, START_LOCATION } from 'vue-router';

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/instances' },
    {
      path: '/instances',
      name: 'instances',
      component: () => import('../views/InstancesView.vue'),
    },

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
      path: '/instances/:id/tab',
      name: 'instance-tab',
      component: () => import('../views/InstanceTabView.vue'),
      props: true,
    },

    {
      path: '/instances/:id/args',
      name: 'instance-args',
      component: () => import('../views/ArgsEditorView.vue'),
      props: true,
    },

    {
      path: '/quit',
      name: 'quit',
      component: () => import('../views/QuitView.vue'),
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

    { path: '/workflows', redirect: '/settings/workflow-library' },
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
        {
          path: 'duplicates',
          name: 'settings-duplicates',
          component: () => import('../views/settings/DuplicatesView.vue'),
        },
        {
          path: 'archives',
          name: 'settings-archives',
          component: () => import('../views/settings/ArchivesView.vue'),
        },
      ],
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/AboutView.vue'),
    },
    { path: '/:pathMatch(.*)*', redirect: '/instances' },
  ],
});

router.beforeResolve((to, from) => {
  if (!document.startViewTransition) return;
  if (from === START_LOCATION) return;
  if (to.name === 'instance-tab' || from.name === 'instance-tab') return;

  return new Promise<void>((resolve) => {
    const transition = document.startViewTransition(async () => {
      resolve();

      await nextTick();
    });

    transition.finished.catch(() => {});
  });
});
