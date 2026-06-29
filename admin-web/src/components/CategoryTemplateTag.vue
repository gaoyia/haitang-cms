<template>
  <el-tag :type="style.type" :effect="style.effect" size="small">
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
</script>
