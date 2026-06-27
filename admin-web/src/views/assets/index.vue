<template>
  <div class="assets-page koi-page">
    <KoiCard>
      <template #header>
        <div class="page-head">
          <div>
            <div class="page-head__title">{{ t("menu.assets.title") }}</div>
            <div class="page-head__desc">{{ t("menu.assets.subtitle") }}</div>
          </div>
          <KoiToolbar :show-maximize="false" @refresh-table="loadAssets" />
        </div>
      </template>

      <div class="table-toolbar">
        <div class="table-toolbar__left filters">
          <span class="table-toolbar__label">{{ t("menu.assets.purpose") }}</span>
          <el-select v-model="filterPurpose" clearable style="width: 140px" @change="onFilterChange">
            <el-option :label="t('menu.assets.purposeAll')" value="" />
            <el-option :label="t('menu.assets.purposeCover')" value="cover" />
            <el-option :label="t('menu.assets.purposeContent')" value="content" />
            <el-option :label="t('menu.assets.purposeBanner')" value="banner" />
            <el-option :label="t('menu.assets.purposeAttachment')" value="attachment" />
          </el-select>
          <el-input
            v-model="keyword"
            clearable
            :placeholder="t('menu.assets.keywordPh')"
            style="width: 220px"
            @keyup.enter="onFilterChange"
            @clear="onFilterChange"
          />
        </div>
        <div class="table-toolbar__right">
          <AssetUploader purpose="attachment" :label="t('menu.assets.upload')" @success="onUploaded" />
        </div>
      </div>

      <KoiTablePanel
        v-model:page="page"
        v-model:page-size="pageSize"
        :loading="loading"
        :data="assets"
        :total="total"
        :empty-text="t('menu.assets.empty')"
        stripe
        class="assets-table"
        @change="loadAssets"
      >
        <el-table-column type="index" :label="t('table.number')" width="60" />
        <el-table-column :label="t('menu.assets.uploadName')" min-width="180" show-overflow-tooltip>
          <template #default="{ row }">
            <a
              v-if="row.mime_type.startsWith('image/')"
              :href="resolveAssetUrl(row.url)"
              target="_blank"
              rel="noopener"
            >{{ assetDisplayName(row) }}</a>
            <span v-else>{{ assetDisplayName(row) }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="original_name" :label="t('menu.assets.storageName')" min-width="160" show-overflow-tooltip />
        <el-table-column prop="purpose" :label="t('menu.assets.purpose')" width="110" />
        <el-table-column :label="t('menu.assets.size')" width="100">
          <template #default="{ row }">{{ formatFileSize(row.size) }}</template>
        </el-table-column>
        <el-table-column prop="ref_count" :label="t('menu.assets.refs')" width="80" align="center" />
        <el-table-column :label="t('table.operate')" width="100" fixed="right">
          <template #default="{ row }">
            <el-button type="danger" link @click="handleDelete(row)">{{ t("button.delete") }}</el-button>
          </template>
        </el-table-column>
      </KoiTablePanel>
    </KoiCard>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessageBox } from "element-plus";
import AssetUploader from "@/components/AssetUploader.vue";
import {
  assetDisplayName,
  deleteAssetApi,
  formatFileSize,
  listAssetsApi,
  type AssetPurpose,
  type AssetView,
} from "@/api/system/assets.ts";
import { useTablePage } from "@/composables/useTablePage.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

const { t } = useI18n();
const { page, pageSize, total } = useTablePage();
const loading = ref(false);
const assets = ref<AssetView[]>([]);
const filterPurpose = ref<AssetPurpose | "">("");
const keyword = ref("");

async function loadAssets() {
  loading.value = true;
  try {
    const res = await listAssetsApi({
      page: page.value,
      page_size: pageSize.value,
      purpose: filterPurpose.value || undefined,
      keyword: keyword.value.trim() || undefined,
    });
    if (res.code !== 0 || !res.data) {
      koiMsgError(res.message || t("msg.fail"));
      return;
    }
    assets.value = res.data.list;
    total.value = res.data.total;
  } finally {
    loading.value = false;
  }
}

function onFilterChange() {
  page.value = 1;
  loadAssets();
}

function onUploaded() {
  koiMsgSuccess(t("msg.success"));
  loadAssets();
}

async function handleDelete(row: AssetView) {
  try {
    await ElMessageBox.confirm(
      t("menu.assets.deleteConfirm", { name: assetDisplayName(row) }),
      t("button.delete"),
      { type: "warning" },
    );
  } catch {
    return;
  }
  const res = await deleteAssetApi(row.id);
  if (res.code !== 0) {
    koiMsgError(res.message || t("menu.assets.deleteBlocked"));
    return;
  }
  koiMsgSuccess(t("msg.success"));
  loadAssets();
}

onMounted(loadAssets);
</script>

<style scoped lang="scss">
.assets-page {
  height: 100%;
}

.page-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;

  &__title {
    font-size: 18px;
    font-weight: 600;
    color: var(--el-text-color-primary);
  }

  &__desc {
    margin-top: 4px;
    font-size: 13px;
    color: var(--el-text-color-secondary);
  }
}

.table-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 14px;
  flex-shrink: 0;

  &__left {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  &__right {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-shrink: 0;
  }

  &__label {
    font-size: 13px;
    color: var(--el-text-color-secondary);
  }
}

.filters {
  flex-wrap: wrap;
}

.assets-table {
  width: 100%;
}
</style>
