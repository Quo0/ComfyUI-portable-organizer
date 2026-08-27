<script setup lang="ts">
import { Archive, Database, HardDrive, Palette, Workflow } from '@lucide/vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

//
</script>

<template>
  <var class="SettingsView">
    <div class="settings-split">
      <nav class="settings-sections">
        <RouterLink class="nav-item" active-class="on" to="/settings/appearance">
          <Palette class="ico" />
          <span>{{ t('settings.section.appearance') }}</span>
        </RouterLink>
        <RouterLink class="nav-item" active-class="on" to="/settings/shared-models">
          <Database class="ico" />
          <span>{{ t('settings.section.sharedModels') }}</span>
        </RouterLink>
        <RouterLink class="nav-item" active-class="on" to="/settings/workflow-library">
          <Workflow class="ico" />
          <span>{{ t('settings.section.library') }}</span>
        </RouterLink>

        <RouterLink class="nav-item" active-class="on" to="/settings/duplicates">
          <HardDrive class="ico" />
          <span>{{ t('settings.section.report') }}</span>
        </RouterLink>
        <RouterLink class="nav-item" active-class="on" to="/settings/archives">
          <Archive class="ico" />
          <span>{{ t('settings.section.archives') }}</span>
        </RouterLink>
      </nav>

      <RouterView />
    </div>
  </var>
</template>
