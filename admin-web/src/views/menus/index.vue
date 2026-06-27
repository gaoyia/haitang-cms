<template>
  <div class="menu-manage-page koi-page">
    <KoiCard>
      <template #header>
        <div class="menu-page-head">
          <div>
            <div class="menu-page-head__title">{{ t("menu.menu.manage.title") }}</div>
            <div class="menu-page-head__desc">{{ t("menu.menu.manage.subtitle") }}</div>
          </div>
          <KoiToolbar :show-maximize="false" @refresh-table="refreshAll" />
        </div>
      </template>

      <div class="koi-split-layout">
        <!-- 左侧：菜单组 -->
        <aside class="koi-split-layout__aside menu-group-panel">
          <div class="menu-group-panel__head">
            <span>{{ t("menu.menu.manage.groups") }}</span>
            <el-button type="primary" link @click="openGroupDialog(null)">
              <el-icon><Plus /></el-icon>
              {{ t("button.add") }}
            </el-button>
          </div>
          <el-scrollbar class="menu-group-panel__list">
            <el-empty v-if="groups.length === 0" :description="t('menu.menu.manage.groupsEmpty')" />
            <div
              v-for="group in groups"
              :key="group.id"
              class="menu-group-item"
              :class="{ active: selectedGroupId === group.id }"
              @click="selectGroup(group.id)"
            >
              <div class="menu-group-item__main">
                <div class="menu-group-item__name">
                  {{ group.name }}
                  <el-tag v-if="group.readonly" size="small" type="info" effect="plain">
                    {{ t("menu.menu.manage.readonly") }}
                  </el-tag>
                </div>
                <code class="menu-group-item__code">{{ group.code }}</code>
              </div>
              <div v-if="!group.readonly" class="menu-group-item__actions" @click.stop>
                <el-button type="primary" link @click="openGroupDialog(group)">
                  {{ t("button.update") }}
                </el-button>
                <el-button type="danger" link @click="handleDeleteGroup(group)">
                  {{ t("button.delete") }}
                </el-button>
              </div>
            </div>
          </el-scrollbar>
        </aside>

        <!-- 右侧：菜单树 -->
        <section class="koi-split-layout__main menu-tree-panel" v-loading="treeLoading">
          <div class="menu-tree-toolbar">
            <div class="menu-tree-toolbar__left">
              <span class="menu-tree-toolbar__label">{{ t("menu.menu.manage.previewLang") }}</span>
              <el-segmented v-model="previewLang" :options="localeSegmentOptions" size="small" />
            </div>
            <div class="menu-tree-toolbar__right">
              <el-button
                v-if="!currentGroup?.readonly"
                type="primary"
                :disabled="selectedGroupId === null"
                @click="openItemDrawer(null, 0)"
              >
                <el-icon><Plus /></el-icon>
                {{ t("menu.menu.manage.addRoot") }}
              </el-button>
            </div>
          </div>

          <el-alert
            v-if="currentGroup?.readonly"
            type="warning"
            :closable="false"
            show-icon
            class="menu-tree-readonly-alert"
            :title="t('menu.menu.manage.sidebarReadonly')"
          />

          <div v-if="selectedGroupId === null && !treeLoading" class="koi-split-layout__empty">
            <el-empty :description="t('menu.menu.manage.selectGroup')" />
          </div>

          <el-empty
            v-else-if="!treeLoading && selectedGroupId !== null && menuTree.length === 0"
            class="koi-split-layout__empty"
            :description="t('menu.menu.manage.emptyTree')"
          >
            <el-button v-if="!currentGroup?.readonly" type="primary" @click="openItemDrawer(null, 0)">
              {{ t("menu.menu.manage.addFirst") }}
            </el-button>
          </el-empty>

          <div v-else-if="menuTree.length > 0" class="menu-tree-panel__body">
          <el-tree
            :data="menuTree"
            node-key="id"
            default-expand-all
            :expand-on-click-node="false"
            class="menu-el-tree"
          >
            <template #default="{ data }">
              <div class="menu-tree-node">
                <span class="menu-tree-node__icon">
                  <KoiGlobalIcon v-if="isKoiIcon(data.icon)" :name="data.icon" size="16" />
                  <img v-else-if="isImgIcon(data.icon)" :src="data.icon" alt="" class="menu-tree-node__img" />
                  <span v-else-if="data.icon">{{ data.icon }}</span>
                  <el-icon v-else><Document /></el-icon>
                </span>
                <span class="menu-tree-node__title">{{ data.title || t("menu.menu.manage.untitled") }}</span>
                <el-tag v-if="!data.path" size="small" type="info" effect="plain">
                  {{ t("button.catalog") }}
                </el-tag>
                <el-tag v-if="data.status === 0" size="small" type="danger" effect="plain">
                  {{ t("menu.menu.manage.disabled") }}
                </el-tag>
                <span v-if="data.path" class="menu-tree-node__path">{{ data.path }}</span>
                <span v-if="data.permission" class="menu-tree-node__perm">{{ data.permission }}</span>
                <span class="menu-tree-node__actions">
                  <template v-if="!currentGroup?.readonly">
                    <el-button type="primary" link size="small" @click.stop="openItemDrawer(null, data.id)">
                      {{ t("menu.menu.manage.addChild") }}
                    </el-button>
                    <el-button type="primary" link size="small" @click.stop="openItemDrawer(data.id, 0)">
                      {{ t("button.update") }}
                    </el-button>
                    <el-button type="danger" link size="small" @click.stop="handleDeleteItem(data)">
                      {{ t("button.delete") }}
                    </el-button>
                  </template>
                  <el-button v-else type="primary" link size="small" @click.stop="openItemDrawer(data.id, 0)">
                    {{ t("button.view") }}
                  </el-button>
                </span>
              </div>
            </template>
          </el-tree>
          </div>
        </section>
      </div>
    </KoiCard>

    <MenuGroupDialog v-model="groupDialogVisible" :edit-group="editingGroup" @saved="onGroupSaved" />

    <MenuItemDrawer
      v-model="itemDrawerVisible"
      :edit-id="editingItemId"
      :group-id="selectedGroupId ?? 0"
      :group-code="currentGroup?.code ?? ''"
      :group-name="currentGroup?.name ?? ''"
      :readonly="currentGroup?.readonly ?? false"
      :menus="menuTree"
      :preset-parent-id="presetParentId"
      :site-locales="siteLocales"
      :default-locale="defaultLocale"
      :permission-groups="permissionGroups"
      @saved="loadMenus"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { Document, Plus } from "@element-plus/icons-vue";
