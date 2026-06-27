<template>
  <div class="post-assets-section">
    <el-form-item
      v-if="!postId && hasPending"
      label=" "
      class="post-assets-section__hint-item"
    >
      <el-alert
        type="warning"
        :closable="false"
        show-icon
        :title="t('menu.content.post.manage.pendingAssetsHint')"
      />
    </el-form-item>
    <el-form-item
      v-else-if="!postId"
      label=" "
      class="post-assets-section__hint-item"
    >
      <el-alert
        type="info"
        :closable="false"
        show-icon
        :title="t('menu.content.post.manage.assetsNeedSave')"
      />
    </el-form-item>

    <el-form-item :label="t('menu.content.post.manage.cover')">
      <div class="post-assets-section__field">
        <div v-if="displayCovers.length" class="cover-grid">
          <div v-for="item in displayCovers" :key="item.asset.id" class="cover-grid__item">
            <el-image
              :src="resolveAssetUrl(item.asset.url)"
              fit="cover"
              class="cover-grid__img"
              :class="{ 'is-pending': item.pending }"
              :preview-src-list="coverPreviewList"
            />
            <el-button type="danger" link class="cover-grid__remove" @click="removeCover(item)">
              {{ t("button.delete") }}
            </el-button>
          </div>
        </div>
        <el-button v-if="canPickCover" type="primary" @click="openCoverPicker">
          {{ t("menu.content.post.manage.selectCover") }}
        </el-button>
        <p v-else-if="postId || displayCovers.length" class="post-assets-section__hint-text post-assets-section__limit">
          {{ t("menu.content.post.manage.coverLimitReached", { max: coverMax }) }}
        </p>
        <p class="post-assets-section__hint-text">
          {{ t("menu.content.post.manage.coverHint", { max: coverMax }) }}
        </p>
      </div>
    </el-form-item>

    <el-form-item :label="t('menu.content.post.manage.attachments')">
      <div class="post-assets-section__field">
        <el-button type="primary" @click="openAttachmentPicker">
          {{ t("menu.content.post.manage.selectAttachment") }}
        </el-button>
        <el-table v-if="displayAttachments.length" :data="displayAttachments" size="small" class="attachment-table">
          <el-table-column :label="t('menu.assets.preview')" width="88" align="center">
            <template #default="{ row }">
              <el-image
                v-if="row.asset.mime_type.startsWith('image/')"
                :src="resolveAssetUrl(row.asset.url)"
                fit="cover"
                class="attachment-table__thumb"
                :class="{ 'is-pending': row.pending }"
                :preview-src-list="[resolveAssetUrl(row.asset.url)]"
              />
              <video
                v-else-if="row.asset.mime_type.startsWith('video/')"
                :src="resolveAssetUrl(row.asset.url)"
                class="attachment-table__thumb"
                controls
                preload="metadata"
              />
              <AssetFileIcon
                v-else
                block
                :mime-type="row.asset.mime_type"
                :filename="assetDisplayName(row.asset)"
                :size="36"
              />
            </template>
          </el-table-column>
          <el-table-column :label="t('menu.assets.uploadName')" min-width="160">
            <template #default="{ row }">{{ assetDisplayName(row.asset) }}</template>
          </el-table-column>
          <el-table-column :label="t('menu.assets.size')" width="100">
            <template #default="{ row }">{{ formatFileSize(row.asset.size) }}</template>
          </el-table-column>
          <el-table-column :label="t('table.operate')" width="100">
            <template #default="{ row }">
              <el-button type="danger" link @click="removeAttachment(row)">{{ t("button.delete") }}</el-button>
            </template>
          </el-table-column>
        </el-table>
        <p class="post-assets-section__hint-text">{{ t("menu.content.post.manage.attachmentsHint") }}</p>
      </div>
    </el-form-item>

    <AssetPickerDialog
      v-model="coverPickerVisible"
      purpose="cover"
      :title="t('menu.content.post.manage.pickerCoverTitle')"
      @select="onCoverPicked"
    />
    <AssetPickerDialog
      v-model="attachmentPickerVisible"
      purpose="attachment"
      :title="t('menu.content.post.manage.pickerAttachmentTitle')"
      @select="onAttachmentPicked"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessageBox } from "element-plus";
