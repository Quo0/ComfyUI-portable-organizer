<script setup lang="ts">
// Редактор аргументов запуска.
//
// Отдельный роут, а не модалка: дисциплина z-order. И отдельный экран
// по существу — здесь пользователь читает итоговую команду, а это не то,
// что разглядывают во всплывашке.
//
// Файлы .bat самой сборки не меняются никогда. Правка сохраняется своим
// профилем поверх одного из них: разбор `.bat` перечитывается при каждом
// запуске, и удержать правку внутри него было бы негде.
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import { commands, type CustomProfile, type LaunchProfile } from '../bindings';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';
import { useUiStore } from '../stores/ui';

const props = defineProps<{ id: string }>();

const instances = useInstancesStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();

const instance = computed(() => instances.byId(props.id));
const all = computed<LaunchProfile[]>(() => run.profiles[props.id] ?? []);
/** Профили из `.bat`: только они могут быть основой. */
const bases = computed(() => all.value.filter((p) => !p.id.startsWith('custom:')));
const mine = computed(() => instance.value?.customProfiles ?? []);

const draft = ref<CustomProfile>({ id: '', name: '', baseId: '', args: [] });
const preview = ref<string[]>([]);

onMounted(async () => {
  if (!instances.loaded) await instances.load();
  await run.loadProfiles(props.id);
  if (!draft.value.baseId) reset(bases.value[0]?.id ?? '');
});

watch(bases, (list) => {
  if (!draft.value.baseId && list.length) reset(list[0].id);
});

/** Берёт аргументы базового профиля как есть — то, что написано в .bat. */
function reset(baseId: string): void {
  const base = all.value.find((p) => p.id === baseId);
  draft.value = {
    id: draft.value.id,
    name: draft.value.name || (base ? `${base.name} +` : ''),
    baseId,
    args: [...(base?.args ?? [])],
  };
  void refreshPreview();
}

function edit(profile: CustomProfile): void {
  draft.value = { ...profile, args: [...profile.args] };
  void refreshPreview();
}

function addArg(): void {
  draft.value.args.push('');
}

function removeArg(index: number): void {
  draft.value.args.splice(index, 1);
  void refreshPreview();
}

/**
 * Предпросмотр считает Rust, а не фронт: мутацию аргументов делает он,
 * и повторять её здесь значило бы завести вторую правду о команде.
 *
 * Пока профиль не сохранён, Rust о черновике не знает — поэтому от него
 * берётся только хвост, дописываемый при запуске, а сами аргументы
 * подставляются из черновика. Иначе предпросмотр показывал бы то,
 * что лежит в `.bat`, а не то, что пользователь только что набрал.
 */
async function refreshPreview(): Promise<void> {
  const source = draft.value.baseId;
  if (!source) return;
  const res = await commands.previewCommand(props.id, source);
  if (res.status !== 'ok') {
    preview.value = [];
    return;
  }
  const base = run.profiles[props.id]?.find((p) => p.id === source);
  const added = res.data.slice(1 + (base?.args.length ?? 0));
  preview.value = [
    res.data[0],
    ...draft.value.args.filter((a) => a.trim() !== ''),
    ...added,
  ];
}

// Реактивность вместо кнопки «обновить»: предпросмотр обязан отвечать
// на каждый набранный символ.
watch(
  () => [draft.value.args.join(' '), draft.value.baseId],
  () => void refreshPreview(),
  { deep: true },
);

async function save(): Promise<void> {
  const res = await commands.saveCustomProfile(props.id, {
    ...draft.value,
    args: draft.value.args.filter((a) => a.trim() !== ''),
  });
  if (res.status === 'error') {
    ui.pushError(res.error);
    return;
  }
  instances.replace(res.data);
  await run.loadProfiles(props.id);
  ui.pushOk(t('args.saved', { name: draft.value.name }));
  draft.value = { id: '', name: '', baseId: draft.value.baseId, args: draft.value.args };
  await refreshPreview();
}

async function remove(profile: CustomProfile): Promise<void> {
  const res = await commands.removeCustomProfile(props.id, profile.id);
  if (res.status === 'error') {
    ui.pushError(res.error);
    return;
  }
  instances.replace(res.data);
  await run.loadProfiles(props.id);
  ui.pushOk(t('args.removed', { name: profile.name }));
  if (draft.value.id === profile.id) reset(draft.value.baseId);
}
</script>

