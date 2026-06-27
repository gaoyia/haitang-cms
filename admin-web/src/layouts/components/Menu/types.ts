/** 纵向 / 经典侧栏菜单项（与 authStore.showMenuList 树节点一致） */
export interface AsideMenuItem {
  path: string;
  children?: AsideMenuItem[];
  meta?: {
    icon?: string;
    title?: string;
    linkUrl?: string;
    isVisible?: string;
  };
}