import AssetFileIcon from "@/components/assets/AssetFileIcon.vue";
import AssetPickerDialog from "@/components/assets/AssetPickerDialog.vue";
import {
  assetDisplayName,
  formatFileSize,
  linkPostAssetApi,
  listPostAssetsApi,
  unlinkPostAssetApi,
  type AssetView,
} from "@/api/system/assets.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

interface LinkedAssetRow {
  asset: AssetView;
  pending: boolean;
}

const props = defineProps<{
  postId: number | null;
  initialCovers?: AssetView[];
  initialAttachments?: AssetView[];
}>();

const { t } = useI18n();
const covers = ref<AssetView[]>(props.initialCovers ?? []);
const attachments = ref<AssetView[]>(props.initialAttachments ?? []);
const pendingCovers = ref<AssetView[]>([]);
const pendingAttachments = ref<AssetView[]>([]);
const coverMax = ref(3);
const coverPickerVisible = ref(false);
const attachmentPickerVisible = ref(false);

const hasPending = computed(
  () => pendingCovers.value.length > 0 || pendingAttachments.value.length > 0,
);

const displayCovers = computed<LinkedAssetRow[]>(() => [
  ...covers.value.map((asset) => ({ asset, pending: false })),
  ...pendingCovers.value.map((asset) => ({ asset, pending: true })),
]);

const displayAttachments = computed<LinkedAssetRow[]>(() => [
  ...attachments.value.map((asset) => ({ asset, pending: false })),
  ...pendingAttachments.value.map((asset) => ({ asset, pending: true })),
]);

const coverPreviewList = computed(() =>
  displayCovers.value.map((item) => resolveAssetUrl(item.asset.url)),
);

const canPickCover = computed(() => displayCovers.value.length < coverMax.value);

watch(
  () => [props.initialCovers, props.initialAttachments] as const,
  ([list, attachmentList]) => {
    covers.value = list ?? [];
    attachments.value = attachmentList ?? [];
  },
);

watch(
  () => props.postId,
  async (id, prev) => {
    if (id == null) return;
    if (prev == null && hasPending.value) {
      await ensurePendingLinked();
      return;
    }
    if (prev == null) {
      await reload();
    }
  },
  { immediate: true },
);

function isAssetUsed(assetId: number): boolean {
  return (
    covers.value.some((a) => a.id === assetId)
    || pendingCovers.value.some((a) => a.id === assetId)
    || attachments.value.some((a) => a.id === assetId)
    || pendingAttachments.value.some((a) => a.id === assetId)
  );
}

function openCoverPicker() {
  coverPickerVisible.value = true;
}

function openAttachmentPicker() {
  attachmentPickerVisible.value = true;
}

async function linkCover(asset: AssetView, postId: number, silent = false): Promise<boolean> {
  const res = await linkPostAssetApi(postId, { asset_id: asset.id, role: "cover" });
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    return false;
  }
  if (!covers.value.some((item) => item.id === asset.id)) {
    covers.value = [...covers.value, asset];
  }
  if (!silent) {
    koiMsgSuccess(t("msg.success"));
  }
  return true;
}

async function linkAttachment(asset: AssetView, postId: number, silent = false): Promise<boolean> {
  const res = await linkPostAssetApi(postId, { asset_id: asset.id, role: "attachment" });
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    return false;
  }
  if (!attachments.value.some((item) => item.id === asset.id)) {
    attachments.value = [...attachments.value, asset];
  }
  if (!silent) {
    koiMsgSuccess(t("msg.success"));
  }
  return true;
}

async function onCoverPicked(asset: AssetView) {
  if (isAssetUsed(asset.id)) {
    koiMsgError(t("menu.content.post.manage.assetAlreadyLinked"));
    return;
  }
  if (!canPickCover.value) {
    koiMsgError(t("menu.content.post.manage.coverLimitReached", { max: coverMax.value }));
    return;
  }
  if (props.postId == null) {
    pendingCovers.value = [...pendingCovers.value, asset];
    koiMsgSuccess(t("menu.content.post.manage.pendingAssetSaved"));
    return;
  }
  await linkCover(asset, props.postId);
}

