<template>
  <div class="home-page" :style="cssVars">
    <!-- 欢迎区 -->
    <el-card class="home-card home-welcome-card" shadow="never" :body-style="cardBodyStyle">
      <div class="home-welcome">
        <el-avatar class="home-welcome__avatar" :size="scale.avatarSize">
          {{ displayName.charAt(0).toUpperCase() }}
        </el-avatar>
        <div class="home-welcome__content">
          <div class="home-welcome__greeting">{{ greetingText }}</div>
          <div class="home-welcome__subtitle">
            {{ $t("menu.home.welcomeBack") }}
            <span class="home-welcome__name">{{ displayName }}</span>
            · {{ siteName }}
          </div>
        </div>
      </div>
    </el-card>

    <!-- 数据概览 -->
    <el-row :gutter="scale.gap" class="home-stat-row">
      <el-col v-for="item in cards" :key="item.title" :xs="24" :sm="12" :lg="6">
        <el-card class="home-card home-stat-card" shadow="never" :body-style="cardBodyStyle">
          <div class="home-stat-card__inner">
            <div class="home-stat-card__info">
              <el-statistic :title="item.title" :value="item.value" />
              <div class="home-stat-card__hint">{{ $t("menu.home.statPending") }}</div>
            </div>
            <div class="home-stat-card__icon" :style="{ background: item.iconBg }">
              <el-icon :size="scale.statIconSize" :style="{ color: item.iconColor }">
                <component :is="item.icon" />
              </el-icon>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 快捷入口 + 系统简介 -->
    <el-row :gutter="scale.gap" class="home-panel-row">
      <el-col :xs="24" :lg="10">
        <el-card class="home-card home-panel-card" shadow="never" :body-style="cardBodyStyle">
          <template #header>
            <div class="home-panel-head">
              <div class="home-panel-head__title">{{ $t("menu.home.quickStart") }}</div>
              <div class="home-panel-head__desc">{{ $t("menu.home.quickStartDesc") }}</div>
            </div>
          </template>
          <div class="home-quick-grid">
            <button
              v-for="link in quickLinks"
              :key="link.path"
              type="button"
              class="home-quick-item"
              @click="$router.push(link.path)"
            >
              <span class="home-quick-item__icon" :style="{ background: link.iconBg, color: link.iconColor }">
                <el-icon :size="scale.quickIconSize"><component :is="link.icon" /></el-icon>
              </span>
              <span class="home-quick-item__body">
                <span class="home-quick-item__label">{{ link.label }}</span>
                <span class="home-quick-item__desc">{{ link.desc }}</span>
              </span>
              <el-icon class="home-quick-item__arrow" :size="scale.arrowIconSize"><ArrowRight /></el-icon>
            </button>
          </div>
        </el-card>
      </el-col>

      <el-col :xs="24" :lg="14">
        <el-card class="home-card home-panel-card" shadow="never" :body-style="cardBodyStyle">
          <template #header>
            <div class="home-panel-head">
              <div class="home-panel-head__title">{{ $t("menu.home.aboutTitle") }}</div>
            </div>
          </template>
          <p class="home-about-desc">{{ $t("menu.home.aboutDesc", { name: siteName }) }}</p>
          <el-space wrap :size="scale.gap / 2.5">
            <el-tag v-for="tag in techTags" :key="tag" type="info" effect="plain" round>{{ tag }}</el-tag>
          </el-space>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { computed, markRaw, reactive } from "vue";
import { useI18n } from "vue-i18n";
import { storeToRefs } from "pinia";
import {
  ArrowRight,
  Document,
  FolderOpened,
  Picture,
  Setting,
  User,
} from "@element-plus/icons-vue";
import { useComponentScale } from "@/composables/useComponentScale.ts";
import { getDayText } from "@/utils/random.ts";
import useAuthStore from "@/stores/modules/auth.ts";
import useUserStore from "@/stores/modules/user.ts";
import useSiteStore from "@/stores/modules/site.ts";

const { t } = useI18n();
const authStore = useAuthStore();
const userStore = useUserStore();
const siteStore = useSiteStore();
const { siteName } = storeToRefs(siteStore);
const { scale, cssVars, cardBodyStyle } = useComponentScale();

