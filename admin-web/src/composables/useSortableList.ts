import { nextTick, onBeforeUnmount, watch, type Ref } from "vue";
import Sortable from "sortablejs";

export interface UseSortableListOptions {
  /** 返回 Sortable 挂载的 DOM 容器 */
  getContainer: () => HTMLElement | null;
  /** Sortable draggable 选择器，相对 container */
  draggable: string;
  /** 拖拽手柄选择器 */
  handle?: string;
  /** 不可拖拽项选择器 */
  filter?: string;
  disabled?: Ref<boolean>;
  getLength: () => number;
  onReorder: (oldIndex: number, newIndex: number) => void | Promise<void>;
}

/** 绑定 Sortable.js 到列表容器，在数据变化后自动重建实例 */
export function useSortableList(options: UseSortableListOptions) {
  let instance: Sortable | null = null;

  function destroy() {
    instance?.destroy();
    instance = null;
  }

  function init() {
    destroy();
    if (options.disabled?.value) return;

    const el = options.getContainer();
    if (!el || options.getLength() < 2) return;

    instance = Sortable.create(el, {
      handle: options.handle,
      draggable: options.draggable,
      filter: options.filter,
      animation: 150,
      ghostClass: "sortable-ghost",
      onEnd: (evt) => {
        const { oldIndex, newIndex } = evt;
        if (oldIndex == null || newIndex == null || oldIndex === newIndex) return;
        void options.onReorder(oldIndex, newIndex);
      },
    });
  }

  watch(
    () => [options.getContainer(), options.disabled?.value, options.getLength()] as const,
    () => nextTick(init),
    { flush: "post" },
  );

  onBeforeUnmount(destroy);

  return {
    refresh: () => nextTick(init),
  };
}
