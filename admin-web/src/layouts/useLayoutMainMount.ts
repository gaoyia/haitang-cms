import { ref, type Ref } from "vue";

/** Teleport 目标：各布局壳内的挂载点，切换壳时 Main 实例保持不变 */
export const layoutMainMountEl: Ref<HTMLElement | null> = ref(null);

const mountRegistry = new Map<symbol, HTMLElement>();

/**
 * 注册/注销布局主内容区挂载点。
 * 使用 symbol 避免旧壳 unmount 时清掉新壳已注册的节点。
 */
export function registerLayoutMainMount(id: symbol, el: HTMLElement | null) {
  if (el) {
    mountRegistry.set(id, el);
  } else {
    mountRegistry.delete(id);
  }
  const values = [...mountRegistry.values()];
  layoutMainMountEl.value = values.length > 0 ? values[values.length - 1] : null;
}
