<template>
  <el-drawer
    v-model="visible"
    :title="drawerTitle"
    size="100%"
    class="post-form-drawer"
    :close-on-click-modal="false"
    append-to-body
    destroy-on-close
    @closed="onClosed"
  >
    <el-form
      ref="formRef"
      :model="form"
      label-width="88px"
      v-loading="loading"
      class="post-form-drawer__form"
    >
      <el-divider content-position="left">{{ t("menu.content.post.manage.sectionMeta") }}</el-divider>

      <el-form-item :label="t('menu.content.post.manage.category')">
        <el-select
          v-model="form.category_id"
          clearable
          filterable
          :placeholder="t('menu.content.post.manage.categoryPh')"
          style="width: 100%"
        >
          <el-option
            v-for="cat in categories"
            :key="cat.id"
            :label="cat.name"
            :value="cat.id"
          />
        </el-select>
      </el-form-item>

      <el-form-item :label="t('menu.content.post.manage.tags')">
        <el-input
          v-model="form.tags"
          :placeholder="t('menu.content.post.manage.tagsPh')"
        />
      </el-form-item>

      <el-form-item :label="t('menu.content.post.manage.status')">
        <el-radio-group v-model="form.status">
          <el-radio :value="0">{{ t("menu.content.post.manage.draft") }}</el-radio>
          <el-radio :value="1">{{ t("menu.content.post.manage.published") }}</el-radio>
        </el-radio-group>
      </el-form-item>

      <el-divider content-position="left">{{ t("menu.content.post.manage.sectionI18n") }}</el-divider>

      <el-tabs v-model="activeLocale" type="border-card" class="post-locale-tabs">
        <el-tab-pane
          v-for="loc in siteLocales"
          :key="loc"
          :label="localeLabel(loc)"
          :name="loc"
        >
          <el-form-item :label="t('menu.content.post.manage.titleCol')">
            <el-input
              v-model="form.i18n[loc].title"
              :placeholder="t('menu.content.post.manage.titlePh')"
            />
          </el-form-item>
          <el-form-item :label="t('menu.content.post.manage.description')">
            <el-input
              v-model="form.i18n[loc].description"
              type="textarea"
              :rows="2"
              :placeholder="t('menu.content.post.manage.descriptionPh')"
            />
          </el-form-item>
          <el-form-item :label="t('menu.content.post.manage.content')" class="post-form-drawer__content-item">
            <KoiMarkdownEditor
              v-model="form.i18n[loc].content"
              :editor-id="`post-content-${loc}`"
              :placeholder="t('menu.content.post.manage.contentPh')"
            />
          </el-form-item>
          <el-form-item :label="t('menu.content.post.manage.routePath')">
            <el-input
              v-model="form.i18n[loc].route_path"
              :placeholder="routePathPlaceholder(loc)"
            />
          </el-form-item>
        </el-tab-pane>
      </el-tabs>
    </el-form>

    <template #footer>
      <div class="post-drawer-footer">
        <el-button @click="visible = false">{{ t("button.cancel") }}</el-button>
        <el-button type="primary" :loading="saving" @click="handleSave">
          {{ t("button.save") }}
        </el-button>
      </div>
    </template>
  </el-drawer>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import type { FormInstance } from "element-plus";
import { useI18n } from "vue-i18n";
import {
  createPostApi,
  getPostApi,
  updatePostApi,
  type PostI18nPayload,
} from "@/api/system/posts.ts";
import type { CategoryView } from "@/api/system/categories.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

export type PostLocaleForm = Record<string, PostI18nPayload>;

const props = defineProps<{
  modelValue: boolean;
  editId: number | null;
  siteLocales: string[];
  defaultLocale: string;
  categories: CategoryView[];
  localeLabel: (loc: string) => string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "saved"): void;
}>();

const { t } = useI18n();
const formRef = ref<FormInstance>();
const loading = ref(false);
const saving = ref(false);
const activeLocale = ref("zh-cn");

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit("update:modelValue", v),
});

const isEdit = computed(() => props.editId !== null);
const drawerTitle = computed(() =>
  isEdit.value ? t("menu.content.post.manage.edit") : t("menu.content.post.manage.create"),
);

const form = reactive({
  category_id: undefined as number | undefined,
  tags: "",
  status: 0,
  i18n: {} as PostLocaleForm,
});

