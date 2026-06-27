<template>
  <div class="user-manage-page koi-page">
    <KoiCard>
      <template #header>
        <div class="page-head">
          <div>
            <div class="page-head__title">{{ t("menu.system.user.manage.title") }}</div>
            <div class="page-head__desc">{{ t("menu.system.user.manage.subtitle") }}</div>
          </div>
          <KoiToolbar :show-maximize="false" @refresh-table="loadUsers" />
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
        :data="users"
        :total="total"
        stripe
        border
        class="page-table"
        @change="loadUsers"
      >
        <el-table-column prop="username" :label="t('menu.system.user.manage.username')" min-width="120" />
        <el-table-column prop="nickname" :label="t('menu.system.user.manage.nickname')" min-width="120">
          <template #default="{ row }">{{ row.nickname || "—" }}</template>
        </el-table-column>
        <el-table-column prop="email" :label="t('menu.system.user.manage.email')" min-width="160">
          <template #default="{ row }">{{ row.email || "—" }}</template>
        </el-table-column>
        <el-table-column :label="t('menu.system.user.manage.status')" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="row.status === 1 ? 'success' : 'danger'" effect="plain" size="small">
              {{ row.status === 1 ? t("menu.system.user.manage.enabled") : t("menu.system.user.manage.disabled") }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="t('menu.system.user.manage.roles')" min-width="160">
          <template #default="{ row }">{{ formatRoles(row.role_ids) }}</template>
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
      :title="isEdit ? t('menu.system.user.manage.edit') : t('menu.system.user.manage.create')"
      width="520px"
      :close-on-click-modal="false"
      append-to-body
      destroy-on-close
      @closed="onDialogClosed"
    >
      <el-form ref="formRef" :model="form" :rules="rules" label-width="88px">
        <el-form-item v-if="!isEdit" :label="t('menu.system.user.manage.username')" prop="username">
          <el-input v-model="form.username" :placeholder="t('menu.system.user.manage.usernameRequired')" />
        </el-form-item>
        <el-form-item
          :label="t('menu.system.user.manage.password')"
          :prop="isEdit ? undefined : 'password'"
        >
          <el-input
            v-model="form.password"
            type="password"
            show-password
            :placeholder="isEdit ? t('menu.system.user.manage.passwordOptional') : t('menu.system.user.manage.passwordRequired')"
          />
        </el-form-item>
        <el-form-item :label="t('menu.system.user.manage.nickname')">
          <el-input v-model="form.nickname" />
        </el-form-item>
        <el-form-item :label="t('menu.system.user.manage.email')">
          <el-input v-model="form.email" type="email" />
        </el-form-item>
        <el-form-item v-if="isEdit" :label="t('menu.system.user.manage.status')">
          <el-radio-group v-model="form.status">
            <el-radio :value="1">{{ t("menu.system.user.manage.enabled") }}</el-radio>
            <el-radio :value="0">{{ t("menu.system.user.manage.disabled") }}</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item :label="t('menu.system.user.manage.roles')">
          <el-select
            v-model="form.role_ids"
            multiple
            collapse-tags
            collapse-tags-tooltip
            :placeholder="t('menu.system.user.manage.rolePlaceholder')"
            style="width: 100%"
          >
            <el-option v-for="role in allRoles" :key="role.id" :label="role.name" :value="role.id" />
          </el-select>
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
  assignUserRolesApi,
  createUserApi,
  deleteUserApi,
  listUsersApi,
  updateUserApi,
  type UserView,
} from "@/api/system/users.ts";
import { listRolesApi, type RoleView } from "@/api/system/roles.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import { useTablePage } from "@/composables/useTablePage.ts";

const { t } = useI18n();
const { page, pageSize, total, pageParams, applyPageResult } = useTablePage();

const loading = ref(false);
const saving = ref(false);
const users = ref<UserView[]>([]);
const allRoles = ref<RoleView[]>([]);

const dialogVisible = ref(false);
const editingUser = ref<UserView | null>(null);
const formRef = ref<FormInstance>();

