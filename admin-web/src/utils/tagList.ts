/** el-input-tag 点击清空时会 emit undefined，统一规范为数组 */
export function normalizeTagList(list: string[] | undefined | null): string[] {
  return Array.isArray(list) ? list : [];
}

/** 将标签数组序列化为 API 逗号分隔字符串 */
export function serializeTags(list: string[] | undefined | null): string {
  return normalizeTagList(list).map((s) => s.trim()).filter(Boolean).join(", ");
}
