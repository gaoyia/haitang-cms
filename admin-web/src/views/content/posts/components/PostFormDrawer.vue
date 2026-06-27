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
      :label-position="isFormStacked ? 'top' : 'right'"
      :label-width="isFormStacked ? undefined : '88px'"
      v-loading="loading"
      class="post-form-drawer__form"
      :class="{ 'post-form-drawer__form--stacked': isFormStacked }"
    >
      <el-divider content-position="left">{{ t("menu.content.post.manage.sectionMeta") }}</el-divider>

      <el-row :gutter="16" class="post-meta-row">
        <el-col :xs="24" :lg="8">
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
        </el-col>

        <el-col :xs="24" :lg="8">
          <el-form-item :label="t('menu.content.post.manage.displayTime')">
            <el-date-picker
              v-model="displayTimeDate"
              type="datetime"
              :placeholder="t('menu.content.post.manage.displayTimePh')"
              style="width: 100%"
            />
          </el-form-item>
        </el-col>

        <el-col :xs="24" :lg="8">
          <el-form-item :label="t('menu.content.post.manage.status')">
            <el-radio-group v-model="form.status">
              <el-radio :value="0">{{ t("menu.content.post.manage.draft") }}</el-radio>
              <el-radio :value="1">{{ t("menu.content.post.manage.published") }}</el-radio>
            </el-radio-group>
          </el-form-item>
        </el-col>
      </el-row>

      <el-divider content-position="left">{{ t("menu.content.post.manage.sectionAssets") }}</el-divider>
      <el-form-item label=" " class="post-form-drawer__assets-note-item">
        <p class="post-form-drawer__assets-note">
          {{ t("menu.content.post.manage.assetsImmediateEffect") }}
        </p>
      </el-form-item>
      <PostAssetsSection
        :post-id="assetPostId"
        :initial-covers="postCovers"
        :initial-attachments="postAttachments"
      />

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
          <el-form-item :label="t('menu.content.post.manage.tags')">
            <el-input-tag
              v-model="form.i18n[loc].tagList"
              :placeholder="t('menu.content.post.manage.tagsPh')"
              tag-type="primary"
              tag-effect="plain"
              delimiter=","
              clearable
              style="width: 100%"
            />
          </el-form-item>
          <el-form-item :label="t('menu.content.post.manage.content')" class="post-form-drawer__content-item">
            <KoiMarkdownEditor
              v-model="form.i18n[loc].content"
              :editor-id="`post-content-${loc}`"
              :placeholder="t('menu.content.post.manage.contentPh')"
              enable-image-upload
            />
          </el-form-item>
          <el-form-item :label="t('menu.content.post.manage.publicUrl')">
            <div class="post-url-block">
              <code class="post-url-preview">{{ publicUrlPreview(loc) }}</code>
              <el-link
                v-if="canOpenPublicUrl"
                :href="publicUrlPreview(loc)"
                target="_blank"
                type="primary"
                class="post-url-open"
              >
                {{ t("menu.content.post.manage.openPublic") }}
              </el-link>
            </div>
            <p class="post-field-hint">{{ t("menu.content.post.manage.publicUrlHint") }}</p>
          </el-form-item>
          <el-form-item
            :label="t('menu.content.post.manage.seoPath')"
            :error="seoSlugError(loc)"
          >
            <el-input
              v-model="form.i18n[loc].seoSlug"
              :placeholder="t('menu.content.post.manage.seoPathPh')"
              clearable
            >
              <template #prepend>{{ seoPathPrefix(loc) }}</template>
            </el-input>
            <p class="post-field-hint">{{ t("menu.content.post.manage.seoPathHint") }}</p>
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
import { useBreakpoints } from "@vueuse/core";
import { useI18n } from "vue-i18n";
import {
  createPostApi,
  getPostApi,
  updatePostApi,
  type PostI18nPayload,
} from "@/api/system/posts.ts";
import type { AssetView } from "@/api/system/assets.ts";
import type { CategoryView } from "@/api/system/categories.ts";
import PostAssetsSection from "@/components/assets/PostAssetsSection.vue";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import { nowUnix } from "@/utils/formatTime.ts";
import { breakpointsEnum } from "@/hooks/screen/index.ts";

