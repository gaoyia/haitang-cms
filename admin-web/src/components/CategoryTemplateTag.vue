<template>
  <el-tag v-bind="tagAttrs" size="small">
    {{ label }}
  </el-tag>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { CategoryListTemplate } from "@/api/system/categories.ts";
import { getCategoryTemplateTagStyle } from "@/utils/categoryTemplate.ts";

const props = defineProps<{
  template: CategoryListTemplate;
  label: string;
}>();

const style = computed(() => getCategoryTemplateTagStyle(props.template));

/** 无 type 时不传该 prop，避免 ElTag 校验警告 */
const tagAttrs = computed(() => {
  const { type, effect } = style.value;
  return type ? { type, effect } : { effect };
});
</script>
