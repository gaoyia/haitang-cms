<template>
  <el-dialog
    v-model="visible"
    :title="editId === null && savedBannerId === null ? t('menu.banner.create') : t('menu.banner.edit')"
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
      <el-form-item :label="t('menu.banner.image')" prop="imageReady">
        <BannerImageField
          ref="imageFieldRef"
          :banner-id="editId ?? savedBannerId"
          :initial-image="initialImage"
          :initial-image-enabled="initialImageEnabled"
          @changed="onImageChanged"
        />
      </el-form-item>
      <el-form-item :label="t('menu.banner.link')">
        <el-input v-model="form.link_url" :placeholder="t('menu.banner.linkPh')" />
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
import { computed, nextTick, reactive, ref, watch } from "vue";
import type { FormInstance, FormRules } from "element-plus";
import { useI18n } from "vue-i18n";
import BannerImageField from "@/components/assets/BannerImageField.vue";
import {
  createBannerApi,
  getBannerApi,
  updateBannerApi,
} from "@/api/system/banners.ts";
import { listBannerAssetsApi, type AssetView } from "@/api/system/assets.ts";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

const props = defineProps<{
  modelValue: boolean;
  editId: number | null;
  groupId: number;
  groupLabel: string;
  defaultSort?: number;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "saved"): void;
}>();

const { t } = useI18n();
const formRef = ref<FormInstance>();
const imageFieldRef = ref<InstanceType<typeof BannerImageField>>();
const detailLoading = ref(false);
const saving = ref(false);
const savedBannerId = ref<number | null>(null);
const initialImage = ref<AssetView | null>(null);
const initialImageEnabled = ref(true);

const visible = computed({
  get: () => props.modelValue,
  set: (v) => emit("update:modelValue", v),
});

const bannerId = computed(() => props.editId ?? savedBannerId.value);

const form = reactive({
  title: "",
  link_url: "",
  description: "",
  sort: 0,
  status: 1,
  imageReady: false,
});

const rules = computed<FormRules>(() => ({
  title: [{ required: true, message: t("menu.banner.titleRequired"), trigger: "blur" }],
      imageReady: [
    {
      validator: (_rule, _value, callback) => {
        const id = props.editId ?? savedBannerId.value;
        if (id != null && imageFieldRef.value?.hasImage()) {
          callback();
          return;
        }
        if (props.editId != null) {
          callback(new Error(t("menu.banner.imageRequired")));
          return;
        }
        callback();
      },
      trigger: "change",
    },
  ],
}));

function resetForm() {
  form.title = "";
  form.link_url = "";
  form.description = "";
  form.sort = props.defaultSort ?? 0;
  form.status = 1;
  form.imageReady = false;
  savedBannerId.value = null;
  initialImage.value = null;
  initialImageEnabled.value = true;
}

async function loadDetail(id: number) {
  detailLoading.value = true;
  try {
    const [bannerRes, assetsRes] = await Promise.all([getBannerApi(id), listBannerAssetsApi(id)]);
    if (bannerRes.code !== 0 || !bannerRes.data) {
      koiMsgError(bannerRes.message || t("msg.fail"));
      return;
    }
    const data = bannerRes.data;
    form.title = data.title;
    form.link_url = data.link_url;
    form.description = data.description;
    form.sort = data.sort;
    form.status = data.status;
    initialImage.value = assetsRes.code === 0 && assetsRes.data ? assetsRes.data.image : null;
    initialImageEnabled.value =
      assetsRes.code === 0 && assetsRes.data ? assetsRes.data.image_enabled : true;
    form.imageReady = initialImage.value != null;
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
  { immediate: true },
);

function onClosed() {
  formRef.value?.resetFields();
}

function onImageChanged() {
  form.imageReady = true;
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
        link_url: form.link_url.trim(),
        description: form.description.trim(),
        sort: form.sort,
        status: form.status,
      };
      const isCreate = props.editId === null && savedBannerId.value === null;
      const res = isCreate
        ? await createBannerApi(payload)
        : await updateBannerApi(bannerId.value!, payload);
      if (res.code !== 0 || !res.data) {
        koiMsgError(res.message || t("msg.fail"));
        return;
      }
      if (isCreate) {
        savedBannerId.value = res.data.id;
        await nextTick();
        if (imageFieldRef.value?.hasImage()) {
          const linked = await imageFieldRef.value.ensurePendingLinked();
          if (!linked) {
            koiMsgError(t("menu.banner.imageRequired"));
            return;
          }
          koiMsgSuccess(t("msg.success"));
          visible.value = false;
          emit("saved");
          return;
        }
        koiMsgSuccess(t("msg.success"));
        emit("saved");
        return;
      }
      if (!imageFieldRef.value?.hasImage()) {
        koiMsgError(t("menu.banner.imageRequired"));
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
