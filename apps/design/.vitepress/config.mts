import { defineConfig } from 'vitepress';
import { fileURLToPath } from 'node:url';
import { join } from 'node:path';

// The repository root: the app's styles are read from here directly, with
// no copy in this package.
const repoRoot = fileURLToPath(new URL('../../..', import.meta.url));

export default defineConfig({
  title: 'ComfyUI Portable Organizer — design',
  description: 'Style guide and screen mock-ups by scenario',
  lang: 'en',
  cleanUrls: true,
  appearance: false, // the page theme is our own (--page-*)

  themeConfig: {
    nav: [
      { text: 'Style guide', link: '/styleguide/' },
      { text: 'Screens', link: '/screens/' },
    ],
    sidebar: {
      '/styleguide/': [
        {
          text: 'Style guide',
          items: [
            { text: 'Palette', link: '/styleguide/palette' },
            { text: 'Typography', link: '/styleguide/typography' },
            { text: 'Navigation rail', link: '/styleguide/nav-rail' },
            { text: 'Instance card', link: '/styleguide/instance-card' },
            { text: 'Icons', link: '/styleguide/icons' },
            { text: 'Actions', link: '/styleguide/actions' },
            { text: 'Scrolling', link: '/styleguide/scroll' },
            { text: 'Input fields', link: '/styleguide/inputs' },
            { text: 'Notifications', link: '/styleguide/notifications' },
            { text: 'Empty registry', link: '/styleguide/empty-registry' },
            { text: 'Wizard steps', link: '/styleguide/wizard-steps' },
            { text: 'Model categories', link: '/styleguide/model-categories' },
            { text: 'Workflow compatibility', link: '/styleguide/workflow-compat' },
          ],
        },
      ],
      '/screens/': [
        {
          text: 'Instances',
          items: [
            { text: 'Empty registry', link: '/screens/empty-registry' },
            { text: 'Build list', link: '/screens/instance-list' },
            { text: 'Build screen: tabs', link: '/screens/instance-tabs' },
            { text: 'Instance starting', link: '/screens/instance-starting' },
            { text: 'Instance running', link: '/screens/instance-running' },
            { text: 'Messages: toast and banner', link: '/screens/toast-and-banner' },
            { text: 'Launch arguments', link: '/screens/launch-args' },
            { text: 'Connecting shared models', link: '/screens/shared-models' },
            { text: 'Taking a workflow into the library', link: '/screens/workflow-collect' },
            { text: 'Two versions side by side', link: '/screens/two-versions' },
          ],
        },
        {
          text: 'Adding',
          items: [
            { text: 'Two paths', link: '/screens/two-paths' },
            { text: 'The folder already exists', link: '/screens/existing-folder' },
            { text: 'Archive', link: '/screens/archive' },
            { text: 'Destinations', link: '/screens/mappings' },
            { text: 'Shared resources', link: '/screens/shared-resources' },
            { text: 'Unpacking', link: '/screens/unpacking' },
            { text: 'Done', link: '/screens/done' },
          ],
        },
        {
          text: 'Settings',
          items: [
            { text: 'Settings: appearance', link: '/screens/settings-appearance' },
            { text: 'Settings: shared models', link: '/screens/settings-shared-models' },
            { text: 'Shared models on a 1920×1080 monitor', link: '/screens/settings-shared-models-hd' },
            { text: 'Settings: workflow library', link: '/screens/settings-workflow-library' },
            { text: 'Settings: disk report', link: '/screens/settings-disk-report' },
            { text: 'Settings: installer archives', link: '/screens/settings-installer-archives' },
            { text: 'Exiting with builds running', link: '/screens/exit-with-running-instances' },
            { text: 'About the app', link: '/screens/about' },
          ],
        },
        {
          text: 'General',
          items: [
            { text: 'The scrolling rule', link: '/screens/scroll-rule' },
            { text: 'Coverage', link: '/screens/coverage' },
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
