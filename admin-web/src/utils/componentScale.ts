/** Element Plus 全局组件尺寸档位 */
export type ElementDimension = "large" | "default" | "small";

/** 各档位下的布局与展示尺度（工作台等页面共用） */
export interface ComponentScaleTokens {
  gap: number;
  cardPadding: string;
  avatarSize: number;
  statIconBox: number;
  statIconSize: number;
  quickIconBox: number;
  quickIconSize: number;
  arrowIconSize: number;
  panelMinHeight: number;
}

export const COMPONENT_SCALE: Record<ElementDimension, ComponentScaleTokens> = {
  large: {
    gap: 24,
    cardPadding: "20px 24px",
    avatarSize: 64,
    statIconBox: 52,
    statIconSize: 26,
    quickIconBox: 44,
    quickIconSize: 20,
    arrowIconSize: 16,
    panelMinHeight: 300,
  },
  default: {
    gap: 20,
    cardPadding: "16px 20px",
    avatarSize: 56,
    statIconBox: 46,
    statIconSize: 22,
    quickIconBox: 40,
    quickIconSize: 18,
    arrowIconSize: 14,
    panelMinHeight: 280,
  },
  small: {
    gap: 16,
    cardPadding: "12px 16px",
    avatarSize: 48,
    statIconBox: 40,
    statIconSize: 18,
    quickIconBox: 36,
    quickIconSize: 16,
    arrowIconSize: 12,
    panelMinHeight: 260,
  },
};

export function getComponentScale(dimension?: string): ComponentScaleTokens {
  if (dimension === "large" || dimension === "small") {
    return COMPONENT_SCALE[dimension];
  }
  return COMPONENT_SCALE.default;
}

/** 转为 CSS 变量，供工作台等页面根节点绑定 */
export function componentScaleToCssVars(scale: ComponentScaleTokens): Record<string, string> {
  return {
    "--home-gap": `${scale.gap}px`,
    "--home-card-padding": scale.cardPadding,
    "--home-avatar-size": `${scale.avatarSize}px`,
    "--home-stat-icon-box": `${scale.statIconBox}px`,
    "--home-stat-icon-size": `${scale.statIconSize}px`,
    "--home-quick-icon-box": `${scale.quickIconBox}px`,
    "--home-quick-icon-size": `${scale.quickIconSize}px`,
    "--home-panel-min-height": `${scale.panelMinHeight}px`,
  };
}
