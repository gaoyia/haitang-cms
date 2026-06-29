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
        <el-table-column :label="t('menu.content.category.name')" min-width="160" show-overflow-tooltip>
          <template #default="{ row }">
            <div class="title-with-public-url">
              <PublicUrlPopover :id-url="categoryIdUrl(row.id)" :seo-url="categorySeoUrl(row)" />
              <a
                :href="primaryPublicUrl(categoryIdUrl(row.id), categorySeoUrl(row))"
                target="_blank"
                rel="noopener noreferrer"
                class="title-with-public-url__link"
              >
                {{ row.name }}
              </a>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('menu.content.category.listTemplate')" width="120" align="center">
          <template #default="{ row }">{{ templateLabel(row.list_template) }}</template>
        </el-table-column>
        <el-table-column
          prop="description"
          :label="t('menu.content.category.description')"
          min-width="180"
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
      width="640px"
      :close-on-click-modal="false"
      append-to-body
      destroy-on-close
      @closed="onDialogClosed"
    >
      <el-form ref="formRef" :model="form" label-width="100px" v-loading="detailLoading">
        <el-form-item :label="t('menu.content.category.sort')">
          <el-input-number v-model="form.sort" :min="0" :max="9999" />
        </el-form-item>
        <el-form-item :label="t('menu.content.category.listTemplate')">
          <el-select v-model="form.list_template" style="width: 100%">
            <el-option value="default" :label="t('menu.content.category.templateDefault')" />
            <el-option value="gallery" :label="t('menu.content.category.templateGallery')" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('menu.content.category.detailTemplate')">
          <el-select v-model="form.detail_template" style="width: 100%">
            <el-option value="default" :label="t('menu.content.category.templateDefault')" />
            <el-option value="gallery" :label="t('menu.content.category.templateGallery')" />
          </el-select>
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
            <el-form-item
              :label="t('menu.content.category.seoPath')"
              :error="seoSlugError(loc)"
            >
              <el-input
                v-model="form.i18n[loc].seoSlug"
                :placeholder="t('menu.content.category.seoPathPh')"
                clearable
              >
                <template #prepend>{{ seoPathPrefix(loc) }}</template>
              </el-input>
              <p class="field-hint">{{ t("menu.content.category.seoPathHint") }}</p>
            </el-form-item>
            <el-form-item v-if="editingId !== null" :label="t('menu.content.category.publicUrl')">
              <code class="public-url-preview">{{ publicCategoryUrlForForm(loc) }}</code>
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
  type CategoryTemplate,
  type CategoryView,
} from "@/api/system/categories.ts";
import { useSiteLocales } from "@/composables/useSiteLocales.ts";
import { useTablePage } from "@/composables/useTablePage.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import { primaryPublicUrl } from "@/utils/publicUrl.ts";
import PublicUrlPopover from "@/components/PublicUrlPopover.vue";

type LocaleFormRow = { name: string; description: string; seoSlug: string };
type LocaleForm = Record<string, LocaleFormRow>;

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
  list_template: "default" as CategoryTemplate,
  detail_template: "default" as CategoryTemplate,
  i18n: {} as LocaleForm,
});

const localeSegmentOptions = computed(() =>
  siteLocales.value.map((loc) => ({
    label: loc === "en-us" ? "EN" : loc === "zh-cn" ? "中文" : loc,
    value: loc,
  })),
);

function templateLabel(tpl: CategoryTemplate): string {
  return tpl === "gallery"
    ? t("menu.content.category.templateGallery")
    : t("menu.content.category.templateDefault");
}

function seoPathPrefix(loc: string): string {
  return `/${loc}/categories/`;
}

function parseSeoSlug(loc: string, routePath: string): string {
  const trimmed = routePath.trim();
  if (!trimmed) return "";
  const prefix = seoPathPrefix(loc);
  if (trimmed.startsWith(prefix)) return trimmed.slice(prefix.length);
  if (!trimmed.includes("/")) return trimmed;
  return "";
}

function buildRoutePath(loc: string, slug: string): string {
  const s = slug.trim();
  if (!s) return "";
  return `${seoPathPrefix(loc)}${s}`;
}

