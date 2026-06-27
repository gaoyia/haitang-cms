<template>
  <div class="dict-manage-page koi-page">
    <KoiCard>
      <template #header>
        <div class="page-head">
          <div>
            <div class="page-head__title">{{ t("menu.dict.manage.title") }}</div>
            <div class="page-head__desc">{{ t("menu.dict.manage.subtitle") }}</div>
          </div>
          <KoiToolbar :show-maximize="false" @refresh-table="loadDicts" />
        </div>
      </template>

      <div class="page-toolbar">
        <el-button type="primary" @click="openDialog(null)">
          <el-icon><Plus /></el-icon>
          {{ t("button.add") }}
        </el-button>
      </div>

      <KoiTablePanel
        v-model:page="page"
        v-model:page-size="pageSize"
        :loading="loading"
        :data="dicts"
        :total="total"
        stripe
        border
        class="page-table"
        @change="loadDicts"
      >
        <el-table-column prop="code" :label="t('menu.dict.manage.code')" min-width="140">
          <template #default="{ row }"><code>{{ row.code }}</code></template>
        </el-table-column>
        <el-table-column prop="label" :label="t('menu.dict.manage.label')" min-width="140" />
        <el-table-column :label="t('menu.dict.manage.translatable')" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="row.translatable ? 'success' : 'info'" effect="plain" size="small">
              {{ row.translatable ? t("menu.dict.manage.yes") : t("menu.dict.manage.no") }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="sort" :label="t('menu.dict.manage.sort')" width="90" align="center" />
        <el-table-column :label="t('table.operate')" width="160" fixed="right" align="center">
          <template #default="{ row }">
            <el-button type="primary" link @click="openDialog(row)">{{ t("button.update") }}</el-button>
            <el-button type="danger" link @click="handleDelete(row)">{{ t("button.delete") }}</el-button>
          </template>
        </el-table-column>
      </KoiTablePanel>
    </KoiCard>

    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? t('menu.dict.manage.edit') : t('menu.dict.manage.create')"
      width="620px"
      :close-on-click-modal="false"
      append-to-body
      destroy-on-close
      @closed="onDialogClosed"
    >
      <el-form ref="formRef" :model="form" :rules="rules" label-width="88px">
        <el-form-item :label="t('menu.dict.manage.code')" prop="code">
          <el-input v-model="form.code" :disabled="isEdit" :placeholder="t('menu.dict.manage.codePh')" />
        </el-form-item>
        <el-form-item :label="t('menu.dict.manage.label')" prop="label">
          <el-input v-model="form.label" />
        </el-form-item>
        <el-form-item :label="t('menu.dict.manage.description')">
          <el-input v-model="form.description" type="textarea" :rows="2" />
        </el-form-item>
        <el-form-item :label="t('menu.dict.manage.translatable')">
          <el-switch v-model="form.translatable" />
        </el-form-item>
        <el-form-item :label="t('menu.dict.manage.sort')">
          <el-input-number v-model="form.sort" :min="0" :max="9999" />
        </el-form-item>
        <el-form-item :label="t('menu.dict.manage.values')">
          <template v-if="form.translatable">
            <el-tabs v-model="activeLocale" type="border-card" class="value-tabs">
              <el-tab-pane
                v-for="loc in siteLocales"
                :key="loc"
                :label="localeLabel(loc)"
                :name="loc"
              >
                <el-input
                  v-model="form.values[loc]"
                  type="textarea"
                  :rows="3"
                  :placeholder="t('menu.dict.manage.valuePh')"
                />
              </el-tab-pane>
            </el-tabs>
          </template>
          <el-input
            v-else
            v-model="singleValue"
            type="textarea"
            :rows="3"
            :placeholder="t('menu.dict.manage.valuePh')"
          />
        </el-form-item>
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
import type { FormInstance, FormRules } from "element-plus";
import { useI18n } from "vue-i18n";
import { Plus } from "@element-plus/icons-vue";
import { ElMessageBox } from "element-plus";
import {
  createDictApi,
  deleteDictApi,
  getDictApi,
  listDictsApi,
  updateDictApi,
  updateDictValuesApi,
  type DictMetaView,
} from "@/api/system/dict.ts";
import { useSiteLocales } from "@/composables/useSiteLocales.ts";
import { useTablePage } from "@/composables/useTablePage.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

const { t } = useI18n();
const { siteLocales, defaultLocale, loadSiteLocales, localeLabel, emptyLocaleRecord } = useSiteLocales();
const { page, pageSize, total, pageParams, applyPageResult } = useTablePage();

const loading = ref(false);
const saving = ref(false);
const dicts = ref<DictMetaView[]>([]);

const dialogVisible = ref(false);
const editingCode = ref<string | null>(null);
const formRef = ref<FormInstance>();
const activeLocale = ref("zh-cn");
const singleValue = ref("");

const isEdit = computed(() => editingCode.value !== null);

const form = reactive({
  code: "",
  label: "",
  description: "",
  translatable: false,
  sort: 0,
  values: {} as Record<string, string>,
});

const rules = computed<FormRules>(() => ({
  code: [
    { required: true, message: t("menu.dict.manage.codeRequired"), trigger: "blur" },
    {
      pattern: /^[a-z][a-z0-9_]*$/,
      message: t("menu.dict.manage.codePattern"),
      trigger: "blur",
    },
  ],
  label: [{ required: true, message: t("menu.dict.manage.labelRequired"), trigger: "blur" }],
}));

watch(
  () => form.translatable,
  (translatable) => {
    if (translatable) {
      const merged = emptyLocaleRecord(() => "");
      for (const loc of siteLocales.value) {
        merged[loc] = form.values[loc] ?? singleValue.value;
      }
      form.values = merged;
      activeLocale.value = defaultLocale.value;
    } else {
      singleValue.value = form.values[defaultLocale.value] ?? form.values[""] ?? singleValue.value;
    }
  },
);

function initValuesMap(existing?: Record<string, string>) {
  if (form.translatable) {
    form.values = emptyLocaleRecord(() => "");
    if (existing) {
      for (const loc of siteLocales.value) {
        if (existing[loc] !== undefined) {
          form.values[loc] = existing[loc];
        }
      }
    }
    activeLocale.value = defaultLocale.value;
  } else {
    singleValue.value = existing?.[""] ?? "";
    form.values = { "": singleValue.value };
  }
}

async function loadDicts() {
  loading.value = true;
  try {
    const res = await listDictsApi(pageParams.value);
    dicts.value = applyPageResult(res.code === 0 ? res.data : null);
  } finally {
    loading.value = false;
  }
}

function resetForm() {
  form.code = "";
  form.label = "";
  form.description = "";
  form.translatable = false;
  form.sort = 0;
  initValuesMap();
}

async function openDialog(row: DictMetaView | null) {
  if (row) {
    editingCode.value = row.code;
    form.code = row.code;
    form.label = row.label;
    form.description = row.description;
    form.translatable = row.translatable;
    form.sort = row.sort;
    const detailRes = await getDictApi(row.code);
    initValuesMap(detailRes.code === 0 && detailRes.data ? detailRes.data.values : undefined);
  } else {
    editingCode.value = null;
    resetForm();
  }
  dialogVisible.value = true;
}

function onDialogClosed() {
  formRef.value?.resetFields();
  editingCode.value = null;
}

function buildValuesPayload(): Record<string, string> {
  if (form.translatable) {
    const payload: Record<string, string> = {};
    for (const loc of siteLocales.value) {
      payload[loc] = form.values[loc] ?? "";
    }
    return payload;
  }
  return { "": singleValue.value };
}

async function handleSave() {
  if (!formRef.value) return;
  await formRef.value.validate(async (valid) => {
    if (!valid) return;
    saving.value = true;
    try {
      const valuesPayload = buildValuesPayload();

      if (isEdit.value && editingCode.value) {
        const metaRes = await updateDictApi(editingCode.value, {
          label: form.label.trim(),
          description: form.description.trim(),
          translatable: form.translatable,
          sort: form.sort,
        });
        if (metaRes.code !== 0) {
          koiMsgError(metaRes.message || t("msg.fail"));
          return;
        }
        const valuesRes = await updateDictValuesApi(editingCode.value, valuesPayload);
        if (valuesRes.code !== 0) {
          koiMsgError(valuesRes.message || t("msg.fail"));
          return;
        }
      } else {
        const firstEntry = Object.entries(valuesPayload)[0];
        const createRes = await createDictApi({
          code: form.code.trim(),
          label: form.label.trim(),
          description: form.description.trim() || undefined,
          translatable: form.translatable,
          sort: form.sort,
          value: firstEntry?.[1] ?? "",
          lang: form.translatable ? firstEntry?.[0] || defaultLocale.value : undefined,
        });
        if (createRes.code !== 0) {
          koiMsgError(createRes.message || t("msg.fail"));
          return;
        }
        const valuesRes = await updateDictValuesApi(form.code.trim(), valuesPayload);
        if (valuesRes.code !== 0) {
          koiMsgError(valuesRes.message || t("msg.fail"));
          return;
        }
      }

      koiMsgSuccess(t("msg.success"));
      dialogVisible.value = false;
      await loadDicts();
    } finally {
      saving.value = false;
    }
  });
}

async function handleDelete(row: DictMetaView) {
  try {
    await ElMessageBox.confirm(
      t("menu.dict.manage.deleteConfirm", { name: row.label }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deleteDictApi(row.code);
  if (res.code === 0) {
    koiMsgSuccess(t("msg.success"));
    await loadDicts();
  } else {
    koiMsgError(res.message || t("msg.fail"));
  }
}

onMounted(async () => {
  await loadSiteLocales();
  activeLocale.value = defaultLocale.value;
  await loadDicts();
});
</script>

<style scoped lang="scss">
.dict-manage-page {
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

.page-toolbar {
  margin-bottom: 12px;
}

.page-table {
  width: 100%;
}

.value-tabs {
  width: 100%;
}
</style>
