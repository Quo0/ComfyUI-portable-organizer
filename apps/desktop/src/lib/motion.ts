import { nextTick, onMounted, onUnmounted, reactive, watch, type Ref } from 'vue';

/**
 * Общий помощник для точечных анимаций компонентов (transitions.dev):
 * тост, выпадающее меню, смена текста статуса — везде нужен один и тот же
 * рецепт «прочитать длительность из CSS-токена и уважить системную
 * настройку уменьшенного движения».
 *
 * Читается один раз при загрузке модуля, а не реактивно: это системная
 * настройка ОС, а не то, что меняется по ходу сессии, — так же устроена
 * и остальная обработка `prefers-reduced-motion` в приложении, через CSS.
 */
const REDUCED = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

/** Длительность CSS-токена движения в миллисекундах; 0, если движение отключено в ОС. */
export function motionMs(varName: string, fallback: number): number {
  if (REDUCED) return 0;
  const raw = getComputedStyle(document.documentElement).getPropertyValue(varName);
  return parseFloat(raw) || fallback;
}

/**
 * Ждёт следующей отрисовки перед вызовом колбэка.
 *
 * Два кадра, а не один: элемент, только что вставленный в DOM, обязан
 * успеть отрисоваться в исходном состоянии до того, как класс сменит его
 * на конечное, — иначе браузер схлопывает оба состояния в один кадр,
 * и анимация не проигрывается вовсе.
 */
export function nextPaint(cb: () => void): void {
  requestAnimationFrame(() => requestAnimationFrame(cb));
}

/**
 * Отслеживает «этот тумблер уже трогали» — для `.is-init` у `.toggle`.
 *
 * Тумблер отрисовывается сразу в своём состоянии, полученном со стороны:
 * из сканирования моделей, из настроек сборки. Это не переход, и играть
 * анимацию перехлёста тут не с чего — она обязана начаться только с первого
 * настоящего клика. Реактивный `Set`, а не булев `ref`: тумблеров на экране
 * бывает много за один рендер (по одному на модель), и трекать их нужно
 * поштучно, по тому же ключу, что уже даёт список отмеченного.
 */
export function useToggleTouch() {
  const touched = reactive(new Set<string>());
  return {
    isTouched: (key: string): boolean => touched.has(key),
    touch: (key: string): void => void touched.add(key),
  };
}

/**
 * Скользящая подчёркивающая плашка у вкладок (transitions.dev, «tabs
 * sliding»). Ширину и смещение считать во время вёрстки нечем — подписи
 * вкладок разной длины и переводятся на четыре языка, — поэтому плашка
 * измеряет активную вкладку в DOM и подстраивается под неё сама.
 *
 * Активная вкладка ищется по `[aria-selected="true"]`, а не приходит
 * параметром: в разметке уже есть эта разметка ради программ чтения
 * с экрана, и вести второй источник истины для одного и того же
 * не нужно.
 *
 * Первая расстановка и ресайз окна — без перехода: иначе плашка на каждую
 * загрузку экрана и на каждое изменение ширины окна наезжала бы с нуля,
 * как будто вкладку только что выбрали.
 */
export function useSlidingTabs(bar: Ref<HTMLElement | null>, active: Ref<unknown>): void {
  function place(animate: boolean): void {
    const el = bar.value;
    const pill = el?.querySelector<HTMLElement>('.tabs-pill');
    const tab = el?.querySelector<HTMLElement>('[role="tab"][aria-selected="true"]');
    if (!el || !pill || !tab) return;
    if (!animate) pill.style.transition = 'none';
    pill.style.transform = `translateX(${tab.offsetLeft}px)`;
    pill.style.width = `${tab.offsetWidth}px`;
    if (!animate) {
      void pill.offsetWidth;
      pill.style.transition = '';
    }
  }
  const onResize = (): void => place(false);
  onMounted(() => window.addEventListener('resize', onResize));
  onUnmounted(() => window.removeEventListener('resize', onResize));
  // И на первое появление панели, и на смену активной — с той разницей,
  // что до появления перехода не бывает: панель вкладок иногда рисуется
  // по `v-if` вместе с тем, что она показывает (библиотека воркфлоу —
  // только когда что-то выбрано), и плашка не должна наезжать с нуля,
  // как будто вкладку только что выбрали.
  watch(bar, (el) => { if (el) nextPaint(() => place(false)); });
  watch(active, () => nextTick(() => place(true)));
}
