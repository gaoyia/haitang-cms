<template>
  <div class="main-wrapper">
    <Maximize />
    <div class="main-frame">
      <Tabs></Tabs>
      <el-main class="main-content">
        <div class="main-content__inner">
          <router-view v-slot="{ Component, route }">
            <transition :name="transition" mode="out-in" appear>
              <keep-alive :max="16" :include="keepAliveStore.keepAliveName">
                <component :is="Component" :key="route.fullPath" v-if="isRouterShow" />
              </keep-alive>
            </transition>
          </router-view>
        </div>
      </el-main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, provide, onBeforeUnmount } from "vue";
import Maximize from "@/layouts/components/Main/components/Maximize.vue";
import { useDebounceFn } from "@vueuse/core";
import Tabs from "@/layouts/components/Tabs/index.vue";
import { storeToRefs } from "pinia";
import useKeepAliveStore from "@/stores/modules/keepAlive.ts";
import useGlobalStore from "@/stores/modules/global.ts";

const globalStore = useGlobalStore();

const { transition } = storeToRefs(globalStore);

const keepAliveStore = useKeepAliveStore();

const isRouterShow = ref(true);

const refreshCurrentPage = (val: boolean) => (isRouterShow.value = val);

provide("refresh", refreshCurrentPage);

watch(
  () => globalStore.maximize,
  () => {
    const app = document.getElementById("app") as HTMLElement;
    if (globalStore.maximize) app.classList.add("main-maximize");
    else app.classList.remove("main-maximize");
    window.dispatchEvent(new Event("resize"));
  },
  { deep: true, immediate: true }
);

const screenWidth = ref(0);
const showTabs = ref(true);

const listeningWindow = useDebounceFn(() => {
  screenWidth.value = document.body.clientWidth;
  if (!globalStore.isCollapse && screenWidth.value < 1200) globalStore.setGlobalState("isCollapse", true);
  if (globalStore.isCollapse && screenWidth.value > 1200) globalStore.setGlobalState("isCollapse", false);
  showTabs.value = screenWidth.value >= 520;
}, 100);

window.addEventListener("resize", listeningWindow, false);

onBeforeUnmount(() => {
  window.removeEventListener("resize", listeningWindow);
});
</script>

<style lang="scss" scoped>
@use "../../../styles/transition.scss";

.main-wrapper {
  --layout-canvas-bg: #{$main-canvas-bg};
  --layout-tabs-bg: #{$main-tabs-bg};
  --layout-main-bg: #{$main-surface-bg};
  --layout-main-radius: #{$main-content-radius};
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  box-sizing: border-box;
  padding: $main-area-inset;
  background-color: var(--layout-canvas-bg);

  html.dark & {
    --layout-canvas-bg: #0a0a0a;
    --layout-tabs-bg: #141414;
    --layout-main-bg: var(--el-bg-color);
  }
}

/* 标签栏 + 内容区贴紧：激活标签底边与内容面板顶边无缝衔接 */
.main-frame {
  display: flex;
  flex: 1;
  flex-direction: column;
  gap: 0;
  min-height: 0;
}

.main-frame :deep(.layout-tabs-bar) {
  flex-shrink: 0;
  position: relative;
  z-index: 2;
  margin: 0;
  padding: 0;
  border: none;
  border-radius: 0;
  background-color: transparent;
}

.main-frame :deep(.layout-tabs--google .layout-tabs-bar__item.is-active) {
  z-index: 3;
  border-radius: var(--layout-main-radius) var(--layout-main-radius) 0 0;
}

/* 勿给 main-content 设 z-index，否则会形成层叠上下文，导致内部 fixed 蒙版无法盖住标签栏 */
.main-content {
  --el-main-padding: 0;
  position: relative;
  overflow: hidden;
  flex: 1;
  min-height: 0;
  margin: 0;
  padding: 0;
  border: none;
  border-radius: var(--layout-main-radius);
  background-color: var(--layout-main-bg);
  box-shadow: 0 2px 8px rgb(15 23 42 / 5%);
}

.main-content__inner {
  box-sizing: border-box;
  height: 100%;
  min-height: 0;
  padding: $main-content-padding;
  overflow-x: hidden;
  overflow-y: auto;
}
</style>
