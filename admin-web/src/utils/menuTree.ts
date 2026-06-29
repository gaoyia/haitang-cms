import type { MenuItem } from "@/api/system/menus.ts";

export interface MenuNodeContext {
  siblings: MenuItem[];
  index: number;
  parentId: number;
}

/** 在树中定位节点及其同级列表 */
export function findMenuNodeContext(
  tree: readonly MenuItem[],
  id: number,
  parentId = 0,
): MenuNodeContext | null {
  const index = tree.findIndex((node) => node.id === id);
  if (index >= 0) {
    return { siblings: tree as MenuItem[], index, parentId };
  }

  for (const node of tree) {
    if (!node.children?.length) continue;
    const found = findMenuNodeContext(node.children, id, node.id);
    if (found) return found;
  }

  return null;
}

/** 获取指定父节点下的同级菜单列表 */
export function getMenuSiblings(tree: readonly MenuItem[], parentId: number): MenuItem[] {
  if (parentId === 0) return tree as MenuItem[];

  function findParent(nodes: readonly MenuItem[]): MenuItem | null {
    for (const node of nodes) {
      if (node.id === parentId) return node;
      if (node.children?.length) {
        const found = findParent(node.children);
        if (found) return found;
      }
    }
    return null;
  }

  return findParent(tree)?.children ?? [];
}

/** 从树结构生成期望的 parent_id 与 sort（同级下标） */
export function buildMenuTreeSortState(
  tree: readonly MenuItem[],
  parentId = 0,
): Map<number, { sort: number; parent_id: number }> {
  const map = new Map<number, { sort: number; parent_id: number }>();
  tree.forEach((item, index) => {
    map.set(item.id, { sort: index, parent_id: parentId });
    if (item.children?.length) {
      const childMap = buildMenuTreeSortState(item.children, item.id);
      childMap.forEach((value, key) => map.set(key, value));
    }
  });
  return map;
}

/** 快照当前树节点的 sort / parent_id */
export function snapshotMenuTreeState(
  tree: readonly MenuItem[],
): Map<number, { sort: number; parent_id: number }> {
  const map = new Map<number, { sort: number; parent_id: number }>();
  function walk(nodes: readonly MenuItem[]) {
    for (const item of nodes) {
      map.set(item.id, { sort: item.sort, parent_id: item.parent_id });
      if (item.children?.length) walk(item.children);
    }
  }
  walk(tree);
  return map;
}

/** 对比快照与期望状态，返回需持久化的变更 */
export function diffMenuTreeSortState(
  before: Map<number, { sort: number; parent_id: number }>,
  after: Map<number, { sort: number; parent_id: number }>,
): { id: number; sort: number; parent_id?: number }[] {
  const updates: { id: number; sort: number; parent_id?: number }[] = [];
  after.forEach((next, id) => {
    const prev = before.get(id);
    if (!prev) return;
    const sortChanged = prev.sort !== next.sort;
    const parentChanged = prev.parent_id !== next.parent_id;
    if (!sortChanged && !parentChanged) return;
    updates.push({
      id,
      sort: next.sort,
      ...(parentChanged ? { parent_id: next.parent_id } : {}),
    });
  });
  return updates;
}

/** 判断 targetId 是否位于 node 的子树中 */
export function isMenuDescendant(node: MenuItem, targetId: number): boolean {
  if (!node.children?.length) return false;
  for (const child of node.children) {
    if (child.id === targetId) return true;
    if (isMenuDescendant(child, targetId)) return true;
  }
  return false;
}
