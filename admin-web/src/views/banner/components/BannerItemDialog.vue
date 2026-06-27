<template>
  <el-dialog
    v-model="visible"
    :title="editId === null ? t('menu.banner.create') : t('menu.banner.edit')"
    width="560px"
    :close-on-click-modal="false"
    append-to-body
    destroy-on-close
    @closed="onClosed"
  >
    <el-form ref="formRef" :model="form" :rules="rules" label-width="88px" v-loading="detailLoading">
      <el-form-item :label="t('menu.banner.group')">
        <el-input :model-value="groupLabel" disabled />
      </el-form-item>
      <el-form-item :label="t('menu.banner.title')" prop="title">
        <el-input v-model="form.title" :placeholder="t('menu.banner.titlePh')" />
      </el-form-item>
      <el-form-item :label="t('menu.banner.image')" prop="image_url">
        <el-input v-model="form.image_url" :placeholder="t('menu.banner.imagePh')" />
      </el-form-item>
      <el-form-item :label="t('menu.banner.link')">
        <el-input v-model="form.link_url" :placeholder="t('menu.banner.linkPh')" />
      </el-form-item>
      <el-form-item :label="t('menu.menu.manage.groupDesc')">
        <el-input v-model="form.description" type="textarea" :rows="2" />
      </el-form-item>
      <el-form-item :label="t('menu.menu.manage.sort')">
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
  createBannerApi,
  getBannerApi,
  updateBannerApi,
  type BannerGroup,
} from "@/api/system/banners.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

const props = defineProps<{
  modelValue: boolean;
  editId: number | null;
  groupId: number;
  groupLabel: string;
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
  image_url: "",
  link_url: "",
  description: "",
  sort: 0,
  status: 1,
});

const rules = computed<FormRules>(() => ({
  title: [{ required: true, message: t("menu.banner.titleRequired"), trigger: "blur" }],
  image_url: [{ required: true, message: t("menu.banner.imageRequired"), trigger: "blur" }],
}));

function resetForm() {
  form.title = "";
  form.image_url = "";
  form.link_url = "";
  form.description = "";
  form.sort = 0;
  form.status = 1;
}

async function loadDetail(id: number) {
  detailLoading.value = true;
  try {
    const res = await getBannerApi(id);
    if (res.code !== 0 || !res.data) {
      koiMsgError(res.message || t("msg.fail"));
      return;
    }
    const data = res.data;
    form.title = data.title;
    form.image_url = data.image_url;
    form.link_url = data.link_url;
    form.description = data.description;
    form.sort = data.sort;
    form.status = data.status;
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
        group_id: props.groupId,
        title: form.title.trim(),
        image_url: form.image_url.trim(),
        link_url: form.link_url.trim(),
        description: form.description.trim(),
        sort: form.sort,
        status: form.status,
      };
      const res =
        props.editId === null
          ? await createBannerApi(payload)
          : await updateBannerApi(props.editId, payload);
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
