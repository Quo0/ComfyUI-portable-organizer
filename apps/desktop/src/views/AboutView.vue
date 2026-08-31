<script setup lang="ts">

import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import Group from '../components/ui/Group.vue';
import KeyValueList from '../components/ui/KeyValueList.vue';
import KeyValueRow from '../components/ui/KeyValueRow.vue';
import ScreenHeader from '../components/ui/ScreenHeader.vue';
import OpenFolderButton from '../components/OpenFolderButton.vue';
import PathText from '../components/PathText.vue';
import UpdatePanel from '../components/UpdatePanel.vue';
import { useInstancesStore } from '../stores/instances';
import { useSharedStore } from '../stores/shared';
import { useWorkflowsStore } from '../stores/workflows';
import { useUiStore } from '../stores/ui';

const ui = useUiStore();
const instances = useInstancesStore();
const shared = useSharedStore();
const workflows = useWorkflowsStore();
const { t } = useI18n();

const sharedRoot = computed(() => shared.root?.path ?? '');
const libraryPath = computed(() => workflows.path);

onMounted(async () => {
  if (!instances.loaded) await instances.load();
  if (!shared.loaded) await shared.load();
  if (!workflows.loaded) await workflows.load();
});
</script>

<template>
  <var class="AboutView">
    <section class="screen">

      <ScreenHeader>
        <h1 class="t-lg">{{ t('about.title') }}</h1>
        <span class="lead">
          {{ t('about.version') }} <span class="t-mono">{{ ui.version }}</span>
        </span>
      </ScreenHeader>

      <div class="screen-body">

        <div class="screen-pad wide">

          <div class="about-grid">
            <Group class="cell-uninstall">
              <span class="t-label">{{ t('about.uninstall.title') }}</span>

              <KeyValueList with-acts>
                <KeyValueRow>
                  <span class="lbl">
                    {{ t('about.paths.appData') }}
                    <span class="hint">
                      <PathText :path="ui.appDataDir" />
                    </span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton :path="ui.appDataDir" :title="ui.appDataDir" />
                  </span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">
                    {{ t('about.paths.localData') }}
                    <span class="hint">
                      <PathText :path="ui.appLocalDataDir" />
                    </span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton :path="ui.appLocalDataDir" :title="ui.appLocalDataDir" />
                  </span>
                </KeyValueRow>
              </KeyValueList>
              <p class="hint">{{ t('about.uninstall.body') }}</p>
            </Group>

            <Group class="cell-content">
              <span class="t-label">{{ t('about.content.title') }}</span>

              <KeyValueList with-acts>
                <KeyValueRow>
                  <span class="lbl">
                    {{ t('about.content.shared') }}
                    <span class="hint">
                      <PathText v-if="sharedRoot" :path="sharedRoot" />
                      <template v-else>{{ t('about.content.notSet') }}</template>
                    </span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton :path="sharedRoot" :title="sharedRoot || undefined" />
                  </span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">
                    {{ t('about.content.library') }}
                    <span class="hint">
                      <PathText v-if="libraryPath" :path="libraryPath" />
                      <template v-else>{{ t('about.content.notSet') }}</template>
                    </span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton :path="libraryPath" :title="libraryPath || undefined" />
                  </span>
                </KeyValueRow>
              </KeyValueList>
              <p class="hint">{{ t('about.content.body') }}</p>
            </Group>

            <Group class="cell-written">
              <span class="t-label">{{ t('about.written.title') }}</span>
              <KeyValueList>
                <KeyValueRow>
                  <span class="lbl">{{ t('about.written.yaml') }}</span>
                  <span class="val">{{ t('about.written.yamlWhen') }}</span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">{{ t('about.written.workflows') }}</span>
                  <span class="val">{{ t('about.written.workflowsWhen') }}</span>
                </KeyValueRow>
              </KeyValueList>
              <p class="hint">{{ t('about.written.body') }}</p>
            </Group>

            <Group v-if="instances.items.length" class="cell-instances">
              <span class="t-label">{{ t('about.content.instances') }}</span>
              <KeyValueList with-acts>
                <KeyValueRow v-for="instance in instances.items" :key="instance.id">

                  <span class="lbl">
                    {{ instance.name }}
                    <span class="hint">
                      <PathText :path="instance.path" />
                    </span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton :path="instance.path" :disabled="!instance.available" :title="instance.path" />
                  </span>
                </KeyValueRow>
              </KeyValueList>
              <p class="hint">{{ t('about.content.instancesBody') }}</p>
            </Group>
          </div>

          <UpdatePanel />
        </div>
      </div>
    </section>
  </var>
</template>

<style scoped>

.screen-pad {
  container-type: inline-size;
}

.about-grid {
  display: grid;
  grid-template-columns: 1fr;
  grid-template-areas:
    'uninstall'
    'content'
    'written'
    'instances';
  gap: var(--space-4);
}

.cell-uninstall {
  grid-area: uninstall;
}

.cell-content {
  grid-area: content;
}

.cell-written {
  grid-area: written;
}

.cell-instances {
  grid-area: instances;
}

@container (min-width: 980px) {
  .about-grid {
    grid-template-columns: 1fr 1fr;
    grid-template-areas:
      'uninstall content'
      'written instances';
  }
}
</style>
