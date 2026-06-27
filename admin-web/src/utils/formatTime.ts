/** 将 Unix 秒时间戳格式化为本地日期时间字符串 */
export function formatUnixTime(sec: number | null | undefined, locale?: string): string {
  if (!sec) return "—";
  const loc = locale ?? (typeof navigator !== "undefined" ? navigator.language : "zh-CN");
  return new Intl.DateTimeFormat(loc, {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(sec * 1000));
}

/** 当前 Unix 秒时间戳 */
export function nowUnix(): number {
  return Math.floor(Date.now() / 1000);
}
