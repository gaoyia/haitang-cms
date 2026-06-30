/** 轮播 Hero 操作按钮 */
export interface BannerHeroAction {
  label: string;
  url: string;
  variant: "primary" | "secondary";
}

/** 单语言 Hero overlay 文案 */
export interface BannerHeroLocale {
  badge: string;
  title: string;
  description: string;
  tags: string[];
  actions: BannerHeroAction[];
}

export function emptyBannerHeroLocale(): BannerHeroLocale {
  return {
    badge: "",
    title: "",
    description: "",
    tags: [],
    actions: [],
  };
}

export function emptyBannerHeroAction(): BannerHeroAction {
  return { label: "", url: "", variant: "primary" };
}

export function parseBannerHeroMeta(raw?: string): Record<string, BannerHeroLocale> {
  if (!raw?.trim()) return {};
  try {
    const obj = JSON.parse(raw) as Record<string, unknown>;
    if (!obj || typeof obj !== "object") return {};
    const map: Record<string, BannerHeroLocale> = {};
    for (const [lang, val] of Object.entries(obj)) {
      if (lang.startsWith("_") || !val || typeof val !== "object") continue;
      const row = val as Record<string, unknown>;
      map[lang] = {
        badge: String(row.badge ?? ""),
        title: String(row.title ?? ""),
        description: String(row.description ?? ""),
        tags: Array.isArray(row.tags)
          ? row.tags.map((t) => String(t).trim()).filter(Boolean)
          : [],
        actions: Array.isArray(row.actions)
          ? row.actions
              .map((item) => {
                const a = item as Record<string, unknown>;
                const variant = a.variant === "secondary" ? "secondary" : "primary";
                return {
                  label: String(a.label ?? "").trim(),
                  url: String(a.url ?? "").trim(),
                  variant,
                } satisfies BannerHeroAction;
              })
              .filter((a) => a.label && a.url)
          : [],
      };
    }
    return map;
  } catch {
    return {};
  }
}

export function buildBannerHeroMetaJson(
  i18n: Record<string, BannerHeroLocale>,
  locales: readonly string[],
): string {
  const payload: Record<string, BannerHeroLocale> = {};
  for (const loc of locales) {
    const row = i18n[loc] ?? emptyBannerHeroLocale();
    const locale: BannerHeroLocale = {
      badge: row.badge.trim(),
      title: row.title.trim(),
      description: row.description.trim(),
      tags: row.tags.map((t) => t.trim()).filter(Boolean),
      actions: row.actions
        .map((a) => ({
          label: a.label.trim(),
          url: a.url.trim(),
          variant: (a.variant === "secondary" ? "secondary" : "primary") as BannerHeroAction["variant"],
        }))
        .filter((a) => a.label && a.url),
    };
    if (
      locale.badge ||
      locale.title ||
      locale.description ||
      locale.tags.length ||
      locale.actions.length
    ) {
      payload[loc] = locale;
    }
  }
  return JSON.stringify(payload);
}

export function ensureBannerHeroI18n(
  raw: Record<string, BannerHeroLocale>,
  locales: readonly string[],
): Record<string, BannerHeroLocale> {
  const map: Record<string, BannerHeroLocale> = {};
  for (const loc of locales) {
    map[loc] = raw[loc] ? { ...raw[loc], tags: [...raw[loc].tags], actions: raw[loc].actions.map((a) => ({ ...a })) } : emptyBannerHeroLocale();
  }
  return map;
}