const greetingText = computed(() => getDayText() || t("menu.home.greetingFallback"));

const displayName = computed(
  () => authStore.loginUser?.loginName || userStore.loginName || t("menu.home.defaultUser"),
);

const cards = reactive([
  {
    title: t("menu.content.posts"),
    value: "—",
    icon: markRaw(Document),
    iconColor: "#b7102a",
    iconBg: "rgba(183, 16, 42, 0.1)",
  },
  {
    title: t("menu.content.categories"),
    value: "—",
    icon: markRaw(FolderOpened),
    iconColor: "#516169",
    iconBg: "rgba(81, 97, 105, 0.12)",
  },
  {
    title: t("menu.system.users"),
    value: "—",
    icon: markRaw(User),
    iconColor: "#46634c",
    iconBg: "rgba(70, 99, 76, 0.12)",
  },
  {
    title: t("menu.system.auth"),
    value: "—",
    icon: markRaw(Setting),
    iconColor: "#8f6f6e",
    iconBg: "rgba(143, 111, 110, 0.12)",
  },
]);

const quickLinks = computed(() => [
  {
    label: t("menu.content.posts"),
    desc: t("menu.home.linkPosts"),
    path: "/content/posts",
    icon: markRaw(Document),
    iconColor: "#b7102a",
    iconBg: "rgba(183, 16, 42, 0.1)",
  },
  {
    label: t("menu.banner.auth"),
    desc: t("menu.home.linkBanners"),
    path: "/banner",
    icon: markRaw(Picture),
    iconColor: "#516169",
    iconBg: "rgba(81, 97, 105, 0.12)",
  },
  {
    label: t("menu.menu.tree"),
    desc: t("menu.home.linkMenus"),
    path: "/menus",
    icon: markRaw(FolderOpened),
    iconColor: "#46634c",
    iconBg: "rgba(70, 99, 76, 0.12)",
  },
  {
    label: t("menu.system.users"),
    desc: t("menu.home.linkUsers"),
    path: "/system/users",
    icon: markRaw(User),
    iconColor: "#8f6f6e",
    iconBg: "rgba(143, 111, 110, 0.12)",
  },
]);

const techTags = ["Vue 3", "Vite", "Element Plus", "Rust", "Rocket"];
</script>

<style lang="scss" scoped>
.home-page {
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: var(--home-gap, 20px);
}

.home-card {
  border-radius: var(--layout-main-radius, 8px);
}

.home-stat-row,
.home-panel-row {
  width: 100%;
}

.home-stat-row :deep(.el-col) {
  margin-bottom: var(--home-gap, 20px);
}

.home-panel-row :deep(.el-col) {
  margin-bottom: var(--home-gap, 20px);
}

.home-welcome {
  display: flex;
  align-items: center;
  gap: calc(var(--home-gap, 20px) * 0.8);
  min-height: calc(var(--home-avatar-size, 56px) + 20px);
}

.home-welcome__avatar {
  flex-shrink: 0;
  font-size: calc(var(--el-font-size-base) + 8px);
  font-weight: 700;
  color: var(--el-color-primary);
  background: color-mix(in srgb, var(--el-color-primary) 12%, transparent);
  border: 2px solid color-mix(in srgb, var(--el-color-primary) 18%, transparent);
}

.home-welcome__greeting {
  font-size: calc(var(--el-font-size-base) + 6px);
  font-weight: 700;
  line-height: 1.3;
  color: var(--el-text-color-primary);
}

.home-welcome__subtitle {
  margin-top: calc(var(--home-gap, 20px) * 0.4);
  font-size: var(--el-font-size-small);
  line-height: 1.6;
  color: var(--el-text-color-secondary);
}

.home-welcome__name {
  margin: 0 2px;
  font-weight: 600;
  color: var(--el-color-primary);
}

.home-stat-card {
  height: 100%;
}

.home-stat-card__inner {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: calc(var(--home-gap, 20px) * 0.6);
  min-height: calc(var(--home-stat-icon-box, 46px) + 36px);
}

.home-stat-card__info {
  flex: 1;
  min-width: 0;
}

