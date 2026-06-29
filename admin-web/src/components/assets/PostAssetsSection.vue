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
        <div v-if="coverRows.length" ref="coverGridRef" class="cover-grid">
          <div v-for="item in coverRows" :key="item.asset.id" class="cover-grid__item">
            <div class="cover-grid__card">
              <el-icon class="cover-drag-handle" :title="t('menu.content.post.manage.coverDrag')">
                <Rank />
              </el-icon>
              <el-image
                :src="resolveAssetUrl(item.asset.url)"
                fit="cover"
                class="cover-grid__img"
                :class="{ 'is-pending': item.pending }"
                :preview-src-list="coverPreviewList"
              />
            </div>
            <el-button type="danger" link class="cover-grid__remove" @click="removeCover(item)">
              {{ t("button.delete") }}
            </el-button>
          </div>
        </div>
        <el-button v-if="canPickCover" type="primary" @click="openCoverPicker">
          {{ t("menu.content.post.manage.selectCover") }}
        </el-button>
        <p v-else-if="postId || coverRows.length" class="post-assets-section__hint-text post-assets-section__limit">
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
        <div class="attachment-table-wrap">
          <el-table
            v-if="attachmentRows.length"
            ref="attachmentTableRef"
            :data="attachmentRows"
            size="small"
            class="attachment-table"
            :row-key="attachmentRowKey"
            @selection-change="onAttachmentSelectionChange"
          >
            <el-table-column width="40" align="center" class-name="attachment-drag-col">
              <template #default>
                <el-icon class="attachment-drag-handle" :title="t('menu.content.post.manage.attachmentsDrag')">
                  <Rank />
                </el-icon>
              </template>
            </el-table-column>
            <el-table-column type="selection" width="42" align="center" />
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
          <div class="attachment-table__footer">
            <div v-if="selectedAttachments.length" class="attachment-table__toolbar">
              <span class="attachment-table__selected-count">
                {{ t("menu.content.post.manage.attachmentsSelectedCount", { count: selectedAttachments.length }) }}
              </span>
              <el-button type="danger" size="small" @click="removeSelectedAttachments">
                {{ t("button.delete") }}
              </el-button>
            </div>
            <p class="post-assets-section__hint-text attachment-table__hint">
              {{ t("menu.content.post.manage.attachmentsHint") }}
            </p>
          </div>
        </div>
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
      multiple
      :excluded-ids="attachmentExcludedIds"
      :title="t('menu.content.post.manage.pickerAttachmentTitle')"
      @select="onAttachmentPicked"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessageBox, type TableInstance } from "element-plus";