import { ElMessageBox } from "element-plus";
import {
  deleteMenuApi,
  deleteMenuGroupApi,
  listMenuGroupsApi,
  listMenusApi,
  listPermissionsApi,
  type MenuGroup,
  type MenuItem,
  type PermissionGroup,
} from "@/api/system/menus.ts";
import { getDictMapApi } from "@/api/system/dict.ts";
import { uiLangToApiLocale } from "@/utils/apiLocale.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import useGlobalStore from "@/stores/modules/global.ts";
import MenuGroupDialog from "./components/MenuGroupDialog.vue";
import MenuItemDrawer from "./components/MenuItemDrawer.vue";
import KoiGlobalIcon from "@/components/KoiGlobalIcon/Index.vue";

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const globalStore = useGlobalStore();

const groups = ref<MenuGroup[]>([]);
const menuTree = ref<MenuItem[]>([]);
const selectedGroupId = ref<number | null>(null);
const treeLoading = ref(false);
const previewLang = ref("zh-cn");
const siteLocales = ref<string[]>(["zh-cn", "en-us"]);
const defaultLocale = ref("zh-cn");
const permissionGroups = ref<PermissionGroup[]>([]);

const groupDialogVisible = ref(false);
const editingGroup = ref<MenuGroup | null>(null);

const itemDrawerVisible = ref(false);
const editingItemId = ref<number | null>(null);
const presetParentId = ref(0);

const currentGroup = computed(() => groups.value.find((g) => g.id === selectedGroupId.value));

const localeSegmentOptions = computed(() =>
  siteLocales.value.map((loc) => ({
    label: loc === "en-us" ? "EN" : loc === "zh-cn" ? "中文" : loc,
    value: loc,
  })),
);

function isKoiIcon(icon: string): boolean {
  return !!icon && !icon.startsWith("http") && icon.includes("-");
}

function isImgIcon(icon: string): boolean {
  return icon.startsWith("http://") || icon.startsWith("https://");
}

