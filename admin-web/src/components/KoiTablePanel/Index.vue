<template>
  <div class="koi-table-panel">
    <div class="koi-table-panel__body">
      <el-table v-loading="loading" :data="data" v-bind="$attrs">
        <slot />
        <template #empty>
          <slot name="empty">
            <el-empty :description="emptyText ?? t('msg.null')" />
          </slot>
        </template>
      </el-table>
    </div>
    <div v-if="showPagination" class="koi-table-panel__footer">
      <el-pagination
        :current-page="page"
        :page-size="pageSize"
        :total="total"
        :page-sizes="pageSizes"
        background
        layout="total, sizes, prev, pager, next"
        @current-change="onPageChange"
        @size-change="onSizeChange"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";

defineOptions({ inheritAttrs: false });

const props = withDefaults(
  defineProps<{
    loading?: boolean;
    data: unknown[];
    total: number;
    page: number;
    pageSize: number;
    emptyText?: string;
    pageSizes?: number[];
  }>(),
  {
    loading: false,
    pageSizes: () => [10, 20, 50],
  },
);

const emit = defineEmits<{
  "update:page": [number];
  "update:pageSize": [number];
  change: [];
}>();

const { t } = useI18n();

const showPagination = computed(() => props.total > props.pageSize);

function onPageChange(value: number) {
  emit("update:page", value);
  emit("change");
}

function onSizeChange(value: number) {
  emit("update:pageSize", value);
  emit("update:page", 1);
  emit("change");
}
</script>
