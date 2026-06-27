<template>
  <div class="post-assets-section">
    <el-alert
      v-if="!postId"
      type="info"
      :closable="false"
      show-icon
      :title="t('menu.content.post.manage.assetsNeedSave')"
      class="post-assets-section__hint"
    />

    <el-form-item :label="t('menu.content.post.manage.cover')">
      <div class="post-assets-section__field">
        <div v-if="covers.length" class="cover-grid">
          <div v-for="item in covers" :key="item.id" class="cover-grid__item">
            <el-image
              :src="resolveAssetUrl(item.url)"
              fit="cover"
              class="cover-grid__img"
              :preview-src-list="coverPreviewList"
            />
            <el-button
              type="danger"
              link
              class="cover-grid__remove"
              :disabled="!postId"
              @click="removeCover(item)"
            >
              {{ t("button.delete") }}
            </el-button>
          </div>
        </div>
        <AssetUploader
          v-if="canUploadCover"
          purpose="cover"
          role="cover"
          :post-id="postId"
          accept="image/jpeg,image/png,image/webp,image/gif"
          :disabled="!postId"
          :label="t('menu.content.post.manage.uploadCover')"
          @success="onCoverUploaded"
        />
        <p v-else-if="postId" class="post-assets-section__hint-text post-assets-section__limit">
          {{ t("menu.content.post.manage.coverLimitReached", { max: coverMax }) }}
        </p>
        <p class="post-assets-section__hint-text">
          {{ t("menu.content.post.manage.coverHint", { max: coverMax }) }}
        </p>
      </div>
    </el-form-item>

    <el-form-item :label="t('menu.content.post.manage.attachments')">
      <div class="post-assets-section__field">
        <AssetUploader
          purpose="attachment"
          role="attachment"
          :post-id="postId"
          :accept="attachmentAccept"
          :disabled="!postId"
          :label="t('menu.content.post.manage.uploadAttachment')"
          @success="onAttachmentUploaded"
        />
        <el-table v-if="attachments.length" :data="attachments" size="small" class="attachment-table">
          <el-table-column :label="t('menu.assets.preview')" width="88" align="center">
            <template #default="{ row }">
              <el-image
                v-if="row.mime_type.startsWith('image/')"
                :src="resolveAssetUrl(row.url)"
                fit="cover"
                class="attachment-table__thumb"
                :preview-src-list="[resolveAssetUrl(row.url)]"
              />
              <video
                v-else-if="row.mime_type.startsWith('video/')"
                :src="resolveAssetUrl(row.url)"
                class="attachment-table__thumb"
                controls
                preload="metadata"
              />
              <AssetFileIcon
                v-else
                block
                :mime-type="row.mime_type"
                :filename="assetDisplayName(row)"
                :size="36"
              />
            </template>
          </el-table-column>
          <el-table-column :label="t('menu.assets.uploadName')" min-width="160">
            <template #default="{ row }">{{ assetDisplayName(row) }}</template>
          </el-table-column>
          <el-table-column :label="t('menu.assets.size')" width="100">
            <template #default="{ row }">{{ formatFileSize(row.size) }}</template>
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
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessageBox } from "element-plus";
import AssetFileIcon from "@/components/assets/AssetFileIcon.vue";
import AssetUploader from "@/components/AssetUploader.vue";
import {
  assetDisplayName,
  formatFileSize,
  listPostAssetsApi,
  unlinkPostAssetApi,
  type AssetView,
} from "@/api/system/assets.ts";
import { assetPurposeAccept } from "@/utils/assetPurpose.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

const props = defineProps<{
  postId: number | null;
  initialCovers?: AssetView[];
  initialAttachments?: AssetView[];
}>();

const { t } = useI18n();
const covers = ref<AssetView[]>(props.initialCovers ?? []);
const attachments = ref<AssetView[]>(props.initialAttachments ?? []);
const coverMax = ref(3);

const coverPreviewList = computed(() => covers.value.map((item) => resolveAssetUrl(item.url)));
const attachmentAccept = assetPurposeAccept("attachment");
const canUploadCover = computed(
  () => props.postId != null && covers.value.length < coverMax.value,
);

watch(
  () => [props.initialCovers, props.initialAttachments] as const,
  ([list, attachmentList]) => {
    covers.value = list ?? [];
    attachments.value = attachmentList ?? [];
  },
);

async function reload() {
  if (props.postId == null) return;
  const res = await listPostAssetsApi(props.postId);
  if (res.code === 0 && res.data) {
    covers.value = res.data.covers;
    attachments.value = res.data.attachments;
    coverMax.value = res.data.cover_max;
  }
}

function onCoverUploaded(asset: AssetView) {
  if (covers.value.some((item) => item.id === asset.id)) {
    return;
  }
  covers.value = [...covers.value, asset];
  koiMsgSuccess(t("msg.success"));
}

function onAttachmentUploaded(asset: AssetView) {
  attachments.value = [...attachments.value, asset];
  koiMsgSuccess(t("msg.success"));
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

async function removeCover(row: AssetView) {
  if (!props.postId) return;
  const purge = await confirmPurge(t("menu.content.post.manage.removeCoverConfirm"));
  if (purge === null) return;
  const res = await unlinkPostAssetApi(props.postId, row.id, purge);
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    return;
  }
  covers.value = covers.value.filter((item) => item.id !== row.id);
  koiMsgSuccess(t("msg.success"));
}

async function removeAttachment(row: AssetView) {
  if (!props.postId) return;
  const purge = await confirmPurge(
    t("menu.content.post.manage.removeAttachmentConfirm", { name: assetDisplayName(row) }),
  );
  if (purge === null) return;
  const res = await unlinkPostAssetApi(props.postId, row.id, purge);
  if (res.code !== 0) {
    koiMsgError(res.message || t("msg.fail"));
    return;
  }
  attachments.value = attachments.value.filter((a) => a.id !== row.id);
  koiMsgSuccess(t("msg.success"));
}

defineExpose({ reload });
</script>

<style scoped lang="scss">
.post-assets-section__hint {
  margin-bottom: 16px;
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
}
</style>
