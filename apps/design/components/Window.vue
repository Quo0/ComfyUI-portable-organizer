<script setup lang="ts">
withDefaults(
  defineProps<{
    /** Окно фиксированной высоты — только так возникает настоящая прокрутка. */
    fixed?: boolean;
    /** Кадр в размер настоящего монитора, уменьшенный до ширины страницы. */
    hd?: boolean;
    /**
     * Кадр специально доказывает настоящую прокрутку: данных внутри больше,
     * чем помещается в область. Не то же самое, что `fixed` — `fixed` бывает
     * и просто ради фиксированной высоты (например, чтобы посчитать масштаб
     * `hd`-кадра), без утверждения «тут реально скроллится». `tools/check-styles.mjs`
     * считает кадры с этим пропом и требует от них doказанной плотности данных.
     */
    scroll?: boolean;
    theme?: 'light' | 'dark';
    title?: string;
  }>(),
  { fixed: false, hd: false, scroll: false, theme: 'dark', title: 'ComfyUI Portable Organizer' },
);
</script>

<template>
  <var class="Window">
    <div class="frame" :class="{ hd }" :data-scroll="scroll ? '' : null">
      <div class="win" :class="[`t-${theme}`, { fixed, hd }]">
        <div class="titlebar">
          <span>{{ title }}</span>
          <span class="tb-btns"><span>—</span><span>▢</span><span>✕</span></span>
        </div>
        <div class="win-body">
          <slot name="nav" />
          <slot />
        </div>
      </div>
    </div>
  </var>
</template>

<style scoped>
/* Порт обрамления окна из бывшего design/screens.src.html — оно никогда
   не было частью design/styles/app.css (не компонент приложения, а сама
   идея «вот тут нарисовано окно»), поэтому и живёт теперь при компоненте,
   который эту идею воплощает, а не в общем стилевом файле. */
.frame { overflow-x: auto; padding-bottom: 4px; }
.frame.hd {
  --hd: .55;
  overflow: hidden;
  height: calc(1080px * var(--hd));
}
.frame.hd > .win { transform: scale(var(--hd)); transform-origin: top left; }
@media (max-width: 1180px) {
  .frame.hd { --hd: .45; }
}

/* Верхней границы у ширины раньше не было — на узком экране это скрывала
   собственная ошибка вёрстки страницы (шапка/сайдбар VitePress держали
   колонку контента ýже реальной ширины окна), а с её уходом `.win` стало
   тянуться на весь наличный экран без остатка. Окно приложения такое
   не бывает: `min-width` — это «уже показывать нечем», а сверху нужен
   такой же практический предел, иначе тулбар с `.spacer` посреди пустеет
   на широком мониторе так, что выглядит поломанным. 1440px — то же число,
   что и `.screen-pad.wide` в самом приложении: тот же язык, что и там. */
.win {
  min-width: 940px;
  max-width: 1440px;
  border: 1px solid var(--page-line);
  border-radius: 8px;
  overflow: hidden;
  background: var(--ground);
  color: var(--ink);
  font-family: var(--font-ui);
  font-size: var(--text-base);
  line-height: var(--leading-normal);
}
.win.fixed { height: 560px; display: grid; grid-template-rows: auto minmax(0, 1fr); }
/* hd — кадр в размер настоящего 1920-монитора, уменьшенный `scale()`
   до ширины страницы: верхний предел здесь противоречил бы самой идее
   пропа, поэтому снят обратно именно для него. */
.win.fixed.hd { width: 1920px; max-width: none; height: 1080px; }

.titlebar {
  height: 30px; display: flex; align-items: center; gap: var(--space-2);
  padding: 0 var(--space-3);
  background: var(--surface-sunken); border-bottom: 1px solid var(--line);
  font-size: var(--text-xs); color: var(--ink-muted);
}
.tb-btns { margin-left: auto; display: flex; gap: 16px; color: var(--ink-muted); font-size: 11px; }

.win-body { display: grid; grid-template-columns: auto minmax(0, 1fr); min-height: 430px; }
.win.fixed :deep(.win-body) { min-height: 0; overflow: hidden; }
.win.fixed :deep(.content) { min-height: 0; }

/* Слотовое содержимое несёт разметку родителя ( .md-страницы экрана ),
   поэтому селекторы на неё идут через :deep — как и было задумано этим
   приёмом Vue для содержимого слотов. */
:deep(.nav.in-win) { border: 0; border-right: 1px solid var(--line); border-radius: 0; height: 100%; min-height: 0; }
:deep(.content) { padding: var(--space-4); min-width: 0; display: flex; flex-direction: column; gap: var(--space-4); position: relative; }
:deep(.content.flush) { padding: 0; }
/* Экранный каркас для кадров с настоящей прокруткой: закреплённый верх,
   область данных, закреплённый подвал — тоже жило только в бывшем
   screens.src.html, не в app.css, той же природы, что и .win/.frame выше. */
:deep(.content.framed) { display: grid; grid-template-rows: auto minmax(0, 1fr) auto; gap: var(--space-3); container-type: inline-size; }
:deep(.content.framed.no-foot) { grid-template-rows: auto minmax(0, 1fr); }
:deep(.screen-foot) { display: flex; gap: var(--space-2); justify-content: flex-end; border-top: 1px solid var(--line); padding-top: var(--space-3); }
:deep(.data) { min-height: 0; display: flex; flex-direction: column; }
:deep(.data > .scroll) { border: 1px solid var(--line); border-radius: var(--radius-md); }

/* Место стопки тостов в поддельном окне: угол контентной области, где
   их увидит пользователь. Сам тост — компонент дизайн-системы (app.css),
   а вот куда его класть, знает только оболочка приложения — здесь это
   повторено ровно позиционированием, той же природы, что .frame/.comfy. */
:deep(.win-toasts) {
  position: absolute; z-index: 2;
  right: var(--space-4); bottom: var(--space-4); width: 320px;
}

:deep(.comfy) {
  min-height: 340px;
  background:
    radial-gradient(circle at 1px 1px, var(--line-strong) 1px, transparent 0) 0 0/22px 22px,
    var(--surface-sunken);
  position: relative;
  display: grid; place-items: center;
}
:deep(.comfy-node) {
  position: absolute; width: 132px;
  border: 1px solid var(--line-strong); border-radius: var(--radius-sm);
  background: var(--surface);
  font-size: var(--text-xs);
}
:deep(.comfy-node > b) { display: block; padding: 3px 7px; border-bottom: 1px solid var(--line); font-weight: var(--weight-medium); }
:deep(.comfy-node > span) { display: block; padding: 5px 7px; color: var(--ink-muted); font-family: var(--font-mono); font-size: 10px; }
:deep(.comfy-wire) { position: absolute; height: 1.5px; background: var(--accent-teal); opacity: .75; }
:deep(.comfy-label) {
  font-size: var(--text-xs); color: var(--ink-muted);
  background: var(--ground); border: 1px dashed var(--line-strong);
  padding: 4px 10px; border-radius: var(--radius-pill);
}
</style>
