<template>
  <el-dropdown trigger="click" @command="onCommand">
    <el-button
      :type="buttonType"
      :link="link"
      :size="size"
      :disabled="disabled"
      @click.stop
    >
      <slot>{{ label || t("table.operate") }}</slot>
      <el-icon v-if="showArrow" class="actions-dropdown__arrow"><ArrowDown /></el-icon>
    </el-button>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item
          v-for="(item, index) in items"
          :key="item.key"
          :command="item.key"
          :disabled="item.disabled"
          :divided="item.divided ?? (index > 0 && item.danger)"
        >
          <span :class="{ 'actions-dropdown__danger': item.danger }">{{ item.label }}</span>
        </el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import { ArrowDown } from "@element-plus/icons-vue";
import { useI18n } from "vue-i18n";

export interface ActionDropdownItem {
  key: string;
  label: string;
  disabled?: boolean;
  danger?: boolean;
  divided?: boolean;
}

withDefaults(
  defineProps<{
    items: ActionDropdownItem[];
    label?: string;
    buttonType?: "" | "default" | "primary" | "success" | "warning" | "info" | "danger";
    link?: boolean;
    size?: "small" | "default" | "large";
    disabled?: boolean;
    showArrow?: boolean;
  }>(),
  {
    label: "",
    buttonType: "primary",
    link: true,
    size: "small",
    disabled: false,
    showArrow: true,
  },
);

const emit = defineEmits<{
  (e: "action", key: string): void;
}>();

const { t } = useI18n();

function onCommand(key: string) {
  emit("action", key);
}
</script>

<style scoped lang="scss">
.actions-dropdown__arrow {
  margin-left: 2px;
}

.actions-dropdown__danger {
  color: var(--el-color-danger);
}
</style>
