<template>
  <div class="post-manage-page koi-page">
    <KoiCard>
      <template #header>
        <div class="post-page-head">
          <div>
            <div class="post-page-head__title">{{ t("menu.content.post.manage.title") }}</div>
            <div class="post-page-head__desc">{{ t("menu.content.post.manage.subtitle") }}</div>
          </div>
          <KoiToolbar :show-maximize="false" @refresh-table="loadPosts" />
        </div>
      </template>

      <div class="post-toolbar">
        <div class="post-toolbar__filters">
          <div class="post-toolbar__row">
            <span class="post-toolbar__label">{{ t("menu.content.post.manage.filterCategory") }}</span>
            <div class="post-category-segmented">
              <el-segmented v-model="filterCategoryId" :options="categorySegmentOptions" size="small" />
            </div>
          </div>
          <div class="post-toolbar__row">
            <span class="post-toolbar__label">{{ t("menu.content.post.manage.previewLang") }}</span>
            <el-segmented v-model="previewLang" :options="localeSegmentOptions" size="small" />
          </div>
        </div>
        <div class="post-toolbar__right">
          <el-button type="primary" @click="openDrawer(null)">
            <el-icon><Plus /></el-icon>
            {{ t("button.add") }}
          </el-button>
        </div>
      </div>

      <KoiTablePanel
        v-model:page="page"
        v-model:page-size="pageSize"
        :loading="loading"
        :data="posts"
        :total="total"
        stripe
        class="post-table"
        @change="loadPosts"
      >
        <el-table-column :label="t('menu.content.post.manage.titleCol')" min-width="200" show-overflow-tooltip>
          <template #default="{ row }">
            <div class="title-with-public-url">
              <PublicUrlPopover :id-url="postIdUrl(row.id)" :seo-url="postSeoUrl(row)" />
              <a
                :href="primaryPublicUrl(postIdUrl(row.id), postSeoUrl(row))"
                target="_blank"
                rel="noopener noreferrer"
                class="title-with-public-url__link"
              >
                {{ row.title }}
              </a>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('menu.content.post.manage.displayTime')" width="168" align="center">
          <template #default="{ row }">
            {{ formatUnixTime(row.display_time) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('menu.content.post.manage.pinned')" width="88" align="center">
          <template #default="{ row }">
            <el-tag v-if="row.pinned === 1" type="warning" size="small" effect="plain">
              {{ t("menu.content.post.manage.pinnedYes") }}
            </el-tag>
            <span v-else class="post-pinned-dash">—</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('menu.content.post.manage.publishTime')" width="168" align="center">
          <template #default="{ row }">
            {{ formatUnixTime(row.publish_time) }}
          </template>
        </el-table-column>
        <el-table-column prop="category_name" :label="t('menu.content.post.manage.category')" min-width="120" show-overflow-tooltip />
        <el-table-column :label="t('menu.content.post.manage.status')" width="108" align="center">
          <template #default="{ row }">
            <el-tag v-if="isScheduledPost(row)" type="warning" size="small" effect="plain">
              {{ t("menu.content.post.manage.scheduled") }}
            </el-tag>
            <el-tag v-else-if="row.status === 1" type="success" size="small" effect="plain">
              {{ t("menu.content.post.manage.published") }}
            </el-tag>
            <el-tag v-else type="info" size="small" effect="plain">
              {{ t("menu.content.post.manage.draft") }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('table.operate')" width="140" align="center" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" link @click="openDrawer(row.id)">
              {{ t("button.update") }}
            </el-button>
            <el-button type="danger" link @click="handleDelete(row)">
              {{ t("button.delete") }}
            </el-button>
          </template>
        </el-table-column>
      </KoiTablePanel>
    </KoiCard>

    <PostFormDrawer
      v-model="drawerVisible"
      :edit-id="editingId"
      :site-locales="siteLocales"
      :default-locale="defaultLocale"
      :categories="categories"
      :locale-label="localeLabel"
      @saved="loadPosts"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { Plus } from "@element-plus/icons-vue";
import { ElMessageBox } from "element-plus";
import { deletePostApi, listPostsApi, type PostView } from "@/api/system/posts.ts";
import { listCategoriesApi, type CategoryView } from "@/api/system/categories.ts";
import { useSiteLocales } from "@/composables/useSiteLocales.ts";
import { useTablePage } from "@/composables/useTablePage.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import { formatUnixTime, nowUnix } from "@/utils/formatTime.ts";
import { primaryPublicUrl } from "@/utils/publicUrl.ts";
import PostFormDrawer from "./components/PostFormDrawer.vue";
import PublicUrlPopover from "@/components/PublicUrlPopover.vue";

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const { siteLocales, defaultLocale, previewLang, loadSiteLocales, localeLabel } = useSiteLocales();
const { page, pageSize, total, pageParams, applyPageResult, resetPage } = useTablePage();

const posts = ref<PostView[]>([]);
const categories = ref<CategoryView[]>([]);
const loading = ref(false);
const drawerVisible = ref(false);
const editingId = ref<number | null>(null);
/** 0 表示全部分类 */
const filterCategoryId = ref(0);
/** 避免 URL 回写与 router.replace 互相触发 */
const syncingFromRoute = ref(false);

const localeSegmentOptions = computed(() =>
  siteLocales.value.map((loc) => ({
    label: localeLabel(loc),
    value: loc,
  })),
);

const categorySegmentOptions = computed(() => [
  { label: t("menu.content.post.manage.filterCategoryAll"), value: 0 },
  ...categories.value.map((cat) => ({
    label: cat.name,
    value: cat.id,
  })),
]);

const listQueryParams = computed(() => ({
  ...pageParams.value,
  category_id: filterCategoryId.value > 0 ? filterCategoryId.value : undefined,
}));

function postListQueryFromState(): Record<string, string> {
  const query: Record<string, string> = { lang: previewLang.value };
  if (filterCategoryId.value > 0) {
    query.category_id = String(filterCategoryId.value);
  }
  if (page.value > 1) {
    query.page = String(page.value);
  }
  if (pageSize.value !== 10) {
    query.page_size = String(pageSize.value);
  }
  return query;
}

function queryMatchesRoute(next: Record<string, string>): boolean {
  const cur = route.query;
  const curLang = typeof cur.lang === "string" ? cur.lang : defaultLocale.value;
  const curCategory = cur.category_id ? String(cur.category_id) : "";
  const curPage = cur.page ? String(cur.page) : "1";
  const curPageSize = cur.page_size ? String(cur.page_size) : "10";
  return (
    curLang === next.lang
    && curCategory === (next.category_id ?? "")
    && curPage === (next.page ?? "1")
    && curPageSize === (next.page_size ?? "10")
  );
}

function applyQueryFromRoute() {
  const q = route.query;
  const lang = typeof q.lang === "string" ? q.lang : "";
  if (lang && siteLocales.value.includes(lang)) {
    previewLang.value = lang;
  }

  const rawCategory = q.category_id;
  const categoryId =
    rawCategory != null && rawCategory !== "" ? Number(rawCategory) : 0;
  filterCategoryId.value =
    Number.isFinite(categoryId) && categoryId >= 0 ? categoryId : 0;

  const rawPage = q.page ? Number(q.page) : 1;
  page.value = Number.isFinite(rawPage) && rawPage >= 1 ? rawPage : 1;

  const rawPageSize = q.page_size ? Number(q.page_size) : 10;
  pageSize.value = Number.isFinite(rawPageSize) && rawPageSize >= 1 ? rawPageSize : 10;
}

function syncRouteQuery() {
  if (syncingFromRoute.value) return;
  const next = postListQueryFromState();
  if (queryMatchesRoute(next)) return;
  syncingFromRoute.value = true;
  router.replace({ query: next });
  nextTick(() => {
    syncingFromRoute.value = false;
  });
}

function postIdUrl(id: number): string {
  return `/${previewLang.value}/posts/${id}`;
}

function postSeoUrl(row: PostView): string | null {
  const path = row.route_path?.trim();
  return path || null;
}

function isScheduledPost(row: PostView): boolean {
  return row.status === 1 && row.publish_time > nowUnix();
}

async function loadCategories() {
  const res = await listCategoriesApi(previewLang.value, { page: 1, page_size: 500 });
  categories.value = res.code === 0 && res.data ? res.data.list : [];
  if (
    filterCategoryId.value > 0
    && !categories.value.some((cat) => cat.id === filterCategoryId.value)
  ) {
    filterCategoryId.value = 0;
    syncRouteQuery();
  }
}

async function loadPosts() {
  loading.value = true;
  try {
    const res = await listPostsApi(previewLang.value, listQueryParams.value);
    posts.value = applyPageResult(res.code === 0 ? res.data : null);
    syncRouteQuery();
  } finally {
    loading.value = false;
  }
}

function openDrawer(id: number | null) {
  editingId.value = id;
  drawerVisible.value = true;
}

async function handleDelete(row: PostView) {
  try {
    await ElMessageBox.confirm(
      t("menu.content.post.manage.deleteConfirm", { name: row.title }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deletePostApi(row.id);
  if (res.code === 0) {
    koiMsgSuccess(t("msg.success"));
    await loadPosts();
  } else {
    koiMsgError(res.message || t("msg.fail"));
  }
}

watch(previewLang, async () => {
  if (syncingFromRoute.value) return;
  resetPage();
  syncRouteQuery();
  await loadCategories();
  await loadPosts();
});

watch(filterCategoryId, async () => {
  if (syncingFromRoute.value) return;
  resetPage();
  syncRouteQuery();
  await loadPosts();
});

watch(
  () => route.query,
  async () => {
    if (syncingFromRoute.value) return;
    syncingFromRoute.value = true;
    applyQueryFromRoute();
    await loadCategories();
    await loadPosts();
    nextTick(() => {
      syncingFromRoute.value = false;
    });
  },
);

onMounted(async () => {
  await loadSiteLocales();
  syncingFromRoute.value = true;
  applyQueryFromRoute();
  await loadCategories();
  await loadPosts();
  nextTick(() => {
    syncingFromRoute.value = false;
  });
});
</script>

<style scoped lang="scss">
.post-page-head {
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

.post-toolbar {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 14px;
  flex-shrink: 0;

  &__filters {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  &__row {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  &__label {
    font-size: 13px;
    color: var(--el-text-color-secondary);
    flex-shrink: 0;
  }
}

.post-category-segmented {
  flex: 1;
  min-width: 0;
  overflow-x: auto;
  overflow-y: hidden;
  padding-bottom: 2px;

  :deep(.el-segmented) {
    width: max-content;
    max-width: none;
  }
}

.post-table {
  width: 100%;
}

.post-pinned-dash {
  color: var(--el-text-color-placeholder);
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
</style>