<template>
  <section v-if="instance" class="screen">
    <!-- Ряд шага, как в мастере и на добавлении инстанса: экран линейный —
         пришли сюда с экрана сборки, сохранили, вернулись. Поэтому «Назад»
         здесь не навигация у левого края, а шаг назад рядом с действием,
         и стоит левее его.

         Внизу «Сохранить» уезжала из виду тем вернее, чем больше аргументов
         правят: под ней скроллится их список. -->
    <div class="step-bar">
      <h2 class="title">{{ t('args.title') }}</h2>
      <!-- Имя сборки при заголовке, а не у правого края: оно уточняет,
           чьи аргументы правятся, и в отрыве читалось как чужая строка. -->
      <span class="hint">{{ instance.name }}</span>
      <span class="spacer"></span>
      <span class="acts">
        <RouterLink class="btn ghost" :to="`/instances/${instance.id}`">
          <svg class="ico"><use href="#i-back" /></svg>
          {{ t('common.back') }}
        </RouterLink>
        <button
          type="button"
          class="btn primary lg"
          :disabled="draft.name.trim() === '' || !draft.baseId"
          @click="save"
        >
          {{ t('common.save') }}
        </button>
      </span>
    </div>

    <div class="screen-body">
      <div class="screen-pad">
        <p class="t-sm">{{ t('args.lead') }}</p>

        <div class="group">
          <span class="t-label">{{ t('args.mine') }}</span>
          <div v-if="mine.length" class="wf-list">
            <div v-for="p in mine" :key="p.id" class="wf-row">
              <span class="nm">{{ p.name }}</span>
              <span class="hint">{{ p.baseId }}</span>
              <button type="button" class="btn ghost" @click="edit(p)">
                {{ t('common.edit') }}
              </button>
              <button type="button" class="btn ghost" @click="remove(p)">
                {{ t('instances.remove.action') }}
              </button>
            </div>
          </div>
          <p v-else class="hint">{{ t('args.none') }}</p>
        </div>

        <div class="two">
          <div class="field">
            <label for="base">{{ t('args.base') }}</label>
            <!-- Имена профилей — имена файлов, они не переводятся. -->
            <select
              id="base"
              class="input"
              :value="draft.baseId"
              @change="reset(($event.target as HTMLSelectElement).value)"
            >
              <option v-for="p in bases" :key="p.id" :value="p.id">{{ p.id }}</option>
            </select>
          </div>
          <div class="field">
            <label for="pname">{{ t('args.name') }}</label>
            <input id="pname" v-model="draft.name" class="input" type="text" />
          </div>
        </div>

        <div class="group">
          <span class="t-label">{{ t('args.list') }}</span>
          <div v-for="(_, index) in draft.args" :key="index" class="path-row">
            <!-- Предпросмотр обновляется на каждый ввод, а не по потере
                 фокуса: правишь аргумент — сразу видишь команду. -->
            <input v-model="draft.args[index]" class="input mono" type="text" />
            <!-- Тот же крестик, что убирает строку в списке назначений
                 мастера: одна операция — один значок. -->
            <span class="acts">
              <button
                type="button"
                class="act"
                :title="t('args.remove')"
                :aria-label="t('args.remove')"
                @click="removeArg(index)"
              >
                <svg class="ico"><use href="#i-close" /></svg>
              </button>
            </span>
          </div>
          <div class="row">
            <button type="button" class="btn secondary" @click="addArg">
              {{ t('args.add') }}
            </button>
            <button type="button" class="btn ghost" @click="reset(draft.baseId)">
              {{ t('args.reset') }}
            </button>
          </div>
        </div>

        <div class="group">
          <span class="t-label">{{ t('args.preview') }}</span>
          <!-- Команда не переводится и не сокращается: по ней разбираются. -->
          <pre class="console preview">{{ preview.join(' ') }}</pre>
          <p class="hint">{{ t('args.previewHint') }}</p>
        </div>

        <!-- Кнопки под формой нет: действие уехало в ряд шага. -->
      </div>
    </div>
  </section>
</template>

<style scoped>
/* Ряд шага встал на место шапки экрана и берёт её верхнее поле:
   без него заголовок прилипает к заголовку окна. */
.step-bar {
  padding-top: var(--space-4);
}

/* Вид предпросмотра живёт в дизайн-системе (`.console.preview`):
   здесь он был только у приложения, и макет рисовал команду одной
   строкой с прокруткой — то есть показывал не то, что на экране. */
</style>
