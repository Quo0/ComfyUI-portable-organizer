<script setup lang="ts">
// Вставка воркфлоу текстом.
//
// Отдельный компонент, а не кусок экрана библиотеки: граф присылают текстом
// чаще, чем файлом, и принять его нужно будет и на вкладке «Воркфлоу»
// у сборки. Поэтому здесь только форма и разбор, а куда лёг результат —
// решает тот, кто её показал: он и передаёт `save`.
//
// Форма, а не всплывающий диалог, по той же причине: на экране сборки
// поверх нашего HTML лежит нативное окно ComfyUI, и всплыть над ним
// физически не над чем.
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import type { AppError } from '../bindings';
import { errorText } from '../lib/errors';

const props = defineProps<{
  /**
   * Куда класть. Возвращает ошибку, а не показывает её: ответ «имя занято»
   * относится к полю имени, и в тосте он уезжает от этого поля
   * в противоположный угол экрана.
   */
  save: (name: string, content: string) => Promise<AppError | null>;
}>();

const emit = defineEmits<{ done: [name: string]; cancel: [] }>();

const { t } = useI18n();

const name = ref('');
const content = ref('');
const saving = ref(false);
const failure = ref<AppError | null>(null);
const area = ref<HTMLTextAreaElement | null>(null);
const nameInput = ref<HTMLInputElement | null>(null);

/**
 * Разбор идёт по ходу набора, а не по нажатию «Сохранить»: спрашивать
 * о том, ответ на что уже известен, — не проверка, а лишний шаг.
 *
 * Три состояния, и пустое поле не выдаётся за ошибку: пока ничего
 * не вставили, ругаться не на что.
 */
const graph = computed<{ ok: boolean; nodes: number; types: string[] } | null>(() => {
  const text = content.value.trim();
  if (!text) return null;
  try {
    const value: unknown = JSON.parse(text);
    const nodes = (value as { nodes?: unknown }).nodes;
    if (!Array.isArray(nodes)) return { ok: false, nodes: 0, types: [] };
    const types = [
      ...new Set(
        nodes
          .map((node: unknown) => (node as { type?: unknown }).type)
          .filter((kind): kind is string => typeof kind === 'string'),
      ),
    ];
    return { ok: true, nodes: nodes.length, types };
  } catch {
    return { ok: false, nodes: 0, types: [] };
  }
});

/** Тот же разбор, но до присвоения: подставлять из буфера мусор незачем. */
function looksLikeWorkflow(text: string): boolean {
  try {
    return Array.isArray((JSON.parse(text) as { nodes?: unknown }).nodes);
  } catch {
    return false;
  }
}

/** Что получится: имя без расширения плюс `.json`. Ровно как в Rust. */
const fileName = computed(() => {
  const stem = name.value.trim().replace(/\.json$/i, '').trimEnd();
  return stem ? `${stem}.json` : '';
});

const canSave = computed(() => !saving.value && !!fileName.value && graph.value?.ok === true);

/**
 * Буфер читается сам, но тихо.
 *
 * Кнопка называется «Вставить из буфера», и заставлять после неё жать
 * Ctrl+V было бы обманом. Но право на чтение буфера webview даёт не всегда,
 * и отказ здесь — не ошибка: поле просто останется пустым, и Ctrl+V
 * в нём работает как обычно.
 *
 * Подставляется только то, что разобралось в граф. В буфере чаще лежит
 * что угодно другое, и встречать пользователя красной строкой про то,
 * чего он не вставлял, — не помощь.
 */
onMounted(async () => {
  area.value?.focus();
  try {
    const text = await navigator.clipboard.readText();
    if (text.trim() && looksLikeWorkflow(text)) content.value = text;
  } catch {
    // Молча: пользователь ничего не терял.
  }
});

