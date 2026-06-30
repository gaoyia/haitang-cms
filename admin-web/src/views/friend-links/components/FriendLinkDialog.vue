<template>
  <el-dialog
    v-model="visible"
    :title="editId === null ? t('menu.friendLink.create') : t('menu.friendLink.edit')"
    width="560px"
    :close-on-click-modal="false"
    append-to-body
    destroy-on-close
    @closed="onClosed"
  >
    <el-form ref="formRef" :model="form" :rules="rules" label-width="88px" v-loading="detailLoading">
      <el-form-item :label="t('menu.friendLink.title')" prop="title">
        <el-input v-model="form.title" :placeholder="t('menu.friendLink.titlePh')" />
      </el-form-item>
      <el-form-item :label="t('menu.friendLink.url')" prop="url">
        <el-input v-model="form.url" :placeholder="t('menu.friendLink.urlPh')" />
      </el-form-item>
      <el-form-item :label="t('menu.friendLink.image')" prop="image_url">
        <FriendLinkImageField v-model="form.image_url" />
      </el-form-item>
      <el-form-item :label="t('menu.friendLink.sort')">
        <el-input-number v-model="form.sort" :min="0" :max="9999" />
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
  createFriendLinkApi,
  getFriendLinkApi,
  updateFriendLinkApi,
} from "@/api/system/friendLinks.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";
import FriendLinkImageField from "./FriendLinkImageField.vue";

const props = defineProps<{
  modelValue: boolean;
  editId: number | null;
  defaultSort?: number;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "saved"): void;
}>();

const { t } = useI18n();
const formRef = ref<FormInstance>();
const detailLoading = ref(false);
const saving = ref(false);

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit("update:modelValue", v),
});

const form = reactive({
  title: "",
  url: "",
  image_url: "",
  sort: 0,
  status: 1,
});

const rules = computed<FormRules>(() => ({
  title: [{ required: true, message: t("menu.friendLink.titleRequired"), trigger: "blur" }],
  url: [{ required: true, message: t("menu.friendLink.urlRequired"), trigger: "blur" }],
  image_url: [{ required: true, message: t("menu.friendLink.imageRequired"), trigger: "change" }],
}));

function resetForm() {
  form.title = "";
  form.url = "";
  form.image_url = "";
  form.sort = props.defaultSort ?? 0;
  form.status = 1;
}

async function loadDetail(id: number) {
  detailLoading.value = true;
  try {
    const res = await getFriendLinkApi(id);
    if (res.code !== 0 || !res.data) {
      koiMsgError(res.message || t("msg.fail"));
      return;
    }
    form.title = res.data.title;
    form.url = res.data.url;
    form.image_url = res.data.image_url;
    form.sort = res.data.sort;
    form.status = res.data.status;
  } finally {
    detailLoading.value = false;
  }
}

watch(
  () => props.modelValue,
  (open) => {
    if (!open) return;
    resetForm();
    if (props.editId !== null) {
      loadDetail(props.editId);
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
        title: form.title.trim(),
        url: form.url.trim(),
        image_url: form.image_url.trim(),
        sort: form.sort,
        status: form.status,
      };
      const res =
        props.editId === null
          ? await createFriendLinkApi(payload)
          : await updateFriendLinkApi(props.editId, payload);
      if (res.code !== 0) {
        koiMsgError(res.message || t("msg.fail"));
        return;
      }
      koiMsgSuccess(t("msg.success"));
      visible.value = false;
      emit("saved");
    } finally {
      saving.value = false;
    }
  });
}
</script>
