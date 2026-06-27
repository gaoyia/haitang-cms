<template>
  <MdEditor
    :id="editorId"
    ref="editorRef"
    v-model="model"
    :theme="theme"
    language="zh-CN"
    preview-theme="github"
    code-theme="github"
    :toolbars="resolvedToolbars"
    :preview="showSplitPreview"
    :placeholder="placeholder"
    :style="{ height: resolvedHeight }"
    class="koi-markdown-editor"
    @on-upload-img="onUploadImg"
  >
    <template v-if="isCompact" #defToolbars>
      <CompactToolMenu :sections="compactMenuSections" @command="runToolCommand" />
    </template>
  </MdEditor>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";
import { useBreakpoints } from "@vueuse/core";
import {
  MdEditor,
  type ExposeParam,
  type ToolbarNames,
} from "md-editor-v3";
import "md-editor-v3/lib/style.css";
import useGlobalStore from "@/stores/modules/global.ts";
import { uploadAssetApi } from "@/api/system/assets.ts";
import { resolveAssetUrl } from "@/utils/siteAsset.ts";
import { koiMsgError } from "@/utils/koi.ts";
import { breakpointsEnum } from "@/hooks/screen/index.ts";
import CompactToolMenu from "./CompactToolMenu.vue";
import {
  buildCompactMenuSections,
  compactToolbars,
  COMPACT_FULLSCREEN,
  COMPACT_PAGE_FULLSCREEN,
  fullToolbars,
  withImageUpload,
} from "./toolbar.ts";

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
    height: undefined,
    enableImageUpload: false,
  },
);

const globalStore = useGlobalStore();
const theme = computed(() => (globalStore.isDark ? "dark" : "light"));

const editorRef = ref<ExposeParam>();

let compactPreviewListenerBound = false;

/** 宽度 < 768px（与 admin-ui 表单 xs 断点一致） */
const breakpoints = useBreakpoints(breakpointsEnum);
const isCompact = breakpoints.smaller("sm");

const showSplitPreview = computed(() => !isCompact.value);

const resolvedHeight = computed(() => {
  if (props.height) return props.height;
  if (isCompact.value) return "min(360px, calc(100vh - 220px))";
  return "min(520px, calc(100vh - 380px))";
});

const resolvedToolbars = computed<ToolbarNames[]>(() => {
  const base = isCompact.value ? compactToolbars : fullToolbars;
  if (!props.enableImageUpload || isCompact.value) {
    return base;
  }
  return withImageUpload(base);
});

const compactMenuSections = computed(() => buildCompactMenuSections(props.enableImageUpload));

function syncCompactPreview(open: boolean) {
  if (!isCompact.value) return;
  nextTick(() => editorRef.value?.togglePreviewOnly(open));
}

function bindCompactPreviewListener() {
  if (compactPreviewListenerBound || !editorRef.value) return;
  editorRef.value.on("preview", (open) => {
    if (!isCompact.value) return;
    syncCompactPreview(open);
  });
  compactPreviewListenerBound = true;
}

function runToolCommand(command: string) {
  if (command === COMPACT_PAGE_FULLSCREEN) {
    editorRef.value?.togglePageFullscreen();
    return;
  }
  if (command === COMPACT_FULLSCREEN) {
    editorRef.value?.toggleFullscreen();
    return;
  }
  editorRef.value?.execCommand(command as Parameters<ExposeParam["execCommand"]>[0]);
  nextTick(() => editorRef.value?.focus());
}

watch(editorRef, () => bindCompactPreviewListener(), { immediate: true });

watch(isCompact, (compact) => {
  nextTick(() => {
    if (compact) {
      editorRef.value?.togglePreview(false);
      editorRef.value?.toggleCatalog(false);
      return;
    }
    editorRef.value?.togglePreviewOnly(false);
  });
}, { immediate: true });

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
