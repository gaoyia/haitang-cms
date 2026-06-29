<template>
  <el-dialog
    v-model="visible"
    :title="isEdit ? t('menu.banner.groupEdit') : t('menu.banner.groupCreate')"
    width="520px"
    :close-on-click-modal="false"
    append-to-body
    destroy-on-close
    @closed="onClosed"
  >
    <el-form ref="formRef" :model="form" :rules="rules" label-width="88px">
      <el-form-item :label="t('menu.menu.manage.groupName')" prop="name">
        <el-input v-model="form.name" :placeholder="t('menu.menu.manage.groupNamePh')" />
      </el-form-item>
      <el-form-item :label="t('menu.menu.manage.groupCode')" prop="code">
        <el-input v-model="form.code" :disabled="isEdit" :placeholder="t('menu.menu.manage.groupCodePh')" />
      </el-form-item>
      <el-form-item :label="t('menu.menu.manage.groupDesc')">
        <el-input v-model="form.description" type="textarea" :rows="2" />
      </el-form-item>
      <el-form-item :label="t('menu.menu.manage.status')">
        <el-radio-group v-model="form.status">
          <el-radio :value="1">{{ t("menu.menu.manage.enabled") }}</el-radio>
          <el-radio :value="0">{{ t("menu.menu.manage.disabled") }}</el-radio>
        </el-radio-group>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="visible = false">{{ t("button.cancel") }}</el-button>
      <el-button type="primary" :loading="saving" @click="handleSave">{{ t("button.confirm") }}</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import type { FormInstance, FormRules } from "element-plus";
import { useI18n } from "vue-i18n";
import {
  createBannerGroupApi,
  updateBannerGroupApi,
  type BannerGroup,
} from "@/api/system/banners.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

const props = defineProps<{
  modelValue: boolean;
  editGroup: BannerGroup | null;
  defaultSort?: number;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "saved"): void;
}>();

const { t } = useI18n();
const formRef = ref<FormInstance>();
const saving = ref(false);

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit("update:modelValue", v),
});

const isEdit = computed(() => props.editGroup !== null);

const form = reactive({
  name: "",
  code: "",
  description: "",
  status: 1,
});

const rules = computed<FormRules>(() => ({
  name: [{ required: true, message: t("menu.menu.manage.groupNameRequired"), trigger: "blur" }],
  code: [
    { required: true, message: t("menu.menu.manage.groupCodeRequired"), trigger: "blur" },
    {
      pattern: /^[a-z][a-z0-9_]*$/,
      message: t("menu.menu.manage.groupCodePattern"),
      trigger: "blur",
    },
  ],
}));

watch(
  () => props.modelValue,
  (open) => {
    if (!open) return;
    if (props.editGroup) {
      form.name = props.editGroup.name;
      form.code = props.editGroup.code;
      form.description = props.editGroup.description;
      form.status = props.editGroup.status;
    } else {
      form.name = "";
      form.code = "";
      form.description = "";
      form.status = 1;
    }
  },
);

function onClosed() {
  formRef.value?.resetFields();
}

async function handleSave() {
  if (!formRef.value) return;
  await formRef.value.validate(async (valid) => {
    if (!valid) return;
    saving.value = true;
    try {
      const payload = {
        name: form.name.trim(),
        code: form.code.trim(),
        description: form.description.trim(),
        status: form.status,
        ...(isEdit.value ? {} : { sort: props.defaultSort ?? 0 }),
      };
      const res = isEdit.value
        ? await updateBannerGroupApi(props.editGroup!.id, payload)
        : await createBannerGroupApi(payload);
      if (res.code === 0) {
        koiMsgSuccess(t("msg.success"));
        visible.value = false;
        emit("saved");
      } else {
        koiMsgError(res.message || t("msg.fail"));
      }
    } finally {
      saving.value = false;
    }
  });
}
</script>