async function loadSiteLocales() {
  const lang = uiLangToApiLocale(globalStore.language);
  const res = await getDictMapApi(lang);
  if (res.code === 0 && res.data) {
    defaultLocale.value = res.data.site_default_locale?.trim() || "zh-cn";
    const raw = res.data.site_locales?.trim();
    if (raw) {
      siteLocales.value = raw.split(",").map((s) => uiLangToApiLocale(s.trim()));
    }
    if (!siteLocales.value.includes(defaultLocale.value)) {
      siteLocales.value = [defaultLocale.value, ...siteLocales.value];
    }
    previewLang.value = defaultLocale.value;
  }
}

async function loadPermissions() {
  const res = await listPermissionsApi();
  if (res.code === 0 && Array.isArray(res.data)) {
    permissionGroups.value = res.data as PermissionGroup[];
  }
}

async function loadGroups() {
  const res = await listMenuGroupsApi();
  if (res.code !== 0 || !res.data) return;
  groups.value = res.data;

  const queryId = route.query.group_id ? Number(route.query.group_id) : null;
  if (queryId !== null && res.data.some((g) => g.id === queryId)) {
    selectedGroupId.value = queryId;
  } else if (selectedGroupId.value === null || !res.data.some((g) => g.id === selectedGroupId.value)) {
    const fallback = res.data.find((g) => !g.readonly) ?? res.data[0];
    selectedGroupId.value = fallback?.id ?? null;
  }
}

async function loadMenus() {
  if (selectedGroupId.value === null) {
    menuTree.value = [];
    return;
  }
  treeLoading.value = true;
  try {
    const res = await listMenusApi(selectedGroupId.value, previewLang.value);
    menuTree.value = res.code === 0 && res.data ? res.data : [];
  } finally {
    treeLoading.value = false;
  }
}

function selectGroup(id: number) {
  selectedGroupId.value = id;
  router.replace({ query: { ...route.query, group_id: String(id) } });
}

async function refreshAll() {
  await loadGroups();
  await loadMenus();
}

function openGroupDialog(group: MenuGroup | null) {
  editingGroup.value = group;
  groupDialogVisible.value = true;
}

async function onGroupSaved() {
  await loadGroups();
}

async function handleDeleteGroup(group: MenuGroup) {
  if (group.readonly) return;
  try {
    await ElMessageBox.confirm(
      t("menu.menu.manage.groupDeleteConfirm", { name: group.name }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deleteMenuGroupApi(group.id);
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

function openItemDrawer(editId: number | null, parentId: number) {
  editingItemId.value = editId;
  presetParentId.value = parentId;
  itemDrawerVisible.value = true;
}

async function handleDeleteItem(item: MenuItem) {
  try {
    await ElMessageBox.confirm(
      t("menu.menu.manage.itemDeleteConfirm", { name: item.title }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deleteMenuApi(item.id);
  if (res.code === 0) {
    koiMsgSuccess(t("msg.success"));
    await loadMenus();
  } else {
    koiMsgError(res.message || t("msg.fail"));
  }
}

watch(selectedGroupId, loadMenus);
watch(previewLang, loadMenus);

onMounted(async () => {
  await loadSiteLocales();
  await loadPermissions();
  await loadGroups();
  await loadMenus();
});
</script>

<style scoped lang="scss">
.menu-manage-page {
  height: 100%;
}

.menu-page-head {
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

.menu-group-panel {
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

.menu-group-item {
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
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
    font-weight: 500;
  }

  &__code {
    display: block;
    margin-top: 4px;
    font-size: 12px;
    color: var(--el-text-color-secondary);
  }

  &__actions {
    margin-top: 6px;
  }
}

.menu-tree-panel {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;

  &__body {
    flex: 1;
    min-height: 0;
    overflow: auto;
  }
}

.menu-tree-toolbar {
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

  &__label {
    font-size: 13px;
    color: var(--el-text-color-secondary);
  }
}

.menu-tree-readonly-alert {
  margin-bottom: 12px;
}

.menu-el-tree {
  --el-tree-node-content-height: 40px;
}

.menu-tree-node {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 0;
  padding-right: 8px;

  &__icon {
    flex-shrink: 0;
    width: 20px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  &__img {
    width: 16px;
    height: 16px;
    object-fit: contain;
  }

  &__title {
    font-weight: 500;
    max-width: 160px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  &__path {
    font-size: 12px;
    color: var(--el-text-color-secondary);
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  &__perm {
    font-size: 11px;
    color: var(--el-color-warning);
    font-family: monospace;
  }

  &__actions {
    margin-left: auto;
    flex-shrink: 0;
  }
}
</style>
