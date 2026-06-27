<template>
  <el-menu-item :index="item.path" @click="handleClick">
    <!-- 折叠：默认插槽放图标，#title 供 tooltip -->
    <KoiGlobalIcon
      v-if="collapse && item.meta?.icon"
      :name="item.meta.icon"
      size="18"
      class="menu-icon"
    />
    <!-- 展开：图标与文案同在默认插槽，避免与 #title 重复渲染 -->
    <div v-else class="menu-title-wrap">
      <div class="icon-container">
        <KoiGlobalIcon
          v-if="item.meta?.icon"
          :name="item.meta.icon"
          size="18"
          class="menu-icon"
        />
      </div>
      <span class="menu-ellipsis" v-text="title" />
    </div>
    <template v-if="collapse" #title>
      <span class="menu-ellipsis" v-text="title" />
    </template>
  </el-menu-item>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { getMenuLanguage } from "@/utils/index.ts";
import { koiMsgWarning } from "@/utils/koi.ts";
import type { AsideMenuItem } from "@/layouts/components/Menu/types.ts";

const props = defineProps<{
  item: AsideMenuItem;
  collapse: boolean;
}>();

const router = useRouter();
const title = computed(() => getMenuLanguage(props.item.meta?.title ?? ""));

function handleClick() {
  const linkUrl = props.item.meta?.linkUrl;
  if (linkUrl) {
    if (/^https?:\/\//.test(linkUrl)) {
      window.open(linkUrl, "_blank");
      return;
    }
    koiMsgWarning("非正确链接地址，禁止跳转");
    return;
  }
  router.push(props.item.path);
}
</script>

<style lang="scss" scoped>
.menu-title-wrap {
  display: flex;
  align-items: center;
  gap: 2px;
  width: 100%;
}

.menu-ellipsis {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  transform: translate($aside-menu-font-icon-translate);
}

.icon-container {
  display: inline-flex;
  flex-shrink: 0;
  align-items: center;
  justify-content: center;
  transform: translate($aside-menu-font-icon-translate);
}

.menu-icon {
  display: block;
  transition: transform 0.3s ease;
}

:deep(.el-menu-item) {
  height: $aside-menu-height !important;
  margin-bottom: $aside-menu-margin-bottom;
  font-weight: $aside-menu-font-weight;
  --el-menu-item-height: $aside-menu-height;
  color: var(--el-menu-text-color);
  user-select: none;
  border-radius: $aside-menu-border-left;

  &:hover {
    color: var(--el-menu-hover-text-color);
    background: var(--el-menu-hover-bg-color);
    border-radius: $aside-menu-border-left;

    .menu-icon {
      color: var(--el-menu-hover-text-color);
      animation: koi-icon-scale 0.6s ease-in-out forwards;
    }
  }

  &.is-active {
    color: var(--el-menu-active-text-color);
    background: var(--el-menu-active-bg-color);

    &::before {
      border-left: $aside-menu-border-left solid var(--el-menu-border-left-color);
      z-index: auto;
      content: "";
      background-color: transparent;
      position: absolute;
      inset: 0;
      pointer-events: none;
      border-radius: $aside-menu-border-left;
      transition: background-color 0.3s ease-in-out;
    }

    .menu-icon {
      color: var(--el-menu-active-text-color);
    }
  }
}

@keyframes koi-icon-scale {
  0% {
    transform: scale(1);
  }
  70% {
    transform: scale(1.12);
  }
  100% {
    transform: scale(1);
  }
}
</style>
