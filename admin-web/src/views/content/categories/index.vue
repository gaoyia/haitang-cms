<template>
  <div class="category-page koi-page">
    <KoiCard>
      <template #header>
        <div class="page-head">
          <div>
            <div class="page-head__title">{{ t("menu.content.category.title") }}</div>
            <div class="page-head__desc">{{ t("menu.content.category.subtitle") }}</div>
          </div>
          <KoiToolbar :show-maximize="false" @refresh-table="loadCategories" />
        </div>
      </template>

      <div class="table-toolbar">
        <div class="table-toolbar__left">
          <span class="table-toolbar__label">{{ t("menu.content.category.previewLang") }}</span>
          <el-segmented v-model="previewLang" :options="localeSegmentOptions" size="small" />
        </div>
        <el-button type="primary" @click="openDialog(null)">
          <el-icon><Plus /></el-icon>
          {{ t("button.add") }}
        </el-button>
      </div>

      <KoiTablePanel
        v-model:page="page"
        v-model:page-size="pageSize"
        :loading="loading"
        :data="categories"
        :total="total"
        :empty-text="t('menu.content.category.empty')"
        stripe
        class="category-table"
        @change="loadCategories"
      >
        <el-table-column type="index" :label="t('table.number')" width="60" />
        <el-table-column prop="name" :label="t('menu.content.category.name')" min-width="160" />
        <el-table-column
          prop="description"
          :label="t('menu.content.category.description')"
          min-width="220"
          show-overflow-tooltip
        />
        <el-table-column prop="sort" :label="t('menu.content.category.sort')" width="80" align="center" />
        <el-table-column :label="t('table.operate')" width="140" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" link @click="openDialog(row)">{{ t("button.update") }}</el-button>
            <el-button type="danger" link @click="handleDelete(row)">{{ t("button.delete") }}</el-button>
          </template>
        </el-table-column>
      </KoiTablePanel>
    </KoiCard>

    <el-dialog
      v-model="dialogVisible"
      :title="editingId === null ? t('menu.content.category.create') : t('menu.content.category.edit')"
      width="520px"
      :close-on-click-modal="false"
      append-to-body
      destroy-on-close
      @closed="onDialogClosed"
    >
      <el-form ref="formRef" :model="form" label-width="88px" v-loading="detailLoading">
        <el-form-item :label="t('menu.content.category.sort')">
          <el-input-number v-model="form.sort" :min="0" :max="9999" />
        </el-form-item>

        <el-tabs v-model="activeLocale" type="border-card" class="locale-tabs">
          <el-tab-pane
            v-for="loc in siteLocales"
            :key="loc"
            :label="localeLabel(loc)"
            :name="loc"
          >
            <el-form-item :label="t('menu.content.category.name')">
              <el-input
                v-model="form.i18n[loc].name"
                :placeholder="t('menu.content.category.namePh')"
              />
            </el-form-item>
            <el-form-item :label="t('menu.content.category.description')">
              <el-input
                v-model="form.i18n[loc].description"
                type="textarea"
                :rows="3"
                :placeholder="t('menu.content.category.descPh')"
              />
            </el-form-item>
          </el-tab-pane>
        </el-tabs>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">{{ t("button.cancel") }}</el-button>
        <el-button type="primary" :loading="saving" @click="handleSave">{{ t("button.confirm") }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from "vue";
import type { FormInstance } from "element-plus";
import { useI18n } from "vue-i18n";
import { Plus } from "@element-plus/icons-vue";
import { ElMessageBox } from "element-plus";
import {
  createCategoryApi,
  deleteCategoryApi,
  getCategoryApi,
  listCategoriesApi,
  updateCategoryApi,
  type CategoryView,
} from "@/api/system/categories.ts";
import { useSiteLocales } from "@/composables/useSiteLocales.ts";
import { useTablePage } from "@/composables/useTablePage.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

type LocaleForm = Record<string, { name: string; description: string }>;

const { t } = useI18n();
const { siteLocales, defaultLocale, previewLang, loadSiteLocales, localeLabel, emptyLocaleRecord } =
  useSiteLocales();
const { page, pageSize, total, pageParams, applyPageResult, resetPage } = useTablePage();

const loading = ref(false);
const detailLoading = ref(false);
const saving = ref(false);
const categories = ref<CategoryView[]>([]);

const dialogVisible = ref(false);
const editingId = ref<number | null>(null);
const formRef = ref<FormInstance>();
const activeLocale = ref("zh-cn");

const form = reactive({
  sort: 0,
  i18n: {} as LocaleForm,
});

const localeSegmentOptions = computed(() =>
  siteLocales.value.map((loc) => ({
    label: loc === "en-us" ? "EN" : loc === "zh-cn" ? "中文" : loc,
    value: loc,
  })),
);

function resetForm() {
  form.sort = 0;
  form.i18n = emptyLocaleRecord(() => ({ name: "", description: "" }));
  activeLocale.value = defaultLocale.value;
}

async function loadCategories() {
  loading.value = true;
  try {
    const res = await listCategoriesApi(previewLang.value, pageParams.value);
    categories.value = applyPageResult(res.code === 0 ? res.data : null);
  } finally {
    loading.value = false;
  }
}

function openDialog(row: CategoryView | null) {
  editingId.value = row?.id ?? null;
  resetForm();
  if (row) {
    form.sort = row.sort;
  }
  dialogVisible.value = true;
  if (row) {
    loadDetail(row.id);
  }
}

async function loadDetail(id: number) {
  detailLoading.value = true;
  form.i18n = emptyLocaleRecord(() => ({ name: "", description: "" }));
  try {
    const res = await getCategoryApi(id);
    if (res.code !== 0 || !res.data) {
      koiMsgError(res.message || t("msg.fail"));
      return;
    }
    form.sort = res.data.sort;
    for (const loc of siteLocales.value) {
      const row = res.data.translations[loc];
      if (row) {
        form.i18n[loc] = { name: row.name, description: row.description };
      }
    }
  } finally {
    detailLoading.value = false;
  }
}

async function handleSave() {
  const defaultName = form.i18n[defaultLocale.value]?.name?.trim();
  if (!defaultName) {
    activeLocale.value = defaultLocale.value;
    koiMsgError(t("menu.content.category.nameRequired"));
    return;
  }

  saving.value = true;
  try {
    if (editingId.value === null) {
      const primary = form.i18n[defaultLocale.value];
      const createRes = await createCategoryApi({
        name: primary.name.trim(),
        description: primary.description.trim(),
        sort: form.sort,
        lang: defaultLocale.value,
      });
      if (createRes.code !== 0 || !createRes.data) {
        koiMsgError(createRes.message || t("msg.fail"));
        return;
      }
      const newId = createRes.data.id;
      for (const loc of siteLocales.value) {
        if (loc === defaultLocale.value) continue;
        const row = form.i18n[loc];
        if (!row.name.trim() && !row.description.trim()) continue;
        const res = await updateCategoryApi(newId, {
          name: row.name.trim(),
          description: row.description.trim(),
          lang: loc,
        });
        if (res.code !== 0) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
      }
    } else {
      const sortRes = await updateCategoryApi(editingId.value, { sort: form.sort });
      if (sortRes.code !== 0) {
        koiMsgError(sortRes.message || t("msg.fail"));
        return;
      }
      for (const loc of siteLocales.value) {
        const row = form.i18n[loc];
        if (!row.name.trim() && !row.description.trim()) continue;
        const res = await updateCategoryApi(editingId.value, {
          name: row.name.trim(),
          description: row.description.trim(),
          lang: loc,
        });
        if (res.code !== 0) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
      }
    }

    koiMsgSuccess(t("msg.success"));
    dialogVisible.value = false;
    await loadCategories();
  } finally {
    saving.value = false;
  }
}

async function handleDelete(row: CategoryView) {
  try {
    await ElMessageBox.confirm(
      t("menu.content.category.deleteConfirm", { name: row.name }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deleteCategoryApi(row.id);
  if (res.code === 0) {
    koiMsgSuccess(t("msg.success"));
    await loadCategories();
  } else {
    koiMsgError(res.message || t("msg.fail"));
  }
}

function onDialogClosed() {
  formRef.value?.resetFields();
  editingId.value = null;
}

watch(previewLang, () => {
  resetPage();
  loadCategories();
});

onMounted(async () => {
  await loadSiteLocales();
  await loadCategories();
});
</script>

<style scoped lang="scss">
.category-page {
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

  &__left {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  &__label {
    font-size: 13px;
    color: var(--el-text-color-secondary);
  }
}

.category-table {
  width: 100%;
}

.locale-tabs {
  margin-top: 4px;
}
</style>
