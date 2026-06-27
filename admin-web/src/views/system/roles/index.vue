<template>
  <div class="role-manage-page koi-page">
    <KoiCard>
      <template #header>
        <div class="page-head">
          <div>
            <div class="page-head__title">{{ t("menu.system.role.manage.title") }}</div>
            <div class="page-head__desc">{{ t("menu.system.role.manage.subtitle") }}</div>
          </div>
          <KoiToolbar :show-maximize="false" @refresh-table="loadRoles" />
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
        :data="roles"
        :total="total"
        stripe
        border
        class="page-table"
        @change="loadRoles"
      >
        <el-table-column prop="name" :label="t('menu.system.role.manage.name')" min-width="140" />
        <el-table-column prop="description" :label="t('menu.system.role.manage.description')" min-width="200">
          <template #default="{ row }">{{ row.description || "—" }}</template>
        </el-table-column>
        <el-table-column :label="t('menu.system.role.manage.permCount')" width="120" align="center">
          <template #default="{ row }">{{ row.permissions.length }}</template>
        </el-table-column>
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
      :title="isEdit ? t('menu.system.role.manage.edit') : t('menu.system.role.manage.create')"
      width="640px"
      :close-on-click-modal="false"
      append-to-body
      destroy-on-close
      @closed="onDialogClosed"
    >
      <el-form ref="formRef" :model="form" :rules="rules" label-width="88px">
        <el-form-item :label="t('menu.system.role.manage.name')" prop="name">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="t('menu.system.role.manage.description')">
          <el-input v-model="form.description" type="textarea" :rows="2" />
        </el-form-item>
        <el-form-item :label="t('menu.system.role.manage.permissions')">
          <div v-if="permGroups.length === 0" class="perm-empty">{{ t("msg.null") }}</div>
          <div v-else class="perm-groups">
            <div v-for="group in permGroups" :key="group.group" class="perm-group">
              <div class="perm-group__head">
                <el-checkbox
                  :model-value="isGroupAllChecked(group)"
                  :indeterminate="isGroupIndeterminate(group)"
                  @change="(val: boolean) => toggleGroupAll(group, val)"
                >
                  {{ group.group }}
                </el-checkbox>
              </div>
              <el-checkbox-group v-model="form.permissions" class="perm-group__body">
                <el-checkbox
                  v-for="perm in group.permissions"
                  :key="perm.code"
                  :label="perm.code"
                >
                  {{ perm.label }}
                </el-checkbox>
              </el-checkbox-group>
            </div>
          </div>
          <div v-if="form.permissions.length > 0" class="perm-summary">
            {{ t("menu.system.role.manage.selectedCount", { count: form.permissions.length }) }}
          </div>
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
import { computed, onMounted, reactive, ref } from "vue";
import type { FormInstance, FormRules } from "element-plus";
import { useI18n } from "vue-i18n";
import { Plus } from "@element-plus/icons-vue";
import { ElMessageBox } from "element-plus";
import {
  createRoleApi,
  deleteRoleApi,
  listPermissionsApi,
  listRolesApi,
  updateRoleApi,
  type PermissionGroup,
  type RoleView,
} from "@/api/system/roles.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import { useTablePage } from "@/composables/useTablePage.ts";

const { t } = useI18n();
const { page, pageSize, total, pageParams, applyPageResult } = useTablePage();

const loading = ref(false);
const saving = ref(false);
const roles = ref<RoleView[]>([]);
const permGroups = ref<PermissionGroup[]>([]);

const dialogVisible = ref(false);
const editingRole = ref<RoleView | null>(null);
const formRef = ref<FormInstance>();

const isEdit = computed(() => editingRole.value !== null);

const form = reactive({
  name: "",
  description: "",
  permissions: [] as string[],
});

const rules = computed<FormRules>(() => ({
  name: [{ required: true, message: t("menu.system.role.manage.nameRequired"), trigger: "blur" }],
}));

function isGroupAllChecked(group: PermissionGroup): boolean {
  return group.permissions.every((p) => form.permissions.includes(p.code));
}

function isGroupIndeterminate(group: PermissionGroup): boolean {
  const selected = group.permissions.filter((p) => form.permissions.includes(p.code)).length;
  return selected > 0 && selected < group.permissions.length;
}

function toggleGroupAll(group: PermissionGroup, checked: boolean) {
  const codes = group.permissions.map((p) => p.code);
  if (checked) {
    for (const code of codes) {
      if (!form.permissions.includes(code)) {
        form.permissions.push(code);
      }
    }
  } else {
    form.permissions = form.permissions.filter((code) => !codes.includes(code));
  }
}

async function loadPermissions() {
  const res = await listPermissionsApi();
  if (res.code === 0 && res.data) {
    permGroups.value = res.data;
  }
}

async function loadRoles() {
  loading.value = true;
  try {
    const res = await listRolesApi(pageParams.value);
    roles.value = applyPageResult(res.code === 0 ? res.data : null);
  } finally {
    loading.value = false;
  }
}

function resetForm() {
  form.name = "";
  form.description = "";
  form.permissions = [];
}

function openDialog(role: RoleView | null) {
  editingRole.value = role;
  if (role) {
    form.name = role.name;
    form.description = role.description;
    form.permissions = [...role.permissions];
  } else {
    resetForm();
  }
  dialogVisible.value = true;
}

function onDialogClosed() {
  formRef.value?.resetFields();
  editingRole.value = null;
}

async function handleSave() {
  if (!formRef.value) return;
  await formRef.value.validate(async (valid) => {
    if (!valid) return;
    saving.value = true;
    try {
      const payload = {
        name: form.name.trim(),
        description: form.description.trim() || undefined,
        permissions: form.permissions,
      };
      const res = isEdit.value && editingRole.value
        ? await updateRoleApi(editingRole.value.id, payload)
        : await createRoleApi(payload);
      if (res.code === 0) {
        koiMsgSuccess(t("msg.success"));
        dialogVisible.value = false;
        await loadRoles();
      } else {
        koiMsgError(res.message || t("msg.fail"));
      }
    } finally {
      saving.value = false;
    }
  });
}

async function handleDelete(role: RoleView) {
  try {
    await ElMessageBox.confirm(
      t("menu.system.role.manage.deleteConfirm", { name: role.name }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deleteRoleApi(role.id);
  if (res.code === 0) {
    koiMsgSuccess(t("msg.success"));
    await loadRoles();
  } else {
    koiMsgError(res.message || t("msg.fail"));
  }
}

onMounted(async () => {
  await loadPermissions();
  await loadRoles();
});
</script>

<style scoped lang="scss">
.role-manage-page {
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

.perm-empty {
  color: var(--el-text-color-secondary);
  font-size: 13px;
}

.perm-groups {
  width: 100%;
  max-height: 320px;
  overflow: auto;
  border: 1px solid var(--el-border-color-lighter);
  border-radius: 8px;
  padding: 8px 12px;
}

.perm-group {
  & + & {
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px dashed var(--el-border-color-lighter);
  }

  &__head {
    margin-bottom: 8px;
    font-weight: 600;
  }

  &__body {
    display: flex;
    flex-wrap: wrap;
    gap: 4px 16px;
    padding-left: 8px;
  }
}

.perm-summary {
  margin-top: 8px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}
</style>
