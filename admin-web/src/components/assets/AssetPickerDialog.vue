<template>
  <el-dialog
    v-model="visible"
    :title="title || t('menu.assets.pickerTitle')"
    width="720px"
    :close-on-click-modal="false"
    append-to-body
    destroy-on-close
    class="asset-picker-dialog"
    @open="onOpen"
  >
    <div class="asset-picker-dialog__purpose">
      <span class="asset-picker-dialog__label">{{ t("menu.assets.purpose") }}</span>
      <el-tag type="info" effect="plain">{{ purposeLabel }}</el-tag>
      <span class="asset-picker-dialog__hint">{{ t("menu.assets.pickerPurposeHint") }}</span>
    </div>

    <el-tabs v-model="activeTab">
      <el-tab-pane :label="t('menu.assets.pickerSelectTab')" name="select">
        <div class="asset-picker-dialog__toolbar">
          <el-input
            v-model="keyword"
            clearable
            :placeholder="t('menu.assets.keywordPh')"
            style="width: 240px"
            @keyup.enter="loadAssets"
            @clear="loadAssets"
          />
          <el-button @click="loadAssets">{{ t("button.search") }}</el-button>
        </div>

        <div v-loading="loading" class="asset-picker-dialog__grid-wrap">
          <el-empty v-if="!loading && assets.length === 0" :description="t('menu.assets.empty')" />
          <div v-else class="asset-picker-dialog__grid">
            <button
              v-for="item in assets"
              :key="item.id"
              type="button"
              class="asset-picker-dialog__item"
              :class="{ 'is-selected': selectedId === item.id }"
              @click="selectedId = item.id"
            >
              <div class="asset-picker-dialog__thumb">
                <el-image
                  v-if="isImageAsset(item)"
                  :src="resolveAssetUrl(item.url)"
                  fit="cover"
                  class="asset-picker-dialog__img"
                />
                <div v-else class="asset-picker-dialog__file">
                  <AssetFileIcon
                    block
                    :mime-type="item.mime_type"
                    :filename="assetDisplayName(item)"
                    :size="40"
                    :show-title="false"
                  />
                </div>
              </div>
              <div class="asset-picker-dialog__name" :title="assetDisplayName(item)">{{ assetDisplayName(item) }}</div>
            </button>
          </div>
        </div>

        <div v-if="total > pageSize" class="asset-picker-dialog__pager">
          <el-pagination
            v-model:current-page="page"
            v-model:page-size="pageSize"
            layout="prev, pager, next"
            :total="total"
            @current-change="loadAssets"
          />
        </div>
      </el-tab-pane>

      <el-tab-pane :label="t('menu.assets.pickerUploadTab')" name="upload">
        <p class="asset-picker-dialog__upload-hint">{{ t("menu.assets.pickerUploadHint") }}</p>
        <AssetUploader
          :purpose="purpose"
          :accept="accept"
          :label="t('menu.assets.upload')"
          @success="onUploaded"
        />
      </el-tab-pane>
    </el-tabs>

    <template #footer>
      <el-button @click="visible = false">{{ t("button.cancel") }}</el-button>
      <el-button type="primary" :disabled="selectedId === null" @click="confirmSelect">
        {{ t("menu.assets.pickerConfirm") }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import AssetFileIcon from "@/components/assets/AssetFileIcon.vue";
import AssetUploader from "@/components/AssetUploader.vue";
import { assetDisplayName, listAssetsApi, type AssetPurpose, type AssetView } from "@/api/system/assets.ts";
import { assetPurposeAccept, assetPurposeIsImage } from "@/utils/assetPurpose.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { koiMsgError } from "@/utils/koi.ts";

const props = defineProps<{
  modelValue: boolean;
  purpose: AssetPurpose;
  title?: string;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
  select: [asset: AssetView];
}>();

const { t } = useI18n();
const activeTab = ref<"select" | "upload">("select");
const loading = ref(false);
const assets = ref<AssetView[]>([]);
const selectedId = ref<number | null>(null);
const keyword = ref("");
const page = ref(1);
const pageSize = ref(12);
const total = ref(0);

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit("update:modelValue", v),
});

const accept = computed(() => assetPurposeAccept(props.purpose));

const purposeLabel = computed(() => {
  const map: Record<AssetPurpose, string> = {
    cover: t("menu.assets.purposeCover"),
    content: t("menu.assets.purposeContent"),
    banner: t("menu.assets.purposeBanner"),
    attachment: t("menu.assets.purposeAttachment"),
  };
  return map[props.purpose];
});

function isImageAsset(item: AssetView): boolean {
  return assetPurposeIsImage(item.purpose) || item.mime_type.startsWith("image/");
}

function onOpen() {
  activeTab.value = "select";
  selectedId.value = null;
  keyword.value = "";
  page.value = 1;
  loadAssets();
}

async function loadAssets() {
  loading.value = true;
  try {
    const res = await listAssetsApi({
      page: page.value,
      page_size: pageSize.value,
      purpose: props.purpose,
      keyword: keyword.value.trim() || undefined,
    });
    if (res.code !== 0 || !res.data) {
      koiMsgError(res.message || t("msg.fail"));
      assets.value = [];
      total.value = 0;
      return;
    }
    assets.value = res.data.list;
    total.value = res.data.total;
  } finally {
    loading.value = false;
  }
}

function onUploaded(asset: AssetView) {
  emit("select", asset);
  visible.value = false;
}

function confirmSelect() {
  const picked = assets.value.find((a) => a.id === selectedId.value);
  if (!picked) return;
  emit("select", picked);
  visible.value = false;
}
</script>

<style scoped lang="scss">
.asset-picker-dialog__purpose {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 12px;
}

.asset-picker-dialog__label {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.asset-picker-dialog__hint {
  font-size: 12px;
  color: var(--el-text-color-placeholder);
}

.asset-picker-dialog__toolbar {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 12px;
}

.asset-picker-dialog__grid-wrap {
  min-height: 200px;
}

.asset-picker-dialog__grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 12px;
}

.asset-picker-dialog__item {
  border: 2px solid var(--el-border-color-lighter);
  border-radius: 8px;
  padding: 6px;
  background: var(--el-fill-color-blank);
  cursor: pointer;
  text-align: left;
  transition: border-color 0.15s;

  &:hover {
    border-color: var(--el-color-primary-light-5);
  }

  &.is-selected {
    border-color: var(--el-color-primary);
  }
}

.asset-picker-dialog__thumb {
  aspect-ratio: 4 / 3;
  border-radius: 4px;
  overflow: hidden;
  background: var(--el-fill-color-light);
}

.asset-picker-dialog__img {
  width: 100%;
  height: 100%;
}

.asset-picker-dialog__file {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
  word-break: break-all;
}

.asset-picker-dialog__name {
  margin-top: 6px;
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.asset-picker-dialog__pager {
  margin-top: 12px;
  display: flex;
  justify-content: flex-end;
}

.asset-picker-dialog__upload-hint {
  margin: 0 0 12px;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}
</style>
