<template>
  <div class="banner-hero-meta">
    <el-form-item :label="t('menu.banner.heroBadge')">
      <el-input v-model="locale.badge" :placeholder="t('menu.banner.heroBadgePh')" />
    </el-form-item>

    <el-form-item :label="t('menu.banner.heroTitle')">
      <el-input v-model="locale.title" :placeholder="t('menu.banner.heroTitlePh', emTagParams)" />
      <p class="field-hint">{{ t("menu.banner.heroTitleHint", emTagParams) }}</p>
    </el-form-item>

    <el-form-item :label="t('menu.banner.heroDesc')">
      <el-input v-model="locale.description" type="textarea" :rows="3" :placeholder="t('menu.banner.heroDescPh')" />
    </el-form-item>

    <el-form-item :label="t('menu.banner.heroTags')">
      <div class="banner-hero-meta__tags">
        <el-tag
          v-for="(tag, index) in locale.tags"
          :key="`${tag}-${index}`"
          closable
          @close="removeTag(index)"
        >
          {{ tag }}
        </el-tag>
        <div class="banner-hero-meta__tag-add">
          <el-input
            v-model="tagInput"
            size="small"
            :placeholder="t('menu.banner.heroTagPh')"
            @keyup.enter="addTag"
          />
          <el-button size="small" @click="addTag">{{ t("menu.banner.heroTagAdd") }}</el-button>
        </div>
      </div>
      <p class="field-hint">{{ t("menu.banner.heroTagsHint") }}</p>
    </el-form-item>

    <el-form-item :label="t('menu.banner.heroActions')">
      <div class="banner-hero-meta__actions">
        <div v-for="(action, index) in locale.actions" :key="index" class="banner-hero-meta__action-row">
          <el-input
            v-model="action.label"
            :placeholder="t('menu.banner.heroActionLabelPh')"
            class="banner-hero-meta__action-label"
          />
          <el-input
            v-model="action.url"
            :placeholder="t('menu.banner.heroActionUrlPh')"
            class="banner-hero-meta__action-url"
          />
          <el-select v-model="action.variant" class="banner-hero-meta__action-variant">
            <el-option :label="t('menu.banner.heroActionPrimary')" value="primary" />
            <el-option :label="t('menu.banner.heroActionSecondary')" value="secondary" />
          </el-select>
          <el-button type="danger" link @click="removeAction(index)">{{ t("button.delete") }}</el-button>
        </div>
        <el-button type="primary" link @click="addAction">
          {{ t("menu.banner.heroActionAdd") }}
        </el-button>
      </div>
      <p class="field-hint">{{ t("menu.banner.heroActionsHint") }}</p>
    </el-form-item>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import {
  emptyBannerHeroAction,
  emptyBannerHeroLocale,
  type BannerHeroLocale,
} from "@/utils/bannerMeta.ts";

const model = defineModel<BannerHeroLocale>();

function ensureModel(): BannerHeroLocale {
  if (!model.value) {
    model.value = emptyBannerHeroLocale();
  }
  return model.value;
}

const locale = computed({
  get: () => ensureModel(),
  set: (value: BannerHeroLocale) => {
    model.value = value;
  },
});

const { t } = useI18n();
/** 占位符传入 em 标签，避免 i18n 文案内写 HTML 触发 intlify XSS 警告 */
const emTagParams = { emOpen: "<em>", emClose: "</em>" };
const tagInput = ref("");

function addTag() {
  const value = tagInput.value.trim();
  if (!value) return;
  if (!locale.value.tags.includes(value)) {
    locale.value.tags.push(value);
  }
  tagInput.value = "";
}

function removeTag(index: number) {
  locale.value.tags.splice(index, 1);
}

function addAction() {
  locale.value.actions.push(emptyBannerHeroAction());
}

function removeAction(index: number) {
  locale.value.actions.splice(index, 1);
}
</script>

<style scoped lang="scss">
.banner-hero-meta__tags {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.banner-hero-meta__tag-add {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 200px;
}

.banner-hero-meta__actions {
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 100%;
}

.banner-hero-meta__action-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}

.banner-hero-meta__action-label {
  flex: 1 1 120px;
  min-width: 120px;
}

.banner-hero-meta__action-url {
  flex: 2 1 180px;
  min-width: 180px;
}

.banner-hero-meta__action-variant {
  width: 110px;
}

.field-hint {
  margin: 6px 0 0;
  font-size: 12px;
  color: var(--el-text-color-secondary);
  line-height: 1.5;
}
</style>
