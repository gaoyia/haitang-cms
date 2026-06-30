<template>
  <div class="friend-link-page koi-page">
    <KoiCard>
      <template #header>
        <div class="page-head">
          <div>
            <div class="page-head__title">{{ t("menu.friendLink.manage.title") }}</div>
            <div class="page-head__desc">{{ t("menu.friendLink.manage.subtitle") }}</div>
          </div>
          <KoiToolbar :show-maximize="false" @refresh-table="loadLinks" />
        </div>
      </template>

      <div class="page-toolbar">
        <el-button type="primary" @click="openDialog(null)">
          <el-icon><Plus /></el-icon>
          {{ t("button.add") }}
        </el-button>
      </div>

      <div ref="tableWrapRef" class="friend-link-table-wrap">
        <KoiTablePanel
          v-model:page="page"
          v-model:page-size="pageSize"
          :loading="loading"
          :data="links"
          :total="total"
          :empty-text="t('menu.friendLink.listEmpty')"
          stripe
          class="friend-link-table"
          @change="loadLinks"
        >
          <el-table-column width="44" align="center" class-name="friend-link-drag-col">
            <template #default>
              <el-icon class="friend-link-drag-handle" :title="t('menu.friendLink.manage.dragSort')">
                <Rank />
              </el-icon>
            </template>
          </el-table-column>
          <el-table-column :label="t('menu.friendLink.image')" width="220" align="center">
            <template #default="{ row }">
              <el-image
                v-if="row.image_url"
                :src="resolveAssetUrl(row.image_url)"
                :preview-src-list="[resolveAssetUrl(row.image_url)]"
                fit="contain"
                class="friend-link-thumb"
              />
              <span v-else class="text-muted">—</span>
            </template>
          </el-table-column>
          <el-table-column prop="title" :label="t('menu.friendLink.title')" min-width="140" />
          <el-table-column prop="url" :label="t('menu.friendLink.url')" min-width="200" show-overflow-tooltip />
          <el-table-column prop="sort" :label="t('menu.friendLink.sort')" width="90" align="center" />
          <el-table-column :label="t('menu.menu.manage.status')" width="90" align="center">
            <template #default="{ row }">
              <el-tag :type="row.status === 1 ? 'success' : 'danger'" size="small" effect="plain">
                {{ row.status === 1 ? t("menu.menu.manage.enabled") : t("menu.menu.manage.disabled") }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column :label="t('table.operate')" width="140" align="center" fixed="right">
            <template #default="{ row }">
              <el-button type="primary" link @click="openDialog(row.id)">{{ t("button.update") }}</el-button>
              <el-button type="danger" link @click="handleDelete(row)">{{ t("button.delete") }}</el-button>
            </template>
          </el-table-column>
        </KoiTablePanel>
      </div>
    </KoiCard>

    <FriendLinkDialog
      v-model="dialogVisible"
      :edit-id="editingId"
      :default-sort="nextSort"
      @saved="loadLinks"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { Plus, Rank } from "@element-plus/icons-vue";
import { ElMessageBox } from "element-plus";
import {
  deleteFriendLinkApi,
  listFriendLinksApi,
  updateFriendLinkApi,
  type FriendLink,
} from "@/api/system/friendLinks.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { useTablePage } from "@/composables/useTablePage.ts";
import { useSortableList } from "@/composables/useSortableList.ts";
import {
  applySortUpdates,
  buildSortUpdates,
  diffSortUpdates,
  moveArrayItem,
  nextSortValue,
} from "@/utils/sortOrder.ts";
import FriendLinkDialog from "./components/FriendLinkDialog.vue";

const { t } = useI18n();
const { page, pageSize, total, pageParams, applyPageResult } = useTablePage();

const loading = ref(false);
const links = ref<FriendLink[]>([]);
const dialogVisible = ref(false);
const editingId = ref<number | null>(null);
const tableWrapRef = ref<HTMLElement | null>(null);
const sortSaving = ref(false);

const SORT_STEP = 10;

const nextSort = computed(() => nextSortValue(links.value, SORT_STEP));
const sortDisabled = computed(() => links.value.length < 2);

useSortableList({
  getContainer: () =>
    tableWrapRef.value?.querySelector(".el-table__body-wrapper tbody") as HTMLElement | null,
  draggable: "tr",
  handle: ".friend-link-drag-handle",
  disabled: sortDisabled,
  getLength: () => links.value.length,
  onReorder: onReorder,
});

async function loadLinks() {
  loading.value = true;
  try {
    const res = await listFriendLinksApi(pageParams.value);
    links.value = applyPageResult(res.code === 0 ? res.data : null);
  } finally {
    loading.value = false;
  }
}

function openDialog(id: number | null) {
  editingId.value = id;
  dialogVisible.value = true;
}

async function handleDelete(row: FriendLink) {
  try {
    await ElMessageBox.confirm(
      t("menu.friendLink.deleteConfirm", { name: row.title }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deleteFriendLinkApi(row.id);
  if (res.code === 0) {
    koiMsgSuccess(t("msg.success"));
    await loadLinks();
  } else {
    koiMsgError(res.message || t("msg.fail"));
  }
}

async function onReorder(oldIndex: number, newIndex: number) {
  const snapshot = links.value.map((item) => ({ id: item.id, sort: item.sort }));
  links.value = moveArrayItem(links.value, oldIndex, newIndex);

  const offset = (page.value - 1) * pageSize.value;
  const updates = buildSortUpdates(links.value, offset, SORT_STEP);
  links.value = applySortUpdates(links.value, updates);

  const changed = diffSortUpdates(updates, snapshot);
  if (changed.length === 0) return;

  sortSaving.value = true;
  try {
    for (const update of changed) {
      const res = await updateFriendLinkApi(
        update.id,
        { sort: update.sort },
        { throttle: false },
      );
      if (res.code !== 0) {
        koiMsgError(res.message || t("msg.fail"));
        await loadLinks();
        return;
      }
    }
    koiMsgSuccess(t("msg.success"));
    await loadLinks();
  } finally {
    sortSaving.value = false;
  }
}

onMounted(loadLinks);
</script>

<style scoped lang="scss">
.friend-link-page {
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
  display: flex;
  justify-content: flex-end;
  margin-bottom: 14px;
}

.friend-link-table-wrap {
  min-height: 0;
}

.friend-link-thumb {
  height: 40px;
  max-width: 184px;
  border-radius: 4px;
}

.text-muted {
  color: var(--el-text-color-placeholder);
}

.friend-link-drag-handle {
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
