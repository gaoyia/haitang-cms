/** 后端分页列表响应 */
export interface PageResult<T> {
  list: T[];
  total: number;
  page: number;
  page_size: number;
}

/** 分页请求参数 */
export interface PageParams {
  page?: number;
  page_size?: number;
}