async function submit(): Promise<void> {
  if (!canSave.value) return;
  saving.value = true;
  failure.value = null;
  try {
    const error = await props.save(name.value, content.value.trim());
    if (error) {
      failure.value = error;
      // Занятое имя относится к полю — толчок и там же. Переигрывается
      // явно из места отказа, а не реактивно от текста ошибки: вторая
      // подряд попытка с тем же именем даёт тот же самый текст, и `watch`
      // по значению его бы не заметил, а percussive-намёк должен
      // повториться на каждый настоящий отказ, а не только на первый.
      if (nameFailure.value) shakeName();
      return;
    }
    emit('done', fileName.value);
  } finally {
    saving.value = false;
  }
}

/** Отказ, относящийся к имени, стоит под именем, а не общей строкой. */
const nameFailure = computed(() =>
  failure.value?.code === 'workflows.nameTaken' || failure.value?.code === 'workflows.badName'
    ? errorText(failure.value)
    : null,
);

const otherFailure = computed(() =>
  failure.value && !nameFailure.value ? errorText(failure.value) : null,
);

/**
 * Percussive-толчок поля имени при отказе (transitions.dev, «error state
 * shake»). Красная рамка и текст под полем уже держатся на `.bad`
 * и `nameFailure` — здесь только сама тряска, класс `.is-shaking` снят,
 * положен заново после перерисовки: без сброса второй отказ подряд
 * не переиграл бы уже идущую анимацию.
 *
 * Автосброса ошибки по таймеру нет и не будет: занятое имя остаётся
 * видимым, пока пользователь не поправит поле или не отправит форму
 * заново, — как и любая другая ошибка в этом приложении. Отпускать её
 * молча через три секунды значило бы прятать то, что ещё не решено.
 */
function shakeName(): void {
  const el = nameInput.value;
  if (!el) return;
  el.classList.remove('is-shaking');
  void el.offsetWidth;
  el.classList.add('is-shaking');
}
</script>

<template>
  <form class="paste" @submit.prevent="submit">
    <div class="field">
      <label class="t-label" for="paste-name">{{ t('library.paste.name') }}</label>
      <input
        id="paste-name"
        ref="nameInput"
        v-model="name"
        class="input"
        :class="{ bad: !!nameFailure }"
        type="text"
        autocomplete="off"
        :placeholder="t('library.paste.namePlaceholder')"
      />
      <!-- Расширение дописывается само: набранное руками «.jsn»
           или «.json.json» — описка, а не выбор. -->
      <p v-if="nameFailure" class="hint bad">{{ nameFailure }}</p>
      <p v-else-if="fileName" class="hint">
        {{ t('library.paste.willSaveAs', { name: fileName }) }}
      </p>
    </div>

    <div class="field">
      <label class="t-label" for="paste-json">{{ t('library.paste.json') }}</label>
      <textarea
        id="paste-json"
        ref="area"
        v-model="content"
        class="input area mono"
        :class="{ bad: graph?.ok === false }"
        spellcheck="false"
        :placeholder="t('library.paste.jsonPlaceholder')"
      ></textarea>

      <!-- «Разобрано N нод» — не украшение: это единственный способ увидеть,
           что вставилось то самое, не читая двух тысяч строк JSON. -->
      <p v-if="graph?.ok" class="hint">
        {{ t('library.paste.parsed', graph.nodes) }}
        <template v-if="graph.types.length">
          · {{ t('library.paste.types', graph.types.length) }}
        </template>
      </p>
      <p v-else-if="graph" class="hint bad">{{ t('library.paste.notAWorkflow') }}</p>
      <p v-else class="hint">{{ t('library.paste.hint') }}</p>
    </div>

    <p v-if="otherFailure" class="hint bad">{{ otherFailure }}</p>

    <div class="row">
      <button type="submit" class="btn primary" :disabled="!canSave">
        {{ t('library.paste.save') }}
      </button>
      <button type="button" class="btn ghost" @click="emit('cancel')">
        {{ t('common.cancel') }}
      </button>
    </div>
  </form>
</template>
