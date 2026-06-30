<template>
  <el-drawer
    v-model="visible"
    :title="drawerTitle"
    :size="drawerSize"
    :close-on-click-modal="false"
    append-to-body
    destroy-on-close
    @closed="onClosed"
  >
    <el-alert
      v-if="readonly"
      type="info"
      :closable="false"
      show-icon
      class="menu-drawer-alert"
      :title="t('menu.menu.manage.readonlyHint')"
    />
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="88px"
      v-loading="loading"
      :disabled="readonly"
    >
      <el-divider content-position="left">{{ t("menu.menu.manage.sectionStructure") }}</el-divider>

      <el-form-item :label="t('menu.menu.manage.group')">
        <el-input :model-value="groupName" disabled />
      </el-form-item>

      <el-form-item :label="t('menu.menu.manage.parent')">
        <el-tree-select
          v-model="form.parent_id"
          :data="parentTreeData"
          :props="treeSelectProps"
          check-strictly
          clearable
          :render-after-expand="false"
          :placeholder="t('menu.menu.manage.parentRoot')"
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item :label="t('menu.menu.manage.icon')">
        <KoiSelectIcon v-model="form.icon" width="100%" :disabled="readonly" />
      </el-form-item>

      <el-form-item v-if="showPermission" :label="t('menu.menu.manage.permission')">
        <el-select
          v-model="form.permission"
          clearable
          filterable
          :placeholder="t('menu.menu.manage.permissionPh')"
          style="width: 100%"
        >
          <el-option-group
            v-for="pg in permissionGroups"
            :key="pg.group"
            :label="pg.group"
          >
            <el-option
              v-for="p in pg.permissions"
              :key="p.code"
              :label="`${p.label} (${p.code})`"
              :value="p.code"
            />
          </el-option-group>
        </el-select>
      </el-form-item>

      <el-form-item :label="t('menu.menu.manage.status')">
        <el-radio-group v-model="form.status">
          <el-radio :value="1">{{ t("menu.menu.manage.enabled") }}</el-radio>
          <el-radio :value="0">{{ t("menu.menu.manage.disabled") }}</el-radio>
        </el-radio-group>
      </el-form-item>

      <el-divider content-position="left">{{ t("menu.menu.manage.sectionI18n") }}</el-divider>

      <div v-if="!showPermission" class="locale-prefix-switch">
        <div class="locale-prefix-switch__row">
          <span class="locale-prefix-switch__label">{{ t("menu.menu.manage.autoLocalePrefix") }}</span>
          <el-switch v-model="autoLocalePrefix" @change="onAutoLocalePrefixChange" />
        </div>
        <p class="field-hint">{{ t("menu.menu.manage.autoLocalePrefixHint") }}</p>
      </div>

      <el-tabs v-model="activeLocale" type="border-card" class="menu-locale-tabs">
        <el-tab-pane
          v-for="loc in siteLocales"
          :key="loc"
          :label="localeLabel(loc)"
          :name="loc"
        >
          <el-form-item
            :label="t('menu.menu.manage.itemTitle')"
          >
            <el-input
              v-model="form.i18n[loc].title"
              :placeholder="t('menu.menu.manage.itemTitlePh')"
            />
          </el-form-item>
          <el-form-item :label="t('menu.menu.manage.itemPath')">
            <el-input
              v-if="autoLocalePrefix"
              v-model="form.i18n[loc].path"
              :placeholder="t('menu.menu.manage.itemPathPhAuto')"
            >
              <template #prepend>{{ localePathPrefix(loc) }}</template>
            </el-input>
            <el-input
              v-else
              v-model="form.i18n[loc].path"
              :placeholder="pathPlaceholder(loc)"
            />
          </el-form-item>
        </el-tab-pane>
      </el-tabs>
    </el-form>

    <template #footer>
      <div class="menu-drawer-footer">
        <el-button @click="visible = false">{{ t("button.cancel") }}</el-button>
        <el-button v-if="!readonly" type="primary" :loading="saving" @click="handleSave">
          {{ t("button.save") }}
        </el-button>
      </div>
    </template>
  </el-drawer>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import type { FormInstance, FormRules } from "element-plus";