const isEdit = computed(() => editingUser.value !== null);

const form = reactive({
  username: "",
  password: "",
  nickname: "",
  email: "",
  status: 1,
  role_ids: [] as number[],
});

const roleMap = computed(() => {
  const map = new Map<number, string>();
  for (const role of allRoles.value) {
    map.set(role.id, role.name);
  }
  return map;
});

const rules = computed<FormRules>(() => ({
  username: [{ required: true, message: t("menu.system.user.manage.usernameRequired"), trigger: "blur" }],
  password: [{ required: true, message: t("menu.system.user.manage.passwordRequired"), trigger: "blur" }],
}));

function formatRoles(roleIds: number[]): string {
  const names = roleIds.map((id) => roleMap.value.get(id)).filter(Boolean);
  return names.length > 0 ? names.join("、") : "—";
}

async function loadRoles() {
  const res = await listRolesApi({ page: 1, page_size: 500 });
  if (res.code === 0 && res.data) {
    allRoles.value = res.data.list;
  }
}

async function loadUsers() {
  loading.value = true;
  try {
    const res = await listUsersApi(pageParams.value);
    users.value = applyPageResult(res.code === 0 ? res.data : null);
  } finally {
    loading.value = false;
  }
}

function resetForm() {
  form.username = "";
  form.password = "";
  form.nickname = "";
  form.email = "";
  form.status = 1;
  form.role_ids = [];
}

function openDialog(user: UserView | null) {
  editingUser.value = user;
  if (user) {
    form.username = user.username;
    form.password = "";
    form.nickname = user.nickname;
    form.email = user.email;
    form.status = user.status;
    form.role_ids = [...user.role_ids];
  } else {
    resetForm();
  }
  dialogVisible.value = true;
}

function onDialogClosed() {
  formRef.value?.resetFields();
  editingUser.value = null;
}

async function handleSave() {
  if (!formRef.value) return;
  await formRef.value.validate(async (valid) => {
    if (!valid) return;
    saving.value = true;
    try {
      if (isEdit.value && editingUser.value) {
        const updateInput: Record<string, unknown> = {
          nickname: form.nickname.trim(),
          email: form.email.trim(),
          status: form.status,
        };
        if (form.password.trim()) {
          updateInput.password = form.password;
        }
        const res = await updateUserApi(editingUser.value.id, updateInput);
        if (res.code !== 0) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
        const roleRes = await assignUserRolesApi(editingUser.value.id, form.role_ids);
        if (roleRes.code !== 0) {
          koiMsgError(roleRes.message || t("msg.fail"));
          return;
        }
      } else {
        const res = await createUserApi({
          username: form.username.trim(),
          password: form.password,
          nickname: form.nickname.trim() || undefined,
          email: form.email.trim() || undefined,
        });
        if (res.code !== 0 || !res.data) {
          koiMsgError(res.message || t("msg.fail"));
          return;
        }
        const roleRes = await assignUserRolesApi(res.data.id, form.role_ids);
        if (roleRes.code !== 0) {
          koiMsgError(roleRes.message || t("msg.fail"));
          return;
        }
      }
      koiMsgSuccess(t("msg.success"));
      dialogVisible.value = false;
      await loadUsers();
    } finally {
      saving.value = false;
    }
  });
}

async function handleDelete(user: UserView) {
  try {
    await ElMessageBox.confirm(
      t("menu.system.user.manage.deleteConfirm", { name: user.username }),
      t("msg.remind"),
      { type: "warning", confirmButtonText: t("button.delete"), cancelButtonText: t("button.cancel") },
    );
  } catch {
    return;
  }
  const res = await deleteUserApi(user.id);
  if (res.code === 0) {
    koiMsgSuccess(t("msg.success"));
    await loadUsers();
  } else {
    koiMsgError(res.message || t("msg.fail"));
  }
}

onMounted(async () => {
  await loadRoles();
  await loadUsers();
});
</script>

<style scoped lang="scss">
.user-manage-page {
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
</style>
