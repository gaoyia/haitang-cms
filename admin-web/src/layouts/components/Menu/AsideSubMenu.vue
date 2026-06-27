<template>
  <template v-for="item in menuList" :key="item.path">
    <el-sub-menu v-if="item.children?.length" :index="item.path">
      <template #title>
        <KoiGlobalIcon
          v-if="collapse && item.meta?.icon"
          :name="item.meta.icon"
          size="18"
          class="menu-icon"
        />
        <div v-else class="menu-title-wrap">
          <div class="icon-container">
            <KoiGlobalIcon
              v-if="item.meta?.icon"
              :name="item.meta.icon"
              size="18"
              class="menu-icon"
            />
          </div>
          <span class="menu-ellipsis" v-text="getMenuLanguage(item.meta?.title ?? '')" />
        </div>
      </template>
      <AsideSubMenu :menu-list="item.children" :collapse="collapse" />
    </el-sub-menu>
    <AsideMenuLeaf v-else :item="item" :collapse="collapse" />
  </template>
</template>

<script setup lang="ts">
import AsideSubMenu from "@/layouts/components/Menu/AsideSubMenu.vue";
import AsideMenuLeaf from "@/layouts/components/Menu/AsideMenuLeaf.vue";
import type { AsideMenuItem } from "@/layouts/components/Menu/types.ts";
import { getMenuLanguage } from "@/utils/index.ts";

defineProps<{
  menuList: AsideMenuItem[];
  collapse: boolean;
}>();
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
  position: relative;
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

:deep(.el-sub-menu__title) {
  height: $aside-menu-height;
  padding-right: 0;
  margin-bottom: $aside-menu-margin-bottom;
  font-weight: $aside-menu-font-weight;
  color: var(--el-menu-text-color);
  user-select: none;
  border-radius: $aside-menu-border-left;

  &:hover {
    color: var(--el-menu-hover-text-color);
    background: var(--el-menu-hover-bg-color);

    .menu-icon {
      animation: koi-icon-scale 0.6s ease-in-out forwards;
      color: var(--el-menu-hover-text-color);
    }
  }
}

@keyframes koi-icon-scale {
  0% {
    transform: scale(1);
    box-shadow: 0 0 0 0 rgba(255, 204, 0, 0);
  }
  70% {
    transform: scale(1.12);
    box-shadow: 0 0 0 15px rgba(255, 204, 0, 0);
  }
  100% {
    transform: scale(1);
    box-shadow: 0 0 0 0 rgba(255, 204, 0, 0);
  }
}
</style>

<style lang="scss">
/* 子级高亮时父级标题同步变色；popup 去边框见下方 */
.el-sub-menu.is-active > .el-sub-menu__title {
  color: var(--el-menu-parent-active-text-color) !important;

  .menu-icon {
    color: var(--el-menu-parent-active-text-color) !important;
  }
}

.el-popper.is-pure {
  border: none !important;
  outline: none !important;
}

.el-menu--popup-container .el-menu--popup,
.el-menu--popup {
  border: none !important;
  outline: none !important;

  .el-menu-item,
  .el-sub-menu,
  .el-sub-menu__title {
    border: none !important;
    outline: none !important;
  }
}
</style>