import { useI18n } from "vue-i18n";
import {
  createMenuApi,
  getMenuItemApi,
  updateMenuApi,
  type MenuItem,
  type PermissionGroup,
} from "@/api/system/menus.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import { nextSortValue } from "@/utils/sortOrder.ts";
import { getMenuSiblings } from "@/utils/menuTree.ts";
import {
  buildLocalePath,
  hasLocalePrefix,
  isBlankLocalePath,
  localePathPrefix,
  parseLocalePath,
  shouldAutoLocalePrefix,
} from "@/utils/localePath.ts";
import KoiSelectIcon from "@/components/KoiSelectIcon/Index.vue";
import { useResponsiveDrawerSize } from "@/composables/useResponsiveDrawerSize.ts";

export type LocaleForm = Record<string, { title: string; path: string }>;

const props = defineProps<{
  modelValue: boolean;
  editId: number | null;
  groupId: number;
  groupCode: string;
  groupName: string;
  readonly: boolean;
  menus: MenuItem[];
  presetParentId?: number;
  siteLocales: string[];
  defaultLocale: string;
  permissionGroups: PermissionGroup[];
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "saved"): void;
}>();

const { t } = useI18n();
const drawerSize = useResponsiveDrawerSize("480px");
const formRef = ref<FormInstance>();
const loading = ref(false);
const saving = ref(false);
const activeLocale = ref("zh-cn");
const autoLocalePrefix = ref(true);

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit("update:modelValue", v),
});

const isEdit = computed(() => props.editId !== null);
const drawerTitle = computed(() => {
  if (props.readonly && isEdit.value) {
    return t("button.view");
  }
  return isEdit.value ? t("menu.menu.manage.itemEdit") : t("menu.menu.manage.itemCreate");
});
const showPermission = computed(() => props.groupCode === "admin_sidebar");

const form = reactive({
  parent_id: 0,
  icon: "",
  permission: "",
  sort: 0,
  status: 1,
  i18n: {} as LocaleForm,
});

const rules = computed<FormRules>(() => ({}));

const treeSelectProps = {
  label: "title",
  value: "id",
  children: "children",
};

function localeLabel(loc: string): string {
  if (loc === "en-us") return "English";
  if (loc === "zh-cn") return "中文";
  return loc;
}

function pathPlaceholder(loc: string): string {
  return loc === "en-us" ? "/en-us/about" : "/zh-cn/about";
}

function onAutoLocalePrefixChange(enabled: boolean) {
  for (const loc of props.siteLocales) {
    const raw = form.i18n[loc]?.path ?? "";
    if (isBlankLocalePath(raw)) continue;
    if (enabled) {
      form.i18n[loc].path = parseLocalePath(loc, raw);
    } else if (!hasLocalePrefix(loc, raw)) {
      form.i18n[loc].path = buildLocalePath(loc, raw, true);
    }
  }
}

function resolveStoredPaths() {
  const pathsByLocale: Record<string, string> = {};
  for (const loc of props.siteLocales) {
    pathsByLocale[loc] = form.i18n[loc]?.path ?? "";
  }
  autoLocalePrefix.value = shouldAutoLocalePrefix(pathsByLocale, props.siteLocales);
  if (autoLocalePrefix.value) {
    for (const loc of props.siteLocales) {
      form.i18n[loc].path = parseLocalePath(loc, form.i18n[loc]?.path ?? "");
    }
  }
}

function emptyI18n(): LocaleForm {
  const map: LocaleForm = {};
  for (const loc of props.siteLocales) {
    map[loc] = { title: "", path: "" };
  }
  return map;
}

/** 排除自身及子树，避免循环引用 */
function collectExcludedIds(items: MenuItem[], excludeId: number): Set<number> {
  const ids = new Set<number>();
  function walk(nodes: MenuItem[]) {
    for (const node of nodes) {
      if (node.id === excludeId) {
        collectChildren(node);
      } else if (node.children?.length) {
        walk(node.children);
      }
    }
  }
  function collectChildren(node: MenuItem) {
    ids.add(node.id);
    node.children?.forEach(collectChildren);
  }
  walk(items);
  return ids;
}