.home-stat-card :deep(.el-statistic__head) {
  font-size: var(--el-font-size-small);
  color: var(--el-text-color-secondary);
}

.home-stat-card :deep(.el-statistic__content) {
  font-size: calc(var(--el-font-size-base) * 2);
  font-weight: 700;
  line-height: 1.1;
  color: var(--el-text-color-primary);
}

.home-stat-card__hint {
  display: block;
  margin-top: calc(var(--home-gap, 20px) * 0.35);
  font-size: var(--el-font-size-extra-small);
  font-weight: 400;
  line-height: 1.4;
  color: var(--el-text-color-placeholder);
}

.home-stat-card__icon {
  display: flex;
  flex-shrink: 0;
  align-items: center;
  justify-content: center;
  width: var(--home-stat-icon-box, 46px);
  height: var(--home-stat-icon-box, 46px);
  border-radius: var(--layout-main-radius, 8px);
}

.home-panel-card {
  height: 100%;
  min-height: var(--home-panel-min-height, 280px);
}

.home-panel-card :deep(.el-card__header) {
  padding: var(--home-card-padding, 16px 20px);
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.home-panel-head__title {
  font-size: calc(var(--el-font-size-base) + 2px);
  font-weight: 700;
  color: var(--el-text-color-primary);
}

.home-panel-head__desc {
  margin-top: calc(var(--home-gap, 20px) * 0.3);
  font-size: var(--el-font-size-extra-small);
  color: var(--el-text-color-secondary);
}

.home-quick-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: calc(var(--home-gap, 20px) * 0.6);
  width: 100%;
}

.home-quick-item {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: calc(var(--home-gap, 20px) * 0.55);
  align-items: center;
  width: 100%;
  min-height: calc(var(--home-quick-icon-box, 40px) + var(--el-component-size) * 0.55);
  padding: calc(var(--el-component-size) * 0.38) calc(var(--el-component-size) * 0.45);
  text-align: left;
  cursor: pointer;
  background: var(--el-fill-color-blank);
  border: 1px solid var(--el-border-color-lighter);
  border-radius: var(--layout-main-radius, 8px);
  outline: none;
  transition:
    border-color 0.2s ease,
    box-shadow 0.2s ease,
    transform 0.2s ease,
    background-color 0.2s ease;

  &:hover {
    background: color-mix(in srgb, var(--el-color-primary) 4%, var(--el-fill-color-blank));
    border-color: color-mix(in srgb, var(--el-color-primary) 35%, var(--el-border-color));
    box-shadow: 0 4px 14px rgb(15 23 42 / 6%);
    transform: translateY(-1px);

    .home-quick-item__arrow {
      color: var(--el-color-primary);
      transform: translateX(2px);
    }
  }

  &:active {
    transform: translateY(0);
  }
}

.home-quick-item__icon {
  display: flex;
  flex-shrink: 0;
  align-items: center;
  justify-content: center;
  width: var(--home-quick-icon-box, 40px);
  height: var(--home-quick-icon-box, 40px);
  border-radius: var(--layout-main-radius, 8px);
}

.home-quick-item__body {
  display: flex;
  flex-direction: column;
  gap: 3px;
  min-width: 0;
}

.home-quick-item__label {
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: var(--el-font-size-base);
  font-weight: 600;
  line-height: 1.35;
  color: var(--el-text-color-primary);
  white-space: nowrap;
}

.home-quick-item__desc {
  display: -webkit-box;
  overflow: hidden;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
  font-size: var(--el-font-size-extra-small);
  line-height: 1.45;
  color: var(--el-text-color-secondary);
}

.home-quick-item__arrow {
  flex-shrink: 0;
  color: var(--el-text-color-placeholder);
  transition:
    color 0.2s ease,
    transform 0.2s ease;
}

.home-about-desc {
  margin: 0 0 calc(var(--home-gap, 20px) * 0.8);
  font-size: var(--el-font-size-small);
  line-height: 1.75;
  color: var(--el-text-color-secondary);
}

@media (width <= 768px) {
  .home-welcome__greeting {
    font-size: calc(var(--el-font-size-base) + 4px);
  }

  .home-quick-grid {
    grid-template-columns: 1fr;
  }
}
</style>
