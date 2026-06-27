import { computed, ref } from "vue";
import type { PageParams, PageResult } from "@/types/page.ts";

/** 管理端表格分页状态 */
export function useTablePage(defaultPageSize = 10) {
  const page = ref(1);
  const pageSize = ref(defaultPageSize);
  const total = ref(0);

  const showPagination = computed(() => total.value > pageSize.value);

  const pageParams = computed<PageParams>(() => ({
    page: page.value,
    page_size: pageSize.value,
  }));

  function applyPageResult<T>(data: PageResult<T> | undefined | null): T[] {
    if (!data) {
      total.value = 0;
      return [];
    }
    total.value = data.total;
    page.value = data.page;
    pageSize.value = data.page_size;
    return data.list;
  }

  function resetPage() {
    page.value = 1;
  }

  return {
    page,
    pageSize,
    total,
    showPagination,
    pageParams,
    applyPageResult,
    resetPage,
  };
}
