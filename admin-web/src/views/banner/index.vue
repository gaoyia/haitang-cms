<template>
  <div class="banner-manage-page koi-page">
    <KoiCard>
      <template #header>
        <div class="page-head">
          <div>
            <div class="page-head__title">{{ t("menu.banner.manage.title") }}</div>
            <div class="page-head__desc">{{ t("menu.banner.manage.subtitle") }}</div>
          </div>
          <KoiToolbar :show-maximize="false" @refresh-table="refreshAll" />
        </div>
      </template>

      <div class="koi-split-layout">
        <!-- 左侧：轮播图组 -->
        <aside class="koi-split-layout__aside group-panel">
          <div class="group-panel__head">
            <span>{{ t("menu.banner.manage.groups") }}</span>
            <el-button type="primary" link @click="openGroupDialog(null)">
              <el-icon><Plus /></el-icon>
              {{ t("button.add") }}
            </el-button>
          </div>
          <el-scrollbar class="group-panel__list">
            <div
              v-for="group in groups"
              :key="group.id"
              class="group-item"
              :class="{ active: selectedGroupId === group.id }"
              @click="selectGroup(group.id)"
            >
              <div class="group-item__main">
                <div class="group-item__name">{{ group.name }}</div>
                <code class="group-item__code">{{ group.code }}</code>
                <el-tag
                  :type="group.status === 1 ? 'success' : 'danger'"
                  size="small"
                  effect="plain"
                  class="group-item__status"
                >
                  {{ group.status === 1 ? t("menu.menu.manage.enabled") : t("menu.menu.manage.disabled") }}
                </el-tag>
              </div>
              <div class="group-item__actions" @click.stop>
                <el-button type="primary" link @click="openGroupDialog(group)">{{ t("button.update") }}</el-button>
                <el-button type="danger" link @click="handleDeleteGroup(group)">{{ t("button.delete") }}</el-button>
              </div>
            </div>
            <el-empty v-if="!groupsLoading && groups.length === 0" :description="t('menu.banner.groupsEmpty')" />
          </el-scrollbar>
        </aside>

        <!-- 右侧：轮播图列表 -->
        <section class="koi-split-layout__main banner-panel">
          <div class="banner-panel__toolbar">
            <span v-if="currentGroup" class="banner-panel__hint">
              {{ t("menu.banner.manage.currentGroup", { name: currentGroup.name, code: currentGroup.code }) }}
            </span>
            <el-button type="primary" :disabled="selectedGroupId === null" @click="openBannerDialog(null)">
              <el-icon><Plus /></el-icon>
              {{ t("menu.banner.manage.addBanner") }}
            </el-button>
          </div>

          <div v-if="selectedGroupId === null && !bannersLoading" class="koi-split-layout__empty">
            <el-empty :description="t('menu.banner.manage.selectGroup')" />
          </div>

          <KoiTablePanel
            v-else
            v-model:page="page"
            v-model:page-size="pageSize"
            :loading="bannersLoading"
            :data="banners"
            :total="total"
            :empty-text="t('menu.banner.listEmpty')"
            stripe
            class="banner-table"
            @change="loadBanners"
          >
            <el-table-column type="index" :label="t('table.number')" width="60" />
            <el-table-column :label="t('menu.banner.image')" width="100" align="center">
              <template #default="{ row }">
                <el-image
                  v-if="row.image_url"
                  :src="resolveAssetUrl(row.image_url)"
                  :preview-src-list="[resolveAssetUrl(row.image_url)]"
                  fit="cover"
                  class="banner-thumb"
                />
                <span v-else class="text-muted">—</span>
              </template>
            </el-table-column>
            <el-table-column prop="title" :label="t('menu.banner.title')" min-width="160" />
            <el-table-column prop="link_url" :label="t('menu.banner.link')" min-width="160" show-overflow-tooltip />
            <el-table-column prop="sort" :label="t('menu.menu.manage.sort')" width="80" align="center" />
            <el-table-column :label="t('menu.menu.manage.status')" width="90" align="center">
              <template #default="{ row }">
                <el-tag :type="row.status === 1 ? 'success' : 'danger'" size="small" effect="plain">
                  {{ row.status === 1 ? t("menu.menu.manage.enabled") : t("menu.menu.manage.disabled") }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column :label="t('table.operate')" width="140" fixed="right">
              <template #default="{ row }">
                <el-button type="primary" link @click="openBannerDialog(row.id)">{{ t("button.update") }}</el-button>
                <el-button type="danger" link @click="handleDeleteBanner(row)">{{ t("button.delete") }}</el-button>
              </template>
            </el-table-column>
            <template #empty>
              <el-empty :description="t('menu.banner.listEmpty')">
                <el-button type="primary" @click="openBannerDialog(null)">{{ t("menu.banner.manage.addFirst") }}</el-button>
              </el-empty>
            </template>
          </KoiTablePanel>
        </section>
      </div>
    </KoiCard>

    <BannerGroupDialog v-model="groupDialogVisible" :edit-group="editingGroup" @saved="onGroupSaved" />

    <BannerItemDialog
      v-model="bannerDialogVisible"
      :edit-id="editingBannerId"
      :group-id="selectedGroupId ?? 0"
      :group-label="currentGroupLabel"
      @saved="loadBanners"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { Plus } from "@element-plus/icons-vue";
import { ElMessageBox } from "element-plus";
import {
  deleteBannerApi,
  deleteBannerGroupApi,
  listBannerGroupsApi,
  listBannersApi,
  type Banner,
  type BannerGroup,
} from "@/api/system/banners.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { useTablePage } from "@/composables/useTablePage.ts";
import BannerGroupDialog from "./components/BannerGroupDialog.vue";
import BannerItemDialog from "./components/BannerItemDialog.vue";

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const { page, pageSize, total, pageParams, applyPageResult, resetPage } = useTablePage();

const groups = ref<BannerGroup[]>([]);
const banners = ref<Banner[]>([]);
const selectedGroupId = ref<number | null>(null);
const groupsLoading = ref(false);
const bannersLoading = ref(false);

const groupDialogVisible = ref(false);
const editingGroup = ref<BannerGroup | null>(null);

const bannerDialogVisible = ref(false);
const editingBannerId = ref<number | null>(null);

const currentGroup = computed(() => groups.value.find((g) => g.id === selectedGroupId.value));

const currentGroupLabel = computed(() => {
  const g = currentGroup.value;
  return g ? `${g.name} (${g.code})` : "";
});

async function loadGroups() {
  groupsLoading.value = true;
  try {
    const res = await listBannerGroupsApi();
    groups.value = res.code === 0 && res.data ? res.data : [];

    const queryId = route.query.group_id ? Number(route.query.group_id) : null;
    if (queryId !== null && groups.value.some((g) => g.id === queryId)) {
      selectedGroupId.value = queryId;
    } else if (
      selectedGroupId.value === null ||
      !groups.value.some((g) => g.id === selectedGroupId.value)
    ) {
      selectedGroupId.value = groups.value[0]?.id ?? null;
    }
  } finally {
    groupsLoading.value = false;
  }
}

async function loadBanners() {
  if (selectedGroupId.value === null) {
    banners.value = [];
    total.value = 0;
    return;
  }
  bannersLoading.value = true;
  try {
    const res = await listBannersApi(selectedGroupId.value, pageParams.value);
    banners.value = applyPageResult(res.code === 0 ? res.data : null);
  } finally {
    bannersLoading.value = false;
  }
}

function selectGroup(id: number) {
  selectedGroupId.value = id;
  router.replace({ query: { ...route.query, group_id: String(id) } });
}

async function refreshAll() {
  await loadGroups();
  await loadBanners();
}

function openGroupDialog(group: BannerGroup | null) {
  editingGroup.value = group;
  groupDialogVisible.value = true;
}

async function onGroupSaved() {
  await loadGroups();
}

async function handleDeleteGroup(group: BannerGroup) {
  try {
    await ElMessageBox.confirm(
      t("menu.banner.groupDeleteConfirm", { name: group.name }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deleteBannerGroupApi(group.id);
  if (res.code === 0) {
    koiMsgSuccess(t("msg.success"));
    if (selectedGroupId.value === group.id) {
      selectedGroupId.value = null;
    }
    await loadGroups();
  } else {
    koiMsgError(res.message || t("msg.fail"));
  }
}

function openBannerDialog(id: number | null) {
  editingBannerId.value = id;
  bannerDialogVisible.value = true;
}

async function handleDeleteBanner(row: Banner) {
  try {
    await ElMessageBox.confirm(
      t("menu.banner.deleteConfirm", { name: row.title }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deleteBannerApi(row.id);
  if (res.code === 0) {
    koiMsgSuccess(t("msg.success"));
    await loadBanners();
  } else {
    koiMsgError(res.message || t("msg.fail"));
  }
}

watch(selectedGroupId, () => {
  resetPage();
  loadBanners();
});

onMounted(async () => {
  await loadGroups();
  await loadBanners();
});
</script>

<style scoped lang="scss">
.banner-manage-page {
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

.group-panel {
  display: flex;
  flex-direction: column;
  flex: 0 0 260px;
  min-height: 0;

  &__head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 14px;
    font-weight: 600;
    border-bottom: 1px solid var(--el-border-color-lighter);
    flex-shrink: 0;
  }

  &__list {
    flex: 1;
    min-height: 0;
    padding: 8px;
  }
}

.group-item {
  padding: 10px 12px;
  margin-bottom: 6px;
  border-radius: 8px;
  cursor: pointer;
  border: 1px solid transparent;
  transition: background 0.15s, border-color 0.15s;

  &:hover {
    background: var(--el-fill-color-light);
  }

  &.active {
    background: var(--el-color-primary-light-9);
    border-color: var(--el-color-primary-light-5);
  }

  &__name {
    font-size: 14px;
    font-weight: 500;
  }

  &__code {
    display: block;
    margin-top: 4px;
    font-size: 12px;
    color: var(--el-text-color-secondary);
  }

  &__status {
    margin-top: 6px;
  }

  &__actions {
    margin-top: 6px;
  }
}

.banner-panel {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;

  &__toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 12px;
    margin-bottom: 14px;
    flex-shrink: 0;
  }

  &__hint {
    font-size: 13px;
    color: var(--el-text-color-secondary);
  }
}

.banner-table {
  width: 100%;
}

.banner-thumb {
  width: 64px;
  height: 40px;
  border-radius: 4px;
}

.text-muted {
  color: var(--el-text-color-placeholder);
}
</style>
