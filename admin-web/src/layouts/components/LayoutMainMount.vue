<template>
  <div ref="mountRef" class="layout-main-mount" />
</template>

<script setup lang="ts">
import { ref, watch, onBeforeUnmount } from "vue";
import { registerLayoutMainMount } from "@/layouts/useLayoutMainMount.ts";

const mountId = Symbol("layout-main-mount");
const mountRef = ref<HTMLElement | null>(null);

watch(
  mountRef,
  (el) => registerLayoutMainMount(mountId, el),
  { immediate: true },
);

onBeforeUnmount(() => registerLayoutMainMount(mountId, null));
</script>

<style scoped lang="scss">
.layout-main-mount {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  min-width: 0;
  overflow: hidden;
}
</style>
