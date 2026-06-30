<template>
  <div class="friend-link-image-field">
    <div v-if="modelValue" class="friend-link-image-field__preview">
      <el-image
        :src="resolveAssetUrl(modelValue)"
        fit="contain"
        class="friend-link-image-field__img"
        :preview-src-list="[resolveAssetUrl(modelValue)]"
      />
      <div class="friend-link-image-field__actions">
        <el-button type="primary" link @click="openPicker">{{ t("menu.assets.pickerReplace") }}</el-button>
        <el-button type="danger" link @click="clearImage">{{ t("button.delete") }}</el-button>
      </div>
    </div>
    <div v-else class="friend-link-image-field__empty">
      <el-button type="primary" @click="openPicker">{{ t("menu.assets.pickerOpen") }}</el-button>
    </div>
    <p class="friend-link-image-field__hint">{{ t("menu.friendLink.imageHint") }}</p>

    <AssetPickerDialog
      v-model="pickerVisible"
      purpose="friend_link"
      :title="t('menu.friendLink.pickerTitle')"
      @select="onPicked"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import AssetPickerDialog from "@/components/assets/AssetPickerDialog.vue";
import type { AssetView } from "@/api/system/assets.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";

const modelValue = defineModel<string>({ default: "" });

const { t } = useI18n();
const pickerVisible = ref(false);

function openPicker() {
  pickerVisible.value = true;
}

function onPicked(asset: AssetView) {
  modelValue.value = asset.url;
}

function clearImage() {
  modelValue.value = "";
}
</script>

<style scoped lang="scss">
.friend-link-image-field__preview {
  display: flex;
  align-items: flex-start;
  gap: 16px;
}

.friend-link-image-field__img {
  height: 54px;
  max-width: 220px;
  border-radius: 6px;
  border: 1px solid var(--el-border-color-lighter);
  background: var(--el-fill-color-blank);
}

.friend-link-image-field__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 12px;
}

.friend-link-image-field__hint {
  margin: 8px 0 0;
  font-size: 12px;
  line-height: 1.5;
  color: var(--el-text-color-secondary);
}
</style>