function emptyI18n(): PostLocaleForm {
  const map: PostLocaleForm = {};
  for (const loc of props.siteLocales) {
    map[loc] = { title: "", description: "", content: "", route_path: "" };
  }
  return map;
}

function routePathPlaceholder(loc: string): string {
  return loc === "en-us" ? "/en-us/posts/hello" : "/zh-cn/posts/hello";
}

function hasLocaleContent(row: PostI18nPayload): boolean {
  return !!(row.title.trim() || row.description.trim() || row.content.trim() || row.route_path.trim());
}

function resetForm() {
  form.category_id = undefined;
  form.tags = "";
  form.status = 0;
  form.i18n = emptyI18n();
  activeLocale.value = props.defaultLocale || props.siteLocales[0] || "zh-cn";
}

async function loadDetail() {
  if (props.editId === null) return;
  loading.value = true;
  form.i18n = emptyI18n();
  try {
    const res = await getPostApi(props.editId);
    if (res.code !== 0 || !res.data) {
      koiMsgError(res.message || t("msg.fail"));
      return;
    }
    const detail = res.data;
    form.category_id = detail.category_id || undefined;
    form.tags = detail.tags;
    form.status = detail.status;
    for (const loc of props.siteLocales) {
      const tr = detail.translations[loc];
      if (tr) {
        form.i18n[loc] = {
          title: tr.title,
          description: tr.description,
          content: tr.content,
          route_path: tr.route_path,
        };
      }
    }
  } finally {
    loading.value = false;
  }
}

watch(
  () => props.modelValue,
  (open) => {
    if (!open) return;
    if (isEdit.value) {
      loadDetail();
    } else {
      resetForm();
    }
  },
);

function onClosed() {
  formRef.value?.resetFields();
}

async function handleSave() {
  if (!formRef.value) return;
  const defaultTitle = form.i18n[props.defaultLocale]?.title?.trim();
  if (!defaultTitle) {
    activeLocale.value = props.defaultLocale;
    koiMsgError(t("menu.content.post.manage.titleRequired"));
    return;
  }

  saving.value = true;
  try {
    const metaPayload = {
      category_id: form.category_id,
      tags: form.tags || undefined,
      status: form.status,
    };

    if (!isEdit.value) {
      const primary = form.i18n[props.defaultLocale];
      const createRes = await createPostApi({
        ...metaPayload,
        title: primary.title.trim(),
        description: primary.description.trim() || undefined,
        content: primary.content.trim() || undefined,
        route_path: primary.route_path.trim() || undefined,
        lang: props.defaultLocale,
      });
      if (createRes.code !== 0 || !createRes.data) {
        koiMsgError(createRes.message || t("msg.fail"));
        return;
      }
      const newId = createRes.data.id;
      for (const loc of props.siteLocales) {
        if (loc === props.defaultLocale) continue;
        const row = form.i18n[loc];
        if (!hasLocaleContent(row)) continue;
        const res = await updatePostApi(newId, {
          title: row.title.trim(),
          description: row.description.trim() || undefined,
          content: row.content.trim() || undefined,
          route_path: row.route_path.trim() || undefined,
          lang: loc,
        });
        if (res.code !== 0) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
      }
    } else {
      const metaRes = await updatePostApi(props.editId!, metaPayload);
      if (metaRes.code !== 0) {
        koiMsgError(metaRes.message || t("msg.fail"));
        return;
      }
      for (const loc of props.siteLocales) {
        const row = form.i18n[loc];
        if (!hasLocaleContent(row)) continue;
        const res = await updatePostApi(props.editId!, {
          title: row.title.trim(),
          description: row.description.trim() || undefined,
          content: row.content.trim() || undefined,
          route_path: row.route_path.trim() || undefined,
          lang: loc,
        });
        if (res.code !== 0) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
      }
    }

    koiMsgSuccess(t("msg.success"));
    visible.value = false;
    emit("saved");
  } finally {
    saving.value = false;
  }
}
</script>

<style scoped lang="scss">
.post-form-drawer {
  :deep(.el-drawer__body) {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
}

.post-form-drawer__form {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: auto;
}

.post-locale-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  margin-top: 4px;

  :deep(.el-tabs__content) {
    flex: 1;
    overflow: auto;
  }

  :deep(.el-tab-pane) {
    height: 100%;
  }
}

.post-form-drawer__content-item {
  :deep(.el-form-item__content) {
    line-height: normal;
  }
}

.post-drawer-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
