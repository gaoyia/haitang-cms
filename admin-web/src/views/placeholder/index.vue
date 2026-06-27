<template>
  <div class="placeholder-page">
    <el-empty :description="description">
      <el-button type="primary" @click="router.push(HOME_URL)">返回首页</el-button>
    </el-empty>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { HOME_URL } from "@/config/index.ts";

const route = useRoute();
const router = useRouter();
const { t } = useI18n();

const description = computed(() => {
  const title = route.meta?.title;
  if (typeof title === "string" && title.startsWith("menu.")) {
    return `${t(title)} — 功能开发中，敬请期待`;
  }
  return "功能开发中，敬请期待";
});
</script>

<style scoped>
.placeholder-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 360px;
}
</style>
