/** 将 admin-web 界面语言映射为 API lang 参数 */
export function uiLangToApiLocale(language: string): string {
  const normalized = language.trim().toLowerCase();
  if (normalized === "en") return "en-us";
  if (normalized === "zh") return "zh-cn";
  if (normalized.startsWith("en")) return "en-us";
  if (normalized.startsWith("zh")) return "zh-cn";
  return normalized || "zh-cn";
}
