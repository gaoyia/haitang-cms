/** 将数组中某项从 oldIndex 移到 newIndex，返回新数组 */
export function moveArrayItem<T>(list: readonly T[], oldIndex: number, newIndex: number): T[] {
  const next = list.slice();
  const [moved] = next.splice(oldIndex, 1);
  next.splice(newIndex, 0, moved);
  return next;
}

export interface SortableEntity {
  id: number;
  sort: number;
}

/** 按列表顺序生成 sort 值（可带偏移与步长，用于分页表格） */
export function buildSortUpdates(
  ordered: readonly SortableEntity[],
  offset = 0,
  step = 1,
): { id: number; sort: number }[] {
  return ordered.map((item, index) => ({
    id: item.id,
    sort: (offset + index) * step,
  }));
}

/** 将 sort 写回列表项（拖拽后乐观更新排序列） */
export function applySortUpdates<T extends SortableEntity>(
  items: readonly T[],
  updates: readonly { id: number; sort: number }[],
): T[] {
  const sortMap = new Map(updates.map((item) => [item.id, item.sort]));
  return items.map((item) => {
    const sort = sortMap.get(item.id);
    return sort === undefined ? item : { ...item, sort };
  });
}

/** 过滤出 sort 实际发生变化的项 */
export function diffSortUpdates(
  updates: readonly { id: number; sort: number }[],
  current: readonly SortableEntity[],
): { id: number; sort: number }[] {
  const currentMap = new Map(current.map((item) => [item.id, item.sort]));
  return updates.filter((item) => currentMap.get(item.id) !== item.sort);
}

/** 取下一档 sort（追加到末尾） */
export function nextSortValue(items: readonly SortableEntity[], step = 1): number {
  if (items.length === 0) return 0;
  return Math.max(...items.map((item) => item.sort)) + step;
}
