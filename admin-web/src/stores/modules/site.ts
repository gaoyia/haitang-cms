import { defineStore } from "pinia";
import { getDictMapApi } from "@/api/system/dict.ts";
import {
  buildPageTitle,
  DICT_SITE_ICP,
  DICT_SITE_LOGO,
  DICT_SITE_NAME,
  resolveSiteLogoUrl,
  resolveSiteName,
  updateFavicon,
} from "@/utils/siteAsset.ts";
import { uiLangToApiLocale } from "@/utils/apiLocale.ts";
import useGlobalStore from "@/stores/modules/global.ts";

const siteStore = defineStore("site", {
  state: () => ({
    siteName: resolveSiteName(undefined),
    siteLogoRaw: "",
    siteIcp: "",
    loaded: false,
  }),
  getters: {
    /** 已解析、可直接用于 img src 的 Logo 地址 */
    siteLogoUrl: (state) => resolveSiteLogoUrl(state.siteLogoRaw),
  },
  actions: {
    /** 从字典接口加载站点配置（公开接口，登录页也可调用） */
    async fetchSiteConfig() {
      try {
        const globalStore = useGlobalStore();
        const lang = uiLangToApiLocale(globalStore.language);
        const res = await getDictMapApi(lang);
        const map = res.data ?? {};
        this.siteName = resolveSiteName(map[DICT_SITE_NAME]);
        this.siteLogoRaw = map[DICT_SITE_LOGO] ?? "";
        this.siteIcp = map[DICT_SITE_ICP]?.trim() ?? "";
        updateFavicon(this.siteLogoUrl);
      } catch (e) {
        console.warn("[site] 加载字典配置失败，使用默认值", e);
      } finally {
        this.loaded = true;
      }
    },

    /** 设置 document.title（含站点名称后缀） */
    applyDocumentTitle(pageTitle?: string) {
      document.title = buildPageTitle(pageTitle, this.siteName);
    },
  },
});

export default siteStore;
