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

      <SplitGroupMobileBar
        v-if="isMobile"
        :model-value="selectedGroupId"
        :groups="groups"
        :placeholder="t('menu.banner.manage.selectGroup')"
        @update:model-value="selectGroup"
        @add="openGroupDialog(null)"
        @edit="openGroupDialog(currentGroup!)"
        @delete="handleDeleteGroup(currentGroup!)"
      />

      <div class="koi-split-layout" :class="{ 'koi-split-layout--mobile': isMobile }">
        <!-- 左侧：轮播图组（桌面端） -->
        <aside v-if="!isMobile" class="koi-split-layout__aside group-panel">
          <div class="group-panel__head">
            <span>{{ t("menu.banner.manage.groups") }}</span>
            <el-button type="primary" link @click="openGroupDialog(null)">
              <el-icon><Plus /></el-icon>
              {{ t("button.add") }}
            </el-button>
          </div>
          <el-scrollbar class="group-panel__list">
            <div ref="groupListRef" class="group-panel__sortable">
            <div
              v-for="group in groups"
              :key="group.id"
              class="group-item"
              :class="{ active: selectedGroupId === group.id }"
              @click="selectGroup(group.id)"
            >
              <el-icon class="group-item__drag" :title="t('menu.banner.manage.dragSort')" @click.stop>
                <Rank />
              </el-icon>
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
            <el-button
              v-if="!isMobile"
              type="primary"
              :disabled="selectedGroupId === null"
              @click="openBannerDialog(null)"
            >
              <el-icon><Plus /></el-icon>
              {{ t("menu.banner.manage.addBanner") }}
            </el-button>
            <ActionsDropdown
              v-else
              :items="bannerToolbarActions"
              :label="t('table.operate')"
              :link="false"
              size="default"
              :disabled="selectedGroupId === null"
              @action="handleBannerToolbarAction"
            />
          </div>

          <div v-if="selectedGroupId === null && !bannersLoading" class="koi-split-layout__empty">
            <el-empty :description="t('menu.banner.manage.selectGroup')" />
          </div>

          <div v-else ref="bannerTableWrapRef" class="banner-table-wrap">
          <KoiTablePanel
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
            <el-table-column width="44" align="center" class-name="banner-drag-col">
              <template #default>
                <el-icon class="banner-drag-handle" :title="t('menu.banner.manage.dragSort')">
                  <Rank />
                </el-icon>
              </template>
            </el-table-column>
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
            <el-table-column :label="t('menu.menu.manage.status')" width="90" align="center">
              <template #default="{ row }">
                <el-tag :type="row.status === 1 ? 'success' : 'danger'" size="small" effect="plain">
                  {{ row.status === 1 ? t("menu.menu.manage.enabled") : t("menu.menu.manage.disabled") }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column :label="t('table.operate')" :width="isMobile ? 100 : 140" align="center" fixed="right">
              <template #default="{ row }">
                <ActionsDropdown
                  v-if="isMobile"
                  :items="bannerRowActions"
                  @action="(cmd) => handleBannerRowAction(cmd, row)"
                />
                <template v-else>
                  <el-button type="primary" link @click="openBannerDialog(row.id)">{{ t("button.update") }}</el-button>
                  <el-button type="danger" link @click="handleDeleteBanner(row)">{{ t("button.delete") }}</el-button>
                </template>
              </template>
            </el-table-column>
            <template #empty>
              <el-empty :description="t('menu.banner.listEmpty')">
                <el-button type="primary" @click="openBannerDialog(null)">{{ t("menu.banner.manage.addFirst") }}</el-button>
              </el-empty>
            </template>
          </KoiTablePanel>
          </div>
        </section>
      </div>
    </KoiCard>

    <BannerGroupDrawer
      v-model="groupDialogVisible"
      :edit-group="editingGroup"
      :default-sort="nextGroupSort"
      @saved="onGroupSaved"
    />

    <BannerItemDrawer
      v-model="bannerDialogVisible"
      :edit-id="editingBannerId"
      :group-id="selectedGroupId ?? 0"
      :group-label="currentGroupLabel"
      :default-sort="nextBannerSort"
      @saved="loadBanners"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { Plus, Rank } from "@element-plus/icons-vue";
import { ElMessageBox } from "element-plus";
import {
  deleteBannerApi,
  deleteBannerGroupApi,
  listBannerGroupsApi,
  listBannersApi,
  updateBannerApi,
  updateBannerGroupApi,
  type Banner,
  type BannerGroup,
} from "@/api/system/banners.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { useTablePage } from "@/composables/useTablePage.ts";
import { useSortableList } from "@/composables/useSortableList.ts";
import { buildSortUpdates, diffSortUpdates, moveArrayItem, nextSortValue } from "@/utils/sortOrder.ts";
import BannerGroupDrawer from "./components/BannerGroupDrawer.vue";
import BannerItemDrawer from "./components/BannerItemDrawer.vue";
import SplitGroupMobileBar from "@/components/SplitGroupMobileBar.vue";
import ActionsDropdown, { type ActionDropdownItem } from "@/components/ActionsDropdown.vue";
import { useScreenStore } from "@/hooks/screen/index.ts";

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const { isMobile } = useScreenStore();
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

const groupListRef = ref<HTMLElement | null>(null);
const bannerTableWrapRef = ref<HTMLElement | null>(null);
const sortSaving = ref(false);

const nextGroupSort = computed(() => nextSortValue(groups.value));
const nextBannerSort = computed(() => nextSortValue(banners.value));

const groupSortDisabled = computed(() => isMobile.value || groups.value.length < 2);
const bannerSortDisabled = computed(
  () => isMobile.value || selectedGroupId.value === null || banners.value.length < 2,
);

useSortableList({
  getContainer: () => groupListRef.value,
  draggable: ".group-item",
  handle: ".group-item__drag",
  disabled: groupSortDisabled,
  getLength: () => groups.value.length,
  onReorder: onGroupReorder,
});

useSortableList({
  getContainer: () =>
    bannerTableWrapRef.value?.querySelector(".el-table__body-wrapper tbody") as HTMLElement | null,
  draggable: "tr",
  handle: ".banner-drag-handle",
  disabled: bannerSortDisabled,
  getLength: () => banners.value.length,
  onReorder: onBannerReorder,
});

const bannerToolbarActions = computed<ActionDropdownItem[]>(() => [
  { key: "add", label: t("menu.banner.manage.addBanner") },
]);

const bannerRowActions = computed<ActionDropdownItem[]>(() => [
  { key: "edit", label: t("button.update") },
  { key: "delete", label: t("button.delete"), danger: true },
]);

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

async function onGroupReorder(oldIndex: number, newIndex: number) {
  const snapshot = groups.value.map((group) => ({ id: group.id, sort: group.sort }));
  groups.value = moveArrayItem(groups.value, oldIndex, newIndex);

  const changed = diffSortUpdates(buildSortUpdates(groups.value), snapshot);
  if (changed.length === 0) return;

  sortSaving.value = true;
  try {
    for (const update of changed) {
      const res = await updateBannerGroupApi(update.id, { sort: update.sort });
      if (res.code !== 0) {
        koiMsgError(res.message || t("msg.fail"));
        await loadGroups();
        return;
      }
    }
    koiMsgSuccess(t("msg.success"));
  } finally {
    sortSaving.value = false;
  }
}

async function onBannerReorder(oldIndex: number, newIndex: number) {
  const snapshot = banners.value.map((banner) => ({ id: banner.id, sort: banner.sort }));
  banners.value = moveArrayItem(banners.value, oldIndex, newIndex);

  const offset = (page.value - 1) * pageSize.value;
  const changed = diffSortUpdates(buildSortUpdates(banners.value, offset), snapshot);
  if (changed.length === 0) return;

  sortSaving.value = true;
  try {
    for (const update of changed) {
      const res = await updateBannerApi(update.id, { sort: update.sort });
      if (res.code !== 0) {
        koiMsgError(res.message || t("msg.fail"));
        await loadBanners();
        return;
      }
    }
    koiMsgSuccess(t("msg.success"));
  } finally {
    sortSaving.value = false;
  }
}

function handleBannerToolbarAction(cmd: string) {
  if (cmd === "add") openBannerDialog(null);
}

function handleBannerRowAction(cmd: string, row: Banner) {
  if (cmd === "edit") openBannerDialog(row.id);
  else if (cmd === "delete") handleDeleteBanner(row);
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

  &__sortable {
    display: flex;
    flex-direction: column;
  }
}

.group-item {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  padding: 10px 12px;
  margin-bottom: 6px;
  border-radius: 8px;
  cursor: pointer;
  border: 1px solid transparent;
  transition: background 0.15s, border-color 0.15s;

  &__drag {
    flex-shrink: 0;
    margin-top: 2px;
    cursor: grab;
    color: var(--el-text-color-placeholder);

    &:active {
      cursor: grabbing;
    }
  }

  &__main {
    flex: 1;
    min-width: 0;
  }

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

.banner-table-wrap {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
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

.banner-drag-handle {
  cursor: grab;
  color: var(--el-text-color-placeholder);

  &:active {
    cursor: grabbing;
  }
}

:global(.sortable-ghost) {
  opacity: 0.45;
}
</style>
