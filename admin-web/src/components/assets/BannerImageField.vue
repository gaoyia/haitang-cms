<template>
  <div class="banner-image-field">
    <el-alert
      v-if="bannerId == null"
      type="info"
      :closable="false"
      show-icon
      :title="t('menu.banner.assetsNeedSave')"
      class="banner-image-field__hint"
    />
    <el-alert
      v-else-if="pendingAsset"
      type="warning"
      :closable="false"
      show-icon
      :title="t('menu.banner.pendingAssetHint')"
      class="banner-image-field__hint"
    />

    <div v-if="displayImage" class="banner-image-field__preview">
      <el-image
        :src="resolveAssetUrl(displayImage.url)"
        fit="cover"
        class="banner-image-field__img"
        :class="{ 'is-disabled': bannerId != null && !imageEnabled }"
        :preview-src-list="[resolveAssetUrl(displayImage.url)]"
      />
      <div class="banner-image-field__actions">
        <div v-if="bannerId != null" class="banner-image-field__switch">
          <span>{{ t("menu.banner.imageEnabled") }}</span>
          <el-switch
            v-model="imageEnabled"
            :disabled="toggling"
            @change="onToggleEnabled"
          />
        </div>
        <div class="banner-image-field__buttons">
          <el-button type="primary" link @click="openPicker">
            {{ t("menu.assets.pickerReplace") }}
          </el-button>
          <el-button type="danger" link @click="removeImage">
            {{ t("button.delete") }}
          </el-button>
        </div>
      </div>
    </div>

    <div v-else class="banner-image-field__empty">
      <el-button type="primary" @click="openPicker">
        {{ t("menu.assets.pickerOpen") }}
      </el-button>
    </div>

    <p class="banner-image-field__hint-text">{{ t("menu.banner.imageHint") }}</p>

    <AssetPickerDialog
      v-model="pickerVisible"
      purpose="banner"
      :title="t('menu.banner.pickerTitle')"
      @select="onPicked"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessageBox } from "element-plus";
import AssetPickerDialog from "@/components/assets/AssetPickerDialog.vue";
import {
  linkBannerAssetApi,
  listBannerAssetsApi,
  setBannerImageEnabledApi,
  unlinkBannerAssetApi,
  type AssetView,
} from "@/api/system/assets.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

const props = defineProps<{
  bannerId: number | null;
  initialImage?: AssetView | null;
  initialImageEnabled?: boolean;
}>();

const emit = defineEmits<{
  changed: [];
}>();

const { t } = useI18n();
const image = ref<AssetView | null>(props.initialImage ?? null);
const imageEnabled = ref(props.initialImageEnabled ?? true);
const pendingAsset = ref<AssetView | null>(null);
const pickerVisible = ref(false);
const toggling = ref(false);

const displayImage = computed(() => image.value ?? pendingAsset.value);

watch(
  () => [props.initialImage, props.initialImageEnabled] as const,
  ([value, enabled]) => {
    image.value = value ?? null;
    if (enabled !== undefined) {
      imageEnabled.value = enabled;
    }
  },
);

watch(
  () => props.bannerId,
  async (id, prev) => {
    if (id == null) {
      return;
    }
    if (prev == null || image.value == null) {
      await reload();
    }
  },
);

async function reload() {
  if (props.bannerId == null) return;
  const res = await listBannerAssetsApi(props.bannerId);
  if (res.code === 0 && res.data) {
    image.value = res.data.image;
    imageEnabled.value = res.data.image_enabled;
  }
}

function openPicker() {
  pickerVisible.value = true;
}

async function linkAsset(asset: AssetView, silent = false): Promise<boolean> {
  if (props.bannerId == null) return false;
  const res = await linkBannerAssetApi(props.bannerId, {
    asset_id: asset.id,
    role: "image",
    enabled: true,
  });
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    return false;
  }
  image.value = asset;
  imageEnabled.value = true;
  if (!silent) {
    koiMsgSuccess(t("msg.success"));
  }
  emit("changed");
  return true;
}

async function onPicked(asset: AssetView) {
  if (props.bannerId == null) {
    pendingAsset.value = asset;
    image.value = asset;
    koiMsgSuccess(t("menu.banner.pendingAssetSaved"));
    emit("changed");
    return;
  }
  await linkAsset(asset);
}

async function onToggleEnabled(value: boolean) {
  if (props.bannerId == null || !image.value) return;
  toggling.value = true;
  const prev = !value;
  try {
    const res = await setBannerImageEnabledApi(props.bannerId, value);
    if (res.code !== 0) {
      imageEnabled.value = prev;
      koiMsgError(res.message || t("msg.fail"));
      return;
    }
    koiMsgSuccess(t("msg.success"));
    emit("changed");
  } finally {
    toggling.value = false;
  }
}

async function removeImage() {
  if (pendingAsset.value && props.bannerId == null) {
    pendingAsset.value = null;
    image.value = null;
    emit("changed");
    return;
  }
  if (props.bannerId == null || !image.value) return;
  let purge = false;
  try {
    await ElMessageBox.confirm(t("menu.banner.removeImageConfirm"), t("button.delete"), {
      distinguishCancelAndClose: true,
      confirmButtonText: t("menu.banner.removeAndPurge"),
      cancelButtonText: t("menu.banner.removeOnly"),
      type: "warning",
    });
    purge = true;
  } catch (action) {
    if (action === "cancel") {
      purge = false;
    } else {
      return;
    }
  }
  const res = await unlinkBannerAssetApi(props.bannerId, image.value.id, purge);
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    return;
  }
  image.value = null;
  pendingAsset.value = null;
  imageEnabled.value = true;
  koiMsgSuccess(t("msg.success"));
  emit("changed");
}

async function ensurePendingLinked(): Promise<boolean> {
  if (pendingAsset.value && props.bannerId != null) {
    const ok = await linkAsset(pendingAsset.value, true);
    if (ok) {
      pendingAsset.value = null;
    }
    return ok;
  }
  return image.value != null;
}

defineExpose({
  hasImage: () => image.value != null || pendingAsset.value != null,
  ensurePendingLinked,
});
</script>

<style scoped lang="scss">
.banner-image-field__hint {
  margin-bottom: 12px;
}

.banner-image-field__preview {
  display: flex;
  align-items: flex-start;
  gap: 16px;
}

.banner-image-field__img {
  width: 160px;
  height: 90px;
  border-radius: 6px;
  border: 1px solid var(--el-border-color-lighter);

  &.is-disabled {
    opacity: 0.45;
    filter: grayscale(0.6);
  }
}

.banner-image-field__actions {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.banner-image-field__switch {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--el-text-color-regular);
}

.banner-image-field__buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 12px;
}

.banner-image-field__empty {
  margin-bottom: 4px;
}

.banner-image-field__hint-text {
  margin: 6px 0 0;
  font-size: 12px;
  line-height: 1.5;
  color: var(--el-text-color-secondary);
}
</style>