const parentTreeData = computed(() => {
  if (props.editId === null) return props.menus;
  const excluded = collectExcludedIds(props.menus, props.editId);
  function filterNodes(nodes: MenuItem[]): MenuItem[] {
    return nodes
      .filter((n) => !excluded.has(n.id))
      .map((n) => ({
        ...n,
        children: n.children?.length ? filterNodes(n.children) : [],
      }));
  }
  return filterNodes(props.menus);
});

function resetForm() {
  const parentId = props.presetParentId ?? 0;
  form.parent_id = parentId;
  form.icon = "";
  form.permission = "";
  form.sort = nextSortValue(getMenuSiblings(props.menus, parentId));
  form.status = 1;
  form.i18n = emptyI18n();
  autoLocalePrefix.value = true;
  activeLocale.value = props.defaultLocale || props.siteLocales[0] || "zh-cn";
}

async function loadDetail() {
  if (props.editId === null) return;
  loading.value = true;
  form.i18n = emptyI18n();
  try {
    const first = await getMenuItemApi(props.editId, props.defaultLocale);
    if (first.code !== 0 || !first.data) {
      koiMsgError(first.message || t("msg.fail"));
      return;
    }
    const meta = first.data;
    form.parent_id = meta.parent_id;
    form.icon = meta.icon;
    form.permission = meta.permission;
    form.sort = meta.sort;
    form.status = meta.status;
    form.i18n[props.defaultLocale] = {
      title: meta.title,
      path: meta.path,
    };

    await Promise.all(
      props.siteLocales
        .filter((loc) => loc !== props.defaultLocale)
        .map(async (loc) => {
          const res = await getMenuItemApi(props.editId!, loc);
          if (res.code === 0 && res.data) {
            form.i18n[loc] = { title: res.data.title, path: res.data.path };
          }
        }),
    );
    resolveStoredPaths();
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
    } else if (!props.readonly) {
      resetForm();
    }
  },
);

function onClosed() {
  formRef.value?.resetFields();
}

async function handleSave() {
  if (!formRef.value || props.readonly) return;
  const primary = form.i18n[props.defaultLocale];
  const defaultTitle = primary?.title?.trim();
  if (!defaultTitle) {
    activeLocale.value = props.defaultLocale;
    koiMsgError(t("menu.menu.manage.itemTitleRequired"));
    return;
  }
  const defaultPath = buildLocalePath(
    props.defaultLocale,
    primary?.path ?? "",
    autoLocalePrefix.value,
  );
  if (!defaultPath) {
    activeLocale.value = props.defaultLocale;
    koiMsgError(t("menu.menu.manage.itemPathRequired"));
    return;
  }

  saving.value = true;
  try {
    const metaPayload = {
      group_id: props.groupId,
      parent_id: form.parent_id,
      icon: form.icon || undefined,
      permission: form.permission || undefined,
      sort: form.sort,
      status: form.status,
    };

    if (!isEdit.value) {
      const createRes = await createMenuApi({
        ...metaPayload,
        title: defaultTitle,
        path: defaultPath,
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
        const title = row.title.trim();
        const path = buildLocalePath(loc, row.path, autoLocalePrefix.value);
        if (!title && !path) continue;
        const res = await updateMenuApi(newId, {
          title,
          ...(path ? { path } : {}),
          lang: loc,
        });
        if (res.code !== 0) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
      }
    } else {
      const metaRes = await updateMenuApi(props.editId!, metaPayload);
      if (metaRes.code !== 0) {
        koiMsgError(metaRes.message || t("msg.fail"));
        return;
      }
      for (const loc of props.siteLocales) {
        const row = form.i18n[loc];
        const title = row.title.trim();
        const path = buildLocalePath(loc, row.path, autoLocalePrefix.value);
        if (!title && !path) continue;
        const res = await updateMenuApi(props.editId!, {
          title,
          ...(path ? { path } : {}),
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
.menu-drawer-alert {
  margin-bottom: 12px;
}

.menu-locale-tabs {
  margin-top: 4px;
}

.locale-prefix-switch {
  margin-bottom: 12px;
}

.locale-prefix-switch__row {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
}

.locale-prefix-switch__label {
  font-size: 14px;
  color: var(--el-text-color-regular);
  line-height: 1.5;
}

.field-hint {
  margin: 0;
  font-size: 12px;
  color: var(--el-text-color-secondary);
  line-height: 1.5;
}

.menu-drawer-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
