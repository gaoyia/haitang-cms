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
  />
</template>

<script setup lang="ts">
import { computed } from "vue";
import { MdEditor, type ToolbarNames } from "md-editor-v3";
import "md-editor-v3/lib/style.css";
import useGlobalStore from "@/stores/modules/global.ts";

/** 文章/内容正文 Markdown 编辑器（基于 md-editor-v3） */
const model = defineModel<string>({ default: "" });

const props = withDefaults(
  defineProps<{
    /** 多实例并存时需唯一 id */
    editorId?: string;
    placeholder?: string;
    height?: string;
  }>(),
  {
    editorId: "koi-markdown-editor",
    placeholder: "",
    height: "min(520px, calc(100vh - 380px))",
  },
);

const globalStore = useGlobalStore();
const theme = computed(() => (globalStore.isDark ? "dark" : "light"));

/** 默认工具栏：不含图片上传（未接上传接口） */
const toolbars: ToolbarNames[] = [
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
</script>

<style scoped lang="scss">
.koi-markdown-editor {
  width: 100%;
  border-radius: var(--el-border-radius-base);
  overflow: hidden;
}
</style>
