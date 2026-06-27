<!-- 这里是一次性加载 LayoutComponents -->
<template>
  <div class="layout-app">
    <component :is="activeLayoutComponent" />
    <!-- Main 与布局壳解耦：窄屏/桌面/主题布局切换时不随壳 unmount -->
    <Teleport :to="layoutMainMountEl" :disabled="!layoutMainMountEl">
      <Main />
    </Teleport>
    <component :is="ThemeConfig" />
  </div>
</template>

<script setup lang="ts" name="layout">
import { useScreenStore } from "@/hooks/screen/index.ts";
import { computed, type Component } from "vue";
import useGlobalStore from "@/stores/modules/global.ts";
import { layoutMainMountEl } from "@/layouts/useLayoutMainMount.ts";
import ThemeConfig from "@/layouts/components/ThemeConfig/index.vue";
import Main from "@/layouts/components/Main/index.vue";
import LayoutVertical from "@/layouts/LayoutVertical/index.vue";
import LayoutColumns from "@/layouts/LayoutColumns/index.vue";
import LayoutClassic from "@/layouts/LayoutClassic/index.vue";
import LayoutHorizontal from "@/layouts/LayoutHorizontal/index.vue";
import LayoutOptimum from "@/layouts/LayoutOptimum/index.vue";
import LayoutMobile from "@/layouts/LayoutMobile/index.vue";

type LayoutType = "vertical" | "columns" | "classic" | "horizontal" | "optimum" | string;
const LayoutComponent: Record<LayoutType, Component> = {
  vertical: LayoutVertical,
  columns: LayoutColumns,
  classic: LayoutClassic,
  horizontal: LayoutHorizontal,
  optimum: LayoutOptimum,
};

const globalStore = useGlobalStore();
const layout = computed(() => globalStore.layout as LayoutType);
const { isMobile } = useScreenStore();

const activeLayoutComponent = computed(() => {
  if (isMobile.value) return LayoutMobile;
  return LayoutComponent[layout.value] ?? LayoutVertical;
});
</script>

<style scoped lang="scss">
.layout-app {
  width: 100%;
  height: 100%;
}
</style>
