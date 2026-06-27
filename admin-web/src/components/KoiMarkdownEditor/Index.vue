<template>
  <MdEditor
    :id="editorId"
    v-model="model"
    :theme="theme"
    language="zh-CN"
    preview-theme="github"
    code-theme="github"
    :toolbars="toolbars"
    :placeholder="placeholder"
    :style="{ height }"
    class="koi-markdown-editor"
    @on-upload-img="onUploadImg"
  />
</template>

<script setup lang="ts">
import { computed } from "vue";
import { MdEditor, type ToolbarNames } from "md-editor-v3";
import "md-editor-v3/lib/style.css";
import useGlobalStore from "@/stores/modules/global.ts";
import { uploadAssetApi } from "@/api/system/assets.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { koiMsgError } from "@/utils/koi.ts";

/** 文章/内容正文 Markdown 编辑器（基于 md-editor-v3） */
const model = defineModel<string>({ default: "" });

const props = withDefaults(
  defineProps<{
    /** 多实例并存时需唯一 id */
    editorId?: string;
    placeholder?: string;
    height?: string;
    /** 启用正文插图上传（purpose=content） */
    enableImageUpload?: boolean;
  }>(),
  {
    editorId: "koi-markdown-editor",
    placeholder: "",
    height: "min(520px, calc(100vh - 380px))",
    enableImageUpload: false,
  },
);

const globalStore = useGlobalStore();
const theme = computed(() => (globalStore.isDark ? "dark" : "light"));

const baseToolbars: ToolbarNames[] = [
  "bold",
  "underline",
  "italic",
  "-",
  "title",
  "strikeThrough",
  "quote",
  "unorderedList",
  "orderedList",
  "task",
  "-",
  "codeRow",
  "code",
  "link",
  "table",
  "-",
  "revoke",
  "next",
  "=",
  "pageFullscreen",
  "fullscreen",
  "preview",
  "catalog",
];

const toolbars = computed<ToolbarNames[]>(() => {
  if (!props.enableImageUpload) return baseToolbars;
  const items = [...baseToolbars];
  const linkIdx = items.indexOf("link");
  if (linkIdx >= 0) {
    items.splice(linkIdx + 1, 0, "image");
  } else {
    items.unshift("image");
  }
  return items;
});

async function onUploadImg(
  files: File[],
  callback: (urls: string[]) => void,
) {
  const urls: string[] = [];
  for (const file of files) {
    const res = await uploadAssetApi(file, "content");
    if (res.code !== 0 || !res.data) {
      koiMsgError(res.message || "图片上传失败");
      continue;
    }
    urls.push(resolveAssetUrl(res.data.url));
  }
  callback(urls);
}
</script>

<style scoped lang="scss">
.koi-markdown-editor {
  width: 100%;
  border-radius: var(--el-border-radius-base);
  overflow: hidden;
}
</style>
