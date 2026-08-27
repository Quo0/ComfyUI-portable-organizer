import { nextTick, onMounted, onUnmounted, reactive, watch, type Ref } from 'vue';

const REDUCED = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

export function motionMs(varName: string, fallback: number): number {
  if (REDUCED) return 0;
  const raw = getComputedStyle(document.documentElement).getPropertyValue(varName);
  return parseFloat(raw) || fallback;
}

export function nextPaint(cb: () => void): void {
  requestAnimationFrame(() => requestAnimationFrame(cb));
}

export function useToggleTouch() {
  const touched = reactive(new Set<string>());
  return {
    isTouched: (key: string): boolean => touched.has(key),
    touch: (key: string): void => void touched.add(key),
  };
}

export function withViewTransition(mutate: () => void): void {
  if (!document.startViewTransition) {
    mutate();
    return;
  }
  const transition = document.startViewTransition(() => {
    mutate();
    return nextTick();
  });
  transition.finished.catch(() => {});
}

export function withViewTransitionAsync(mutate: () => Promise<void>): Promise<void> {
  if (!document.startViewTransition) return mutate();
  const transition = document.startViewTransition(async () => {
    await mutate();
    await nextTick();
  });
  transition.finished.catch(() => {});
  return transition.updateCallbackDone;
}

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

  watch(bar, (el) => { if (el) nextPaint(() => place(false)); });
  watch(active, () => nextTick(() => place(true)));
}