/** 宽度 < 768px 时表单改为标签在上、字段纵向排列 */
const isFormStacked = useBreakpoints(breakpointsEnum).smaller("sm");

export type PostLocaleFormRow = Omit<PostI18nPayload, "tags" | "route_path"> & {
  tagList: string[];
  seoSlug: string;
};
export type PostLocaleForm = Record<string, PostLocaleFormRow>;

/** 将 API 逗号分隔标签解析为数组 */
function parseTags(raw: string): string[] {
  if (!raw.trim()) return [];
  return raw.split(/[,，]/).map((s) => s.trim()).filter(Boolean);
}

/** 将标签数组序列化为 API 逗号分隔字符串 */
function serializeTags(list: string[]): string {
  return list.map((s) => s.trim()).filter(Boolean).join(", ");
}

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
const sessionPostId = ref<number | null>(null);
const postCovers = ref<AssetView[]>([]);
const postAttachments = ref<AssetView[]>([]);

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit("update:modelValue", v),
});

const isEdit = computed(() => props.editId !== null);
const assetPostId = computed(() => props.editId ?? sessionPostId.value);
const canOpenPublicUrl = computed(
  () => assetPostId.value !== null && form.status === 1,
);
const drawerTitle = computed(() =>
  isEdit.value ? t("menu.content.post.manage.edit") : t("menu.content.post.manage.create"),
);

/** 当前语言下访客实际使用的公开路径 */
function publicUrlPreview(loc: string): string {
  const idPart = assetPostId.value != null
    ? String(assetPostId.value)
    : t("menu.content.post.manage.publicUrlPending");
  return `/${loc}/posts/${idPart}`;
}

function seoPathPrefix(loc: string): string {
  return `/${loc}/posts/`;
}

/** 从完整 route_path 解析 slug */
function parseSeoSlug(loc: string, routePath: string): string {
  const trimmed = routePath.trim();
  if (!trimmed) return "";

  const prefix = seoPathPrefix(loc);
  if (trimmed.startsWith(prefix)) {
    return trimmed.slice(prefix.length);
  }

  if (!trimmed.includes("/")) {
    return trimmed;
  }

  return "";
}

function buildRoutePath(loc: string, slug: string): string {
  const s = slug.trim();
  if (!s) return "";
  return `${seoPathPrefix(loc)}${s}`;
}