function validateSeoSlug(slug: string): string {
  const trimmed = slug.trim();
  if (!trimmed) return "";
  if (/[\s#?/]/.test(trimmed)) {
    return t("menu.content.category.seoPathSlugError");
  }
  return "";
}

function seoSlugError(loc: string): string {
  return validateSeoSlug(form.i18n[loc]?.seoSlug ?? "");
}

function validateAllSeoSlugs(): boolean {
  for (const loc of siteLocales.value) {
    const err = validateSeoSlug(form.i18n[loc]?.seoSlug ?? "");
    if (err) {
      activeLocale.value = loc;
      koiMsgError(err);
      return false;
    }
  }
  return true;
}

function categoryIdUrl(id: number): string {
  return `/${previewLang.value}/categories/${id}`;
}

function categorySeoUrl(row: CategoryView): string | null {
  const path = row.route_path?.trim();
  return path || null;
}

function publicCategoryUrlForForm(loc: string): string {
  if (editingId.value === null) return "—";
  const slug = form.i18n[loc]?.seoSlug?.trim();
  const key = slug || String(editingId.value);
  return `/${loc}/categories/${key}`;
}

function resetForm() {
  form.sort = 0;
  form.list_template = "default";
  form.detail_template = "default";
  form.i18n = emptyLocaleRecord(() => ({ name: "", description: "", seoSlug: "" }));
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
    form.list_template = row.list_template;
    form.detail_template = row.detail_template;
  }
  dialogVisible.value = true;
  if (row) {
    loadDetail(row.id);
  }
}

async function loadDetail(id: number) {
  detailLoading.value = true;
  form.i18n = emptyLocaleRecord(() => ({ name: "", description: "", seoSlug: "" }));
  try {
    const res = await getCategoryApi(id);
    if (res.code !== 0 || !res.data) {
      koiMsgError(res.message || t("msg.fail"));
      return;
    }
    form.sort = res.data.sort;
    form.list_template = res.data.list_template;
    form.detail_template = res.data.detail_template;
    for (const loc of siteLocales.value) {
      const row = res.data.translations[loc];
      if (row) {
        form.i18n[loc] = {
          name: row.name,
          description: row.description,
          seoSlug: parseSeoSlug(loc, row.route_path),
        };
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
  if (!validateAllSeoSlugs()) return;

  saving.value = true;
  try {
    if (editingId.value === null) {
      const primary = form.i18n[defaultLocale.value];
      const createRes = await createCategoryApi({
        name: primary.name.trim(),
        description: primary.description.trim(),
        sort: form.sort,
        lang: defaultLocale.value,
        list_template: form.list_template,
        detail_template: form.detail_template,
        route_path: buildRoutePath(defaultLocale.value, primary.seoSlug) || undefined,
      });
      if (createRes.code !== 0 || !createRes.data) {
        koiMsgError(createRes.message || t("msg.fail"));
        return;
      }
      const newId = createRes.data.id;
      for (const loc of siteLocales.value) {
        if (loc === defaultLocale.value) continue;
        const row = form.i18n[loc];
        if (!row.name.trim() && !row.description.trim() && !row.seoSlug.trim()) continue;
        const res = await updateCategoryApi(newId, {
          name: row.name.trim(),
          description: row.description.trim(),
          route_path: buildRoutePath(loc, row.seoSlug) || undefined,
          lang: loc,
        });
        if (res.code !== 0) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
      }
    } else {
      const id = editingId.value;
      const metaRes = await updateCategoryApi(id, {
        sort: form.sort,
        list_template: form.list_template,
        detail_template: form.detail_template,
      });
      if (metaRes.code !== 0) {
        koiMsgError(metaRes.message || t("msg.fail"));
        return;
      }
      for (const loc of siteLocales.value) {
        const row = form.i18n[loc];
        if (!row.name.trim() && !row.description.trim() && !row.seoSlug.trim()) continue;
        const res = await updateCategoryApi(id, {
          name: row.name.trim(),
          description: row.description.trim(),
          route_path: buildRoutePath(loc, row.seoSlug),
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

.title-with-public-url {
  display: flex;
  align-items: center;
  gap: 4px;
  min-width: 0;

  &__link {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--el-text-color-primary);
    text-decoration: none;

    &:hover {
      color: var(--el-color-primary);
      text-decoration: underline;
    }
  }
}

.locale-tabs {
  margin-top: 4px;
}

.field-hint {
  margin: 6px 0 0;
  font-size: 12px;
  line-height: 1.5;
  color: var(--el-text-color-secondary);
}

.public-url-preview {
  padding: 6px 10px;
  border-radius: 6px;
  background: var(--el-fill-color-light);
  font-size: 13px;
  word-break: break-all;
}
</style>
