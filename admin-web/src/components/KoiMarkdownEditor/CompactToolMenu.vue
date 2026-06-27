<template>
  <el-dropdown
    ref="dropdownRef"
    trigger="click"
    placement="bottom-start"
    :hide-on-click="false"
  >
    <button type="button" class="md-editor-toolbar-item koi-md-compact-tool-btn" title="工具">
      <el-icon class="koi-md-compact-tool-btn__icon"><Menu /></el-icon>
      <span>工具</span>
    </button>
    <template #dropdown>
      <div class="koi-md-tool-panel">
        <div class="koi-md-tool-panel__fullscreen">
          <button
            v-for="item in sections.fullscreen"
            :key="item.command"
            type="button"
            class="koi-md-tool-panel__btn"
            @click="pick(item.command)"
          >
            <ToolEntry :item="item" />
          </button>
        </div>
        <div class="koi-md-tool-panel__divider" role="separator" />
        <div class="koi-md-tool-panel__grid">
          <button
            v-for="item in sections.tools"
            :key="item.command"
            type="button"
            class="koi-md-tool-panel__btn"
            @click="pick(item.command)"
          >
            <ToolEntry :item="item" />
          </button>
        </div>
      </div>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import { ref } from "vue";
import type { DropdownInstance } from "element-plus";
import { Menu } from "@element-plus/icons-vue";
import ToolEntry from "./CompactToolEntry.vue";
import type { CompactMenuSections } from "./toolbar.ts";

defineProps<{
  sections: CompactMenuSections;
}>();

const emit = defineEmits<{
  (e: "command", command: string): void;
}>();

const dropdownRef = ref<DropdownInstance>();

function pick(command: string) {
  emit("command", command);
  dropdownRef.value?.handleClose();
}
</script>

<style scoped lang="scss">
.koi-md-compact-tool-btn {
  display: inline-flex;
  flex-direction: row !important;
  align-items: center;
  gap: 4px;
  font-size: 13px;
  line-height: 1;
  white-space: nowrap;

  &__icon {
    font-size: 14px;
    flex-shrink: 0;
  }
}

.koi-md-tool-panel {
  width: min(300px, 88vw);
  padding: 8px;
  box-sizing: border-box;

  &__fullscreen {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 6px;
  }

  &__divider {
    margin: 8px 0;
    border-top: 1px solid var(--el-border-color-lighter);
  }

  &__grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 6px;
  }

  &__btn {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    width: 100%;
    min-height: 36px;
    margin: 0;
    padding: 8px 10px;
    border: none;
    border-radius: var(--el-border-radius-base);
    background: transparent;
    color: var(--el-text-color-primary);
    font-size: 13px;
    line-height: 1.3;
    cursor: pointer;
    text-align: left;
    transition: background 0.15s, color 0.15s;

    &:hover {
      background: var(--el-fill-color-light);
      color: var(--el-color-primary);
    }
  }
}
</style>
