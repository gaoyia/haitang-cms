<template>
  <div class="split-group-mobile-bar">
    <el-select
      :model-value="modelValue ?? undefined"
      :placeholder="placeholder"
      filterable
      class="split-group-mobile-bar__select"
      @change="onSelect"
    >
      <el-option
        v-for="group in groups"
        :key="group.id"
        :label="optionLabel(group)"
        :value="group.id"
      />
    </el-select>
    <ActionsDropdown
      :items="actionItems"
      :label="t('table.operate')"
      :button-type="'primary'"
      :link="false"
      size="default"
      :disabled="actionItems.length === 0"
      @action="onAction"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import ActionsDropdown, { type ActionDropdownItem } from "@/components/ActionsDropdown.vue";

export interface SplitGroupOption {
  id: number;
  name: string;
  code?: string;
  readonly?: boolean;
}

const props = withDefaults(
  defineProps<{
    groups: SplitGroupOption[];
    modelValue: number | null;
    placeholder?: string;
    formatLabel?: (group: SplitGroupOption) => string;
    showAdd?: boolean;
    showEdit?: boolean;
    showDelete?: boolean;
  }>(),
  {
    placeholder: "",
    showAdd: true,
    showEdit: true,
    showDelete: true,
  },
);

const emit = defineEmits<{
  (e: "update:modelValue", id: number): void;
  (e: "add"): void;
  (e: "edit"): void;
  (e: "delete"): void;
}>();

const { t } = useI18n();

const actionItems = computed<ActionDropdownItem[]>(() => {
  const items: ActionDropdownItem[] = [];
  if (props.showAdd) {
    items.push({ key: "add", label: t("button.add") });
  }
  if (props.showEdit) {
    items.push({
      key: "edit",
      label: t("button.update"),
      disabled: props.modelValue === null,
    });
  }
  if (props.showDelete) {
    items.push({
      key: "delete",
      label: t("button.delete"),
      disabled: props.modelValue === null,
      danger: true,
    });
  }
  return items;
});

function optionLabel(group: SplitGroupOption): string {
  if (props.formatLabel) return props.formatLabel(group);
  return group.code ? `${group.name} (${group.code})` : group.name;
}

function onSelect(id: number) {
  emit("update:modelValue", id);
}

function onAction(key: string) {
  if (key === "add") emit("add");
  else if (key === "edit") emit("edit");
  else if (key === "delete") emit("delete");
}
</script>

<style scoped lang="scss">
.split-group-mobile-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 14px;
  flex-shrink: 0;

  &__select {
    flex: 1;
    min-width: 0;
  }
}
</style>
