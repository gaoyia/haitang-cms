/** 规范语言码（与后端 normalize_lang 简化对齐） */
export function normalizeLocale(loc: string): string {
  const s = loc.trim().toLowerCase();
  if (!s) return "zh-cn";
  if (s === "zh" || s === "zh-hans") return "zh-cn";
  if (s === "en" || s === "en-gb") return "en-us";
  return s;
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** 匹配路径首段是否为指定语言码：/zh-cn、/zh-cn/test（大小写不敏感，整段匹配） */
function localePrefixRegExp(loc: string): RegExp {
  const tag = escapeRegExp(normalizeLocale(loc));
  return new RegExp(`^/${tag}(?=/|$)`, "i");
}

/** 公开页语言 URL 前缀，如 /zh-cn */
export function localePathPrefix(loc: string): string {
  return `/${normalizeLocale(loc)}`;
}

/** 判断路径首段是否为指定语言码 */
export function hasLocalePrefix(loc: string, routePath: string): boolean {
  const trimmed = routePath.trim();
  if (!trimmed) return false;
  return localePrefixRegExp(loc).test(trimmed);
}

/** 表单路径是否视为空（不写入、不补前缀） */
export function isBlankLocalePath(path: string): boolean {
  const trimmed = path.trim();
  return !trimmed || trimmed === "/";
}

/**
 * 从完整 route_path 解析去前缀后的路径（用于表单展示）
 *
 * - /zh-cn/test → /test
 * - /test → /test（首段非语言码，原样回显）
 */
export function parseLocalePath(loc: string, routePath: string): string {
  const trimmed = routePath.trim();
  if (!trimmed) return "";

  const re = localePrefixRegExp(loc);
  if (!re.test(trimmed)) return trimmed;

  const tag = normalizeLocale(loc);
  const stripped = trimmed.replace(
    new RegExp(`^/${escapeRegExp(tag)}(?=/|$)`, "i"),
    "",
  );
  if (!stripped || stripped === "/") return "";
  return stripped.startsWith("/") ? stripped : `/${stripped}`;
}

/** 将表单路径转为存储用的完整 route_path */
export function buildLocalePath(loc: string, path: string, autoPrefix: boolean): string {
  const trimmed = path.trim();
  if (!autoPrefix) return trimmed;

  if (isBlankLocalePath(trimmed)) return "";

  const prefix = localePathPrefix(loc);
  const segment = trimmed.startsWith("/") ? trimmed : `/${trimmed}`;
  return `${prefix}${segment}`;
}

/** 是否启用自动语言前缀（回显时推断：各语言非空路径均须带对应前缀，空的忽略） */
export function shouldAutoLocalePrefix(
  pathsByLocale: Readonly<Record<string, string>>,
  siteLocales: readonly string[],
): boolean {
  for (const loc of siteLocales) {
    const stored = pathsByLocale[loc] ?? "";
    if (isBlankLocalePath(stored)) continue;
    if (!hasLocalePrefix(loc, stored)) return false;
  }
  return true;
}
