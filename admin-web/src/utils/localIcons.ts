/** 本地 SVG 图标名集合（与 assets/icons 下文件名一致） */
const LOCAL_ICON_NAMES = new Set(
  Object.keys(import.meta.glob("@/assets/icons/*.svg"))
    .map((path) => path.split("/assets/icons/")[1]?.replace(/\.svg$/, ""))
    .filter((name): name is string => Boolean(name)),
);

/** 本地 koi-* 图标是否存在于 sprite 资源中 */
export function isLocalIconName(name: string): boolean {
  return LOCAL_ICON_NAMES.has(name);
}

/** 供图标选择器等使用的本地图标名列表 */
export function getLocalIconNames(): string[] {
  return [...LOCAL_ICON_NAMES].sort();
}
