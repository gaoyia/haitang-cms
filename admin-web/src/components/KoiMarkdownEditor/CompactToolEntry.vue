<template>
  <span class="koi-md-tool-entry">
    <span
      v-if="item.glyph"
      class="koi-md-tool-entry__glyph"
      :class="item.glyphClass && `koi-md-tool-entry__glyph--${item.glyphClass}`"
    >{{ item.glyph }}</span>
    <el-icon v-else-if="item.icon" class="koi-md-tool-entry__icon">
      <component :is="item.icon" />
    </el-icon>
    <span class="koi-md-tool-entry__label">{{ item.label }}</span>
  </span>
</template>

<script setup lang="ts">
import type { CompactToolItem } from "./toolbar.ts";

defineProps<{
  item: CompactToolItem;
}>();
</script>

<style scoped lang="scss">
.koi-md-tool-entry {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
  white-space: nowrap;

  &__icon {
    flex-shrink: 0;
    font-size: 15px;
    color: var(--el-text-color-secondary);
  }

  &__label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  &__glyph {
    display: inline-flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: center;
    width: 16px;
    font-size: 13px;
    color: var(--el-text-color-secondary);

    &--bold {
      font-weight: 700;
    }

    &--italic {
      font-style: italic;
    }

    &--underline {
      text-decoration: underline;
    }

    &--strike {
      text-decoration: line-through;
    }

    &--mono {
      width: auto;
      min-width: 22px;
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      font-size: 10px;
      letter-spacing: -0.02em;
      white-space: nowrap;
    }
  }
}
</style>
