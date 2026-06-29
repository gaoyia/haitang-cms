<template>
  <el-popover
    placement="bottom-start"
    :width="380"
    trigger="click"
    popper-class="public-url-popover"
  >
    <template #reference>
      <el-button link type="primary" class="public-url-trigger" @click.stop>
        <el-icon><Link /></el-icon>
      </el-button>
    </template>
    <div class="public-url-panel">
      <div class="public-url-item">
        <el-tag size="small" type="info" effect="plain">{{ t("common.publicUrlLinkId") }}</el-tag>
        <a
          :href="idUrl"
          target="_blank"
          rel="noopener noreferrer"
          class="public-url-item__link"
          :title="absoluteUrl(idUrl)"
        >
          {{ idUrl }}
        </a>
        <el-button
          link
          type="primary"
          class="public-url-item__copy"
          :title="t('common.copyLink')"
          @click="copyUrl(idUrl)"
        >
          <el-icon><CopyDocument /></el-icon>
        </el-button>
      </div>
      <div v-if="seoUrl" class="public-url-item">
        <el-tag size="small" type="success" effect="plain">{{ t("common.publicUrlLinkSeo") }}</el-tag>
        <a
          :href="seoUrl"
          target="_blank"
          rel="noopener noreferrer"
          class="public-url-item__link"
          :title="absoluteUrl(seoUrl)"
        >
          {{ seoUrl }}
        </a>
        <el-button
          link
          type="primary"
          class="public-url-item__copy"
          :title="t('common.copyLink')"
          @click="copyUrl(seoUrl)"
        >
          <el-icon><CopyDocument /></el-icon>
        </el-button>
      </div>
    </div>
  </el-popover>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { CopyDocument, Link } from "@element-plus/icons-vue";
import { koiMsgError, koiMsgSuccess } from "@/utils/koi.ts";

defineProps<{
  idUrl: string;
  seoUrl?: string | null;
}>();

const { t } = useI18n();

function absoluteUrl(path: string): string {
  try {
    return new URL(path, window.location.origin).href;
  } catch {
    return path;
  }
}

async function copyUrl(path: string) {
  const text = absoluteUrl(path);
  try {
    if (navigator.clipboard?.writeText) {
      await navigator.clipboard.writeText(text);
    } else {
      const ta = document.createElement("textarea");
      ta.value = text;
      ta.style.position = "fixed";
      ta.style.opacity = "0";
      document.body.appendChild(ta);
      ta.select();
      document.execCommand("copy");
      document.body.removeChild(ta);
    }
    koiMsgSuccess(t("common.copyLinkSuccess"));
  } catch {
    koiMsgError(t("common.copyLinkFailed"));
  }
}
</script>

<style scoped lang="scss">
.public-url-trigger {
  flex-shrink: 0;
  padding: 0 2px;
  height: 22px;
  vertical-align: middle;
}

.public-url-panel {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.public-url-item {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;

  &__link {
    flex: 1;
    min-width: 0;
    font-size: 13px;
    color: var(--el-color-primary);
    text-decoration: none;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    &:hover {
      text-decoration: underline;
    }
  }

  &__copy {
    flex-shrink: 0;
    padding: 0 4px;
    font-size: 16px;
  }
}
</style>