/** 校验 SEO slug；空字符串视为合法 */
function validateSeoSlug(slug: string): string {
  const trimmed = slug.trim();
  if (!trimmed) return "";
  if (/[\s#?/]/.test(trimmed)) {
    return t("menu.content.post.manage.seoPathSlugError");
  }
  return "";
}

function seoSlugError(loc: string): string {
  return validateSeoSlug(form.i18n[loc]?.seoSlug ?? "");
}

function validateAllSeoSlugs(): boolean {
  for (const loc of props.siteLocales) {
    const err = validateSeoSlug(form.i18n[loc]?.seoSlug ?? "");
    if (err) {
      activeLocale.value = loc;
      koiMsgError(err);
      return false;
    }
  }
  return true;
}

const form = reactive({
  category_id: undefined as number | undefined,
  status: 0,
  display_time: 0,
  i18n: {} as PostLocaleForm,
});

const displayTimeDate = computed({
  get: () => (form.display_time > 0 ? new Date(form.display_time * 1000) : null),
  set: (v: Date | null) => {
    form.display_time = v ? Math.floor(v.getTime() / 1000) : 0;
  },
});

function emptyI18n(): PostLocaleForm {
  const map: PostLocaleForm = {};
  for (const loc of props.siteLocales) {
    map[loc] = { title: "", description: "", content: "", tagList: [], seoSlug: "" };
  }
  return map;
}

function hasLocaleContent(row: PostLocaleFormRow): boolean {
  return !!(
    row.title.trim()
    || row.description.trim()
    || row.content.trim()
    || row.seoSlug.trim()
    || row.tagList.length
  );
}

function resetForm() {
  form.category_id = undefined;
  form.status = 0;
  form.display_time = nowUnix();
  form.i18n = emptyI18n();
  sessionPostId.value = null;
  postCovers.value = [];
  postAttachments.value = [];
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
    form.status = detail.status;
    form.display_time = detail.display_time;
    postCovers.value = detail.covers ?? [];
    postAttachments.value = detail.attachments ?? [];
    for (const loc of props.siteLocales) {
      const tr = detail.translations[loc];
      if (tr) {
        form.i18n[loc] = {
          title: tr.title,
          description: tr.description,
          content: tr.content,
          tagList: parseTags(tr.tags),
          seoSlug: parseSeoSlug(loc, tr.route_path),
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

  if (!validateAllSeoSlugs()) {
    return;
  }

  saving.value = true;
  try {
    const metaPayload = {
      category_id: form.category_id,
      status: form.status,
      display_time: form.display_time || undefined,
    };

    const updateTargetId = props.editId ?? sessionPostId.value;

    if (updateTargetId === null) {
      const primary = form.i18n[props.defaultLocale];
      const createRes = await createPostApi({
        ...metaPayload,
        title: primary.title.trim(),
        description: primary.description.trim() || undefined,
        content: primary.content.trim() || undefined,
        route_path: buildRoutePath(props.defaultLocale, primary.seoSlug) || undefined,
        tags: serializeTags(primary.tagList) || undefined,
        lang: props.defaultLocale,
      });
      if (createRes.code !== 0 || !createRes.data) {
        koiMsgError(createRes.message || t("msg.fail"));
        return;
      }
      const newId = createRes.data.id;
      sessionPostId.value = newId;
      form.display_time = createRes.data.display_time;
      for (const loc of props.siteLocales) {
        if (loc === props.defaultLocale) continue;
        const row = form.i18n[loc];
        if (!hasLocaleContent(row)) continue;
        const res = await updatePostApi(newId, {
          title: row.title.trim(),
          description: row.description.trim() || undefined,
          content: row.content.trim() || undefined,
          route_path: buildRoutePath(loc, row.seoSlug) || undefined,
          tags: serializeTags(row.tagList),
          lang: loc,
        });
        if (res.code !== 0) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
      }
    } else {
      const metaRes = await updatePostApi(updateTargetId, metaPayload);
      if (metaRes.code !== 0) {
        koiMsgError(metaRes.message || t("msg.fail"));
        return;
      }
      for (const loc of props.siteLocales) {
        const row = form.i18n[loc];
        if (!hasLocaleContent(row)) continue;
        const res = await updatePostApi(updateTargetId, {
          title: row.title.trim(),
          description: row.description.trim() || undefined,
          content: row.content.trim() || undefined,
          route_path: buildRoutePath(loc, row.seoSlug) || undefined,
          tags: serializeTags(row.tagList),
          lang: loc,
        });
        if (res.code !== 0) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
      }
    }

    koiMsgSuccess(t("msg.success"));
    emit("saved");
    if (props.editId !== null) {
      visible.value = false;
    }
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

  &--stacked {
    :deep(.el-form-item) {
      margin-bottom: 16px;
    }
  }
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

.post-meta-row {
  width: 100%;

  :deep(.el-form-item) {
    margin-bottom: 18px;
  }
}

.post-drawer-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.post-url-block {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 10px;
  width: 100%;
}

.post-url-preview {
  padding: 6px 10px;
  border-radius: 6px;
  background: var(--el-fill-color-light);
  color: var(--el-text-color-primary);
  font-size: 13px;
  word-break: break-all;
}

.post-url-open {
  flex-shrink: 0;
}

.post-field-hint {
  margin: 6px 0 0;
  font-size: 12px;
  line-height: 1.5;
  color: var(--el-text-color-secondary);
}

.post-form-drawer__assets-note-item {
  margin-bottom: 4px;

  :deep(.el-form-item__label) {
    visibility: hidden;
  }
}

.post-form-drawer__assets-note {
  margin: 0;
  font-size: 12px;
  line-height: 1.5;
  color: var(--el-color-primary);
}

:deep(.el-input-group__prepend) {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  font-size: 13px;
  color: var(--el-text-color-secondary);
  user-select: all;
}
</style>