async function onAttachmentPicked(asset: AssetView) {
  if (isAssetUsed(asset.id)) {
    koiMsgError(t("menu.content.post.manage.assetAlreadyLinked"));
    return;
  }
  if (props.postId == null) {
    pendingAttachments.value = [...pendingAttachments.value, asset];
    koiMsgSuccess(t("menu.content.post.manage.pendingAssetSaved"));
    return;
  }
  await linkAttachment(asset, props.postId);
}

async function reload() {
  if (props.postId == null) return;
  const res = await listPostAssetsApi(props.postId);
  if (res.code === 0 && res.data) {
    covers.value = res.data.covers;
    attachments.value = res.data.attachments;
    coverMax.value = res.data.cover_max;
  }
}

async function confirmPurge(title: string) {
  try {
    await ElMessageBox.confirm(title, t("button.delete"), {
      distinguishCancelAndClose: true,
      confirmButtonText: t("menu.content.post.manage.removeAndPurge"),
      cancelButtonText: t("menu.content.post.manage.removeOnly"),
      type: "warning",
    });
    return true;
  } catch (action) {
    if (action === "cancel") return false;
    return null;
  }
}

async function removeCover(row: LinkedAssetRow) {
  if (row.pending) {
    pendingCovers.value = pendingCovers.value.filter((item) => item.id !== row.asset.id);
    return;
  }
  if (!props.postId) return;
  const purge = await confirmPurge(t("menu.content.post.manage.removeCoverConfirm"));
  if (purge === null) return;
  const res = await unlinkPostAssetApi(props.postId, row.asset.id, purge);
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    return;
  }
  covers.value = covers.value.filter((item) => item.id !== row.asset.id);
  koiMsgSuccess(t("msg.success"));
}

async function removeAttachment(row: LinkedAssetRow) {
  if (row.pending) {
    pendingAttachments.value = pendingAttachments.value.filter((item) => item.id !== row.asset.id);
    return;
  }
  if (!props.postId) return;
  const purge = await confirmPurge(
    t("menu.content.post.manage.removeAttachmentConfirm", { name: assetDisplayName(row.asset) }),
  );
  if (purge === null) return;
  const res = await unlinkPostAssetApi(props.postId, row.asset.id, purge);
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    return;
  }
  attachments.value = attachments.value.filter((a) => a.id !== row.asset.id);
  koiMsgSuccess(t("msg.success"));
}

async function ensurePendingLinked(): Promise<boolean> {
  const postId = props.postId;
  if (postId == null) {
    return !hasPending.value;
  }
  for (const asset of pendingCovers.value) {
    const ok = await linkCover(asset, postId, true);
    if (!ok) return false;
  }
  for (const asset of pendingAttachments.value) {
    const ok = await linkAttachment(asset, postId, true);
    if (!ok) return false;
  }
  pendingCovers.value = [];
  pendingAttachments.value = [];
  await reload();
  return true;
}

defineExpose({ reload, ensurePendingLinked, hasPendingAssets: () => hasPending.value });
</script>

<style scoped lang="scss">
.post-assets-section__hint-item {
  margin-bottom: 4px;

  :deep(.el-form-item__label) {
    visibility: hidden;
  }

  :deep(.el-alert) {
    width: 100%;
  }
}

.post-assets-section__field {
  width: 100%;
}

.post-assets-section__hint-text {
  margin: 6px 0 0;
  font-size: 12px;
  line-height: 1.5;
  color: var(--el-text-color-secondary);
}

.post-assets-section__limit {
  color: var(--el-color-warning);
}

.cover-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 12px;
}

.cover-grid__item {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 4px;
}

.cover-grid__img {
  width: 120px;
  height: 80px;
  border-radius: 6px;
  border: 1px solid var(--el-border-color-lighter);

  &.is-pending {
    opacity: 0.75;
    border-style: dashed;
  }
}

.cover-grid__remove {
  padding-left: 0;
}

.attachment-table {
  margin-top: 12px;
  width: 100%;
}

.attachment-table__thumb {
  width: 64px;
  height: 48px;
  border-radius: 4px;
  object-fit: cover;
  vertical-align: middle;

  &.is-pending {
    opacity: 0.75;
    border: 1px dashed var(--el-border-color);
  }
}
</style>
