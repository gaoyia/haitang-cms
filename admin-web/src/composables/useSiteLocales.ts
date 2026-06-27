import { ref } from "vue";
import { getDictMapApi } from "@/api/system/dict.ts";
import { uiLangToApiLocale } from "@/utils/apiLocale.ts";
import useGlobalStore from "@/stores/modules/global.ts";

/** 从站点字典加载支持的语言列表与默认语言 */
export function useSiteLocales() {
  const siteLocales = ref<string[]>(["zh-cn", "en-us"]);
  const defaultLocale = ref("zh-cn");
  const previewLang = ref("zh-cn");

  async function loadSiteLocales() {
    const globalStore = useGlobalStore();
    const lang = uiLangToApiLocale(globalStore.language);
    const res = await getDictMapApi(lang);
    if (res.code === 0 && res.data) {
      defaultLocale.value = res.data.site_default_locale?.trim() || "zh-cn";
      const raw = res.data.site_locales?.trim();
      if (raw) {
        siteLocales.value = raw.split(",").map((s) => uiLangToApiLocale(s.trim()));
      }
      if (!siteLocales.value.includes(defaultLocale.value)) {
        siteLocales.value = [defaultLocale.value, ...siteLocales.value];
      }
      previewLang.value = defaultLocale.value;
    }
  }

  function localeLabel(loc: string): string {
    if (loc === "en-us") return "English";
    if (loc === "zh-cn") return "中文";
    return loc;
  }

  function emptyLocaleRecord<T>(factory: () => T): Record<string, T> {
    const map: Record<string, T> = {};
    for (const loc of siteLocales.value) {
      map[loc] = factory();
    }
    return map;
  }

  return {
    siteLocales,
    defaultLocale,
    previewLang,
    loadSiteLocales,
    localeLabel,
    emptyLocaleRecord,
  };
}