import { Rank } from "@element-plus/icons-vue";
import Sortable from "sortablejs";
import AssetFileIcon from "@/components/assets/AssetFileIcon.vue";
import AssetPickerDialog from "@/components/assets/AssetPickerDialog.vue";
import {
  assetDisplayName,
  formatFileSize,
  linkPostAssetApi,
  listPostAssetsApi,
  reorderPostAttachmentsApi,
  reorderPostCoversApi,
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
const coverRows = ref<LinkedAssetRow[]>(
  (props.initialCovers ?? []).map((asset) => ({ asset, pending: false })),
);
const attachmentRows = ref<LinkedAssetRow[]>(
  (props.initialAttachments ?? []).map((asset) => ({ asset, pending: false })),
);
const coverMax = ref(3);
const coverPickerVisible = ref(false);
const attachmentPickerVisible = ref(false);
const coverGridRef = ref<HTMLElement | null>(null);
const attachmentTableRef = ref<TableInstance | null>(null);
const selectedAttachments = ref<LinkedAssetRow[]>([]);
let coverSortable: Sortable | null = null;
let attachmentSortable: Sortable | null = null;

const hasPending = computed(
  () => coverRows.value.some((row) => row.pending) || attachmentRows.value.some((row) => row.pending),
);

const coverPreviewList = computed(() =>
  coverRows.value.map((item) => resolveAssetUrl(item.asset.url)),
);

const canPickCover = computed(() => coverRows.value.length < coverMax.value);

const attachmentExcludedIds = computed(() => attachmentRows.value.map((row) => row.asset.id));

watch(
  () => [props.initialCovers, props.initialAttachments] as const,
  ([list, attachmentList]) => {
    coverRows.value = (list ?? []).map((asset) => ({ asset, pending: false }));
    attachmentRows.value = (attachmentList ?? []).map((asset) => ({ asset, pending: false }));
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

watch(
  () => coverRows.value.length,
  () => {
    nextTick(initCoverSortable);
  },
);

watch(
  () => attachmentRows.value.length,
  () => {
    nextTick(initAttachmentSortable);
  },
);

onBeforeUnmount(() => {
  coverSortable?.destroy();
  coverSortable = null;
  attachmentSortable?.destroy();
  attachmentSortable = null;
});

onMounted(() => {
  nextTick(() => {
    initCoverSortable();
    initAttachmentSortable();
  });
});

function initCoverSortable() {
  coverSortable?.destroy();
  coverSortable = null;

  const grid = coverGridRef.value;
  if (!grid || coverRows.value.length < 2) return;

  coverSortable = Sortable.create(grid, {
    handle: ".cover-drag-handle",
    draggable: ".cover-grid__item",
    animation: 150,
    onEnd: (evt) => {
      const { oldIndex, newIndex } = evt;
      if (oldIndex == null || newIndex == null || oldIndex === newIndex) return;
      void onCoverReorder(oldIndex, newIndex);
    },
  });
}

function initAttachmentSortable() {
  attachmentSortable?.destroy();
  attachmentSortable = null;

  const table = attachmentTableRef.value;
  if (!table || attachmentRows.value.length < 2) return;

  const tbody = table.$el.querySelector(".el-table__body-wrapper tbody") as HTMLElement | null;
  if (!tbody) return;

  attachmentSortable = Sortable.create(tbody, {
    handle: ".attachment-drag-handle",
    animation: 150,
    onEnd: (evt) => {
      const { oldIndex, newIndex } = evt;
      if (oldIndex == null || newIndex == null || oldIndex === newIndex) return;
      void onAttachmentReorder(oldIndex, newIndex);
    },
  });
}

function isAssetUsed(assetId: number): boolean {
  return (
    coverRows.value.some((row) => row.asset.id === assetId)
    || attachmentRows.value.some((row) => row.asset.id === assetId)
  );
}

function linkedCoverIds(): number[] {
  return coverRows.value.filter((row) => !row.pending).map((row) => row.asset.id);
}

function linkedAttachmentIds(): number[] {
  return attachmentRows.value.filter((row) => !row.pending).map((row) => row.asset.id);
}

async function persistCoverOrder(): Promise<boolean> {
  const postId = props.postId;
  const assetIds = linkedCoverIds();
  if (postId == null || assetIds.length === 0) return true;

  const res = await reorderPostCoversApi(postId, { asset_ids: assetIds });
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    await reload();
    return false;
  }
  return true;
}

async function persistAttachmentOrder(): Promise<boolean> {
  const postId = props.postId;
  const assetIds = linkedAttachmentIds();
  if (postId == null || assetIds.length === 0) return true;

  const res = await reorderPostAttachmentsApi(postId, { asset_ids: assetIds });
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    await reload();
    return false;
  }
  return true;
}

async function onCoverReorder(oldIndex: number, newIndex: number) {
  const rows = coverRows.value.slice();
  const [moved] = rows.splice(oldIndex, 1);
  rows.splice(newIndex, 0, moved);
  coverRows.value = rows;

  if (props.postId != null && linkedCoverIds().length > 0) {
    await persistCoverOrder();
  }
}

async function onAttachmentReorder(oldIndex: number, newIndex: number) {
  const rows = attachmentRows.value.slice();
  const [moved] = rows.splice(oldIndex, 1);
  rows.splice(newIndex, 0, moved);
  attachmentRows.value = rows;

  if (props.postId != null && linkedAttachmentIds().length > 0) {
    await persistAttachmentOrder();
  }
}

function openCoverPicker() {
  coverPickerVisible.value = true;
}

function openAttachmentPicker() {
  attachmentPickerVisible.value = true;
}

function attachmentRowKey(row: LinkedAssetRow) {
  return row.asset.id;
}

function onAttachmentSelectionChange(rows: LinkedAssetRow[]) {
  selectedAttachments.value = rows;
}

function clearAttachmentSelection() {
  attachmentTableRef.value?.clearSelection();
  selectedAttachments.value = [];
}

async function linkCover(asset: AssetView, postId: number, silent = false): Promise<boolean> {
  const res = await linkPostAssetApi(postId, { asset_id: asset.id, role: "cover" });
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    return false;
  }
  if (!coverRows.value.some((row) => row.asset.id === asset.id)) {
    coverRows.value = [...coverRows.value, { asset, pending: false }];
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
  if (!attachmentRows.value.some((row) => row.asset.id === asset.id)) {
    attachmentRows.value = [...attachmentRows.value, { asset, pending: false }];
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
    coverRows.value = [...coverRows.value, { asset, pending: true }];
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
    attachmentRows.value = [...attachmentRows.value, { asset, pending: true }];
    koiMsgSuccess(t("menu.content.post.manage.pendingAssetSaved"));
    return;
  }
  await linkAttachment(asset, props.postId);
}

async function reload() {
  if (props.postId == null) return;
  const res = await listPostAssetsApi(props.postId);
  if (res.code === 0 && res.data) {
    coverRows.value = res.data.covers.map((asset) => ({ asset, pending: false }));
    attachmentRows.value = res.data.attachments.map((asset) => ({ asset, pending: false }));
    coverMax.value = res.data.cover_max;
    nextTick(() => {
      initCoverSortable();
      initAttachmentSortable();
    });
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
    coverRows.value = coverRows.value.filter((item) => item.asset.id !== row.asset.id);
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
  coverRows.value = coverRows.value.filter((item) => item.asset.id !== row.asset.id);
  koiMsgSuccess(t("msg.success"));
}

async function removeAttachment(row: LinkedAssetRow) {
  await removeAttachments([row]);
}

async function removeSelectedAttachments() {
  await removeAttachments(selectedAttachments.value);
}

function buildAttachmentRemoveConfirm(linkedRows: LinkedAssetRow[]): string {
  if (linkedRows.length === 1) {
    return t("menu.content.post.manage.removeAttachmentConfirm", {
      name: assetDisplayName(linkedRows[0].asset),
    });
  }
  return t("menu.content.post.manage.removeAttachmentsConfirm", { count: linkedRows.length });
}

async function removeAttachments(rows: LinkedAssetRow[]) {
  if (rows.length === 0) return;

  const pendingRows = rows.filter((row) => row.pending);
  const linkedRows = rows.filter((row) => !row.pending);

  let purge: boolean | null = null;
  if (linkedRows.length > 0) {
    if (!props.postId) return;
    purge = await confirmPurge(buildAttachmentRemoveConfirm(linkedRows));
    if (purge === null) return;
  }

  if (pendingRows.length > 0) {
    const pendingIds = new Set(pendingRows.map((row) => row.asset.id));
    attachmentRows.value = attachmentRows.value.filter(
      (row) => !(row.pending && pendingIds.has(row.asset.id)),
    );
  }

  if (linkedRows.length > 0 && props.postId) {
    const postId = props.postId;
    for (const row of linkedRows) {
      const res = await unlinkPostAssetApi(postId, row.asset.id, purge!);
      if (res.code !== 0) {
        koiMsgError(res.message || t("msg.fail"));
        await reload();
        clearAttachmentSelection();
        return;
      }
    }
    const linkedIds = new Set(linkedRows.map((row) => row.asset.id));
    attachmentRows.value = attachmentRows.value.filter((row) => !linkedIds.has(row.asset.id));
    koiMsgSuccess(t("msg.success"));
  }

  clearAttachmentSelection();
}

async function ensurePendingLinked(): Promise<boolean> {
  const postId = props.postId;
  if (postId == null) {
    return !hasPending.value;
  }

  for (let i = 0; i < coverRows.value.length; i += 1) {
    const row = coverRows.value[i];
    if (!row.pending) continue;
    const res = await linkPostAssetApi(postId, {
      asset_id: row.asset.id,
      role: "cover",
      sort_order: i,
    });
    if (res.code !== 0) {
      koiMsgError(res.message || t("msg.fail"));
      return false;
    }
    row.pending = false;
  }

  if (linkedCoverIds().length > 0) {
    const coverOk = await persistCoverOrder();
    if (!coverOk) return false;
  }

  for (let i = 0; i < attachmentRows.value.length; i += 1) {
    const row = attachmentRows.value[i];
    if (!row.pending) continue;
    const res = await linkPostAssetApi(postId, {
      asset_id: row.asset.id,
      role: "attachment",
      sort_order: i,
    });
    if (res.code !== 0) {
      koiMsgError(res.message || t("msg.fail"));
      return false;
    }
    row.pending = false;
  }

  if (linkedAttachmentIds().length > 0) {
    const ok = await persistAttachmentOrder();
    if (!ok) return false;
  }

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

.cover-grid__card {
  position: relative;
}

.cover-drag-handle {
  position: absolute;
  top: 4px;
  left: 4px;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border-radius: 4px;
  background: rgb(0 0 0 / 45%);
  color: #fff;
  font-size: 14px;
  cursor: grab;

  &:active {
    cursor: grabbing;
  }
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

.attachment-table-wrap {
  margin-top: 12px;
  width: 100%;
}

.attachment-table__footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-height: 35px;
  margin-top: 8px;
}

.attachment-table__toolbar {
  display: flex;
  align-items: center;
  flex-shrink: 0;
  gap: 12px;
}

.attachment-table__hint {
  margin: 0;
  flex: 1;
  min-width: 0;
  text-align: right;
}

.attachment-table__selected-count {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.attachment-table {
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

.attachment-drag-handle {
  cursor: grab;
  color: var(--el-text-color-secondary);
  font-size: 16px;

  &:active {
    cursor: grabbing;
  }
}

:deep(.attachment-drag-col .cell) {
  padding: 0;
}
</style>
