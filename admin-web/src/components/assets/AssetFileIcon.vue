<template>
  <span
    class="asset-file-icon"
    :class="{ 'asset-file-icon--block': block }"
    :title="titleText"
  >
    <img
      class="asset-file-icon__img"
      :src="iconUrl"
      :width="size"
      :height="size"
      alt=""
      loading="lazy"
    />
  </span>
</template>

<script setup lang="ts">
import { computed } from "vue";
import {
  assetFileIconUrl,
  resolveAssetFileIconSlug,
} from "@/utils/assetFileIcon.ts";

const props = withDefaults(
  defineProps<{
    mimeType: string;
    filename: string;
    size?: number | string;
    /** 占满父容器并居中（用于表格预览格） */
    block?: boolean;
    showTitle?: boolean;
  }>(),
  {
    size: 32,
    block: false,
    showTitle: true,
  },
);

const iconUrl = computed(() =>
  assetFileIconUrl(props.mimeType, props.filename),
);

const titleText = computed(() => {
  if (!props.showTitle) return undefined;
  const slug = resolveAssetFileIconSlug(props.mimeType, props.filename);
  return slug.toUpperCase();
});
</script>

<style scoped lang="scss">
.asset-file-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  vertical-align: middle;
  line-height: 0;
}

.asset-file-icon--block {
  width: 100%;
  height: 100%;
  min-height: 48px;
}

.asset-file-icon__img {
  display: block;
  object-fit: contain;
}
</style>
