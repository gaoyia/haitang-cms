<template>
  <div class="asset-uploader">
    <el-upload
      :show-file-list="false"
      :disabled="disabled || uploading"
      :accept="accept"
      :http-request="handleUpload"
      :before-upload="beforeUpload"
    >
      <slot>
        <el-button type="primary" :loading="uploading" :disabled="disabled">
          {{ label || t("menu.assets.uploadPick") }}
        </el-button>
      </slot>
    </el-upload>
    <el-progress v-if="uploading" :percentage="progress" :stroke-width="4" class="asset-uploader__progress" />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import type { UploadProps, UploadRequestOptions } from "element-plus";
import {
  uploadAssetApi,
  type AssetPurpose,
  type AssetView,
  type BannerAssetRole,
  type PostAssetRole,
} from "@/api/system/assets.ts";
import { koiMsgError } from "@/utils/koi.ts";

const props = withDefaults(
  defineProps<{
    purpose: AssetPurpose;
    postId?: number | null;
    role?: PostAssetRole;
    bannerId?: number | null;
    bannerRole?: BannerAssetRole;
    accept?: string;
    maxSizeMb?: number;
    disabled?: boolean;
    label?: string;
  }>(),
  {
    postId: null,
    bannerId: null,
    accept: "",
    maxSizeMb: 10,
    disabled: false,
    label: "",
  },
);

const emit = defineEmits<{
  success: [asset: AssetView];
}>();

const { t } = useI18n();
const uploading = ref(false);
const progress = ref(0);

const beforeUpload: UploadProps["beforeUpload"] = (file) => {
  const okSize = file.size / 1024 / 1024 <= props.maxSizeMb;
  if (!okSize) {
    koiMsgError(`文件大小不能超过 ${props.maxSizeMb}MB`);
  }
  return okSize;
};

async function handleUpload(options: UploadRequestOptions) {
  uploading.value = true;
  progress.value = 0;
  try {
    const res = await uploadAssetApi(
      options.file as File,
      props.purpose,
      {
        postId: props.postId,
        role: props.role,
        bannerId: props.bannerId,
        bannerRole: props.bannerRole,
      },
      (pct) => {
        progress.value = pct;
      },
    );
    if (res.code !== 0 || !res.data) {
      koiMsgError(res.message || t("msg.fail"));
      throw new Error(res.message || "upload failed");
    }
    emit("success", res.data);
  } finally {
    uploading.value = false;
    progress.value = 0;
  }
}
</script>

<style scoped lang="scss">
.asset-uploader__progress {
  margin-top: 8px;
  max-width: 240px;
}
</style>
