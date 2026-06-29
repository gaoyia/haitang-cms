<template>
  <div class="tag-extract">
    <span class="tag-extract__count-label">
      {{ t("menu.content.post.manage.tagExtractCount") }}
    </span>
    <el-input-number
      v-model="extractCount"
      size="small"
      :min="1"
      :max="30"
      :step="1"
      controls-position="right"
      class="tag-extract__count"
    />
    <el-dropdown
      split-button
      size="small"
      type="primary"
      :loading="loading"
      @click="handleJiebaExtract"
      @command="handleCommand"
    >
      {{ t("menu.content.post.manage.tagExtract") }}
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item command="llm">
            {{ t("menu.content.post.manage.tagExtractLlm") }}
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { extractKeywordsWithJieba, markdownToPlainText } from "@/utils/jiebaKeywords.ts";
import { normalizeTagList } from "@/utils/tagList.ts";
import { koiMsgError, koiMsgInfo, koiMsgSuccess, koiMsgWarning } from "@/utils/koi.ts";

defineOptions({ name: "TagExtract" });

const props = defineProps<{
  /** 待分析正文（Markdown） */
  content: string;
  /** 当前标签列表 */
  modelValue?: string[] | null;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string[]];
}>();

const { t } = useI18n();
const extractCount = ref(5);
const loading = ref(false);

function handleCommand(command: string | number | object) {
  if (command === "llm") {
    koiMsgInfo(t("menu.content.post.manage.tagExtractLlmPending"));
  }
}

async function handleJiebaExtract() {
  const plainContent = markdownToPlainText(props.content);
  if (!plainContent) {
    koiMsgWarning(t("menu.content.post.manage.tagExtractEmpty"));
    return;
  }

  loading.value = true;
  try {
    const currentTags = normalizeTagList(props.modelValue);
    const keywords = await extractKeywordsWithJieba(
      plainContent,
      extractCount.value,
      currentTags,
    );
    if (!keywords.length) {
      koiMsgWarning(t("menu.content.post.manage.tagExtractNoResult"));
      return;
    }
    emit("update:modelValue", [...currentTags, ...keywords]);
    koiMsgSuccess(t("menu.content.post.manage.tagExtractSuccess", { count: keywords.length }));
  } catch {
    koiMsgError(t("menu.content.post.manage.tagExtractFail"));
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped lang="scss">
.tag-extract {
  display: inline-flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}

.tag-extract__count-label {
  font-size: 12px;
  line-height: 1;
  color: var(--el-text-color-secondary);
  white-space: nowrap;
}

.tag-extract__count {
  width: 88px;

  :deep(.el-input__wrapper) {
    padding-left: 8px;
    padding-right: 28px;
  }
}
</style>
