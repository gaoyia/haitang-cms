---
name: 海棠 CMS
colors:
  surface: '#faf9f8'
  surface-dim: '#dadad9'
  surface-bright: '#faf9f8'
  surface-container-lowest: '#ffffff'
  surface-container-low: '#f4f3f2'
  surface-container: '#eeeeed'
  surface-container-high: '#e9e8e7'
  surface-container-highest: '#e3e2e1'
  on-surface: '#1a1c1c'
  on-surface-variant: '#5b403f'
  inverse-surface: '#2f3130'
  inverse-on-surface: '#f1f0f0'
  outline: '#8f6f6e'
  outline-variant: '#e4bebc'
  surface-tint: '#bb152c'
  primary: '#b7102a'
  on-primary: '#ffffff'
  primary-container: '#db313f'
  on-primary-container: '#fffbff'
  inverse-primary: '#ffb3b1'
  secondary: '#516169'
  on-secondary: '#ffffff'
  secondary-container: '#d2e2ec'
  on-secondary-container: '#55656d'
  tertiary: '#46634c'
  on-tertiary: '#ffffff'
  tertiary-container: '#5e7c64'
  on-tertiary-container: '#f6fff4'
  error: '#ba1a1a'
  on-error: '#ffffff'
  error-container: '#ffdad6'
  on-error-container: '#93000a'
  primary-fixed: '#ffdad8'
  primary-fixed-dim: '#ffb3b1'
  on-primary-fixed: '#410007'
  on-primary-fixed-variant: '#92001c'
  secondary-fixed: '#d5e5ef'
  secondary-fixed-dim: '#b9c9d3'
  on-secondary-fixed: '#0e1d25'
  on-secondary-fixed-variant: '#3a4951'
  tertiary-fixed: '#caebce'
  tertiary-fixed-dim: '#afceb3'
  on-tertiary-fixed: '#05210f'
  on-tertiary-fixed-variant: '#314d38'
  background: '#faf9f8'
  on-background: '#1a1c1c'
  surface-variant: '#e3e2e1'
typography:
  display-lg:
    fontFamily: Inter
    fontSize: 48px
    fontWeight: '700'
    lineHeight: 56px
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '600'
    lineHeight: 40px
    letterSpacing: -0.01em
  headline-md:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
    letterSpacing: -0.01em
  title-lg:
    fontFamily: Inter
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 28px
    letterSpacing: '0'
  body-lg:
    fontFamily: Inter
    fontSize: 18px
    fontWeight: '400'
    lineHeight: 28px
    letterSpacing: '0'
  body-md:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: 24px
    letterSpacing: '0'
  label-md:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '500'
    lineHeight: 20px
    letterSpacing: 0.02em
  label-sm:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '600'
    lineHeight: 16px
    letterSpacing: 0.05em
  headline-lg-mobile:
    fontFamily: Inter
    fontSize: 28px
    fontWeight: '600'
    lineHeight: 36px
    letterSpacing: -0.01em
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  base: 8px
  xs: 4px
  sm: 12px
  md: 24px
  lg: 40px
  xl: 64px
  gutter: 24px
  margin-mobile: 16px
  margin-desktop: 32px
---

## 品牌与风格
本 CMS 的设计体系建立在「结构化优雅」（Structured Elegance）这一理念之上。它在内容管理平台对高效实用性的要求，与精致、受花卉启发的审美之间取得平衡。目标用户包括编辑团队、数字营销人员以及开发者——他们希望工作空间像高端工作室，而非冰冷的数据中心。

设计风格为**现代企业风与触感柔和**（Modern Corporate with Tactile Softness）。设计大量运用留白、精炼的色板以及微妙的层次深度，营造平静而高效的氛围。通过将极简布局与温暖、有机的点缀相结合，界面既足够专业、适合企业使用，又显得亲切且高端。

## 色彩
色板以**海棠红**（Begonia Red）为核心——这是一种鲜明而精致的深红色，审慎地用于主要操作与品牌呈现。

- **主色（海棠红）：** 用于高影响力交互与品牌识别。
- **辅色（炭灰）：** 用于文字与图标，确保高可读性，并呈现沉稳的专业感。
- **背景（柔和珍珠白）：** 带有暖色调的白色，相比纯白（#FFFFFF）更护眼，提供具有品质感的画布。
- **强调色（鼠尾草绿）：** 低饱和度的有机绿色，用于成功状态与增长指标，呼应「海棠」的花卉意象。
- **中性色：** 一系列柔和灰色，用于边框与次级表面，保持清晰、分层的视觉层级。

## 字体
本设计体系选用 **Inter** 字体，因其在数据密集场景下具有出色的清晰度。字阶体系面向编辑场景下的精确排版而设计。

- **标题：** 采用较紧的字距与半粗字重，形成清晰的视觉锚点。
- **正文：** 设置较宽松的行高（1.5 倍）与标准字距，便于长文内容的编辑与审阅。
- **标签：** 使用略重的字重与较大的字距，便于快速扫读元数据与界面控件。
- **层级：** 通过字重而非字体族，在「系统界面」（标签/按钮）与「用户内容」（标题/正文）之间保持清晰区分。

## 布局与间距
布局遵循**流体网格**（Fluid Grid）理念，强调「充足留白」。这确保 CMS 在管理复杂数据集时也不会显得拥挤。

- **网格：** 桌面端采用 12 列系统，移动端折叠为 4 列。
- **槽宽（Gutter）：** 固定为 24px，在内容组件之间提供清晰的视觉分隔。
- **节奏：** 以 8px 为基础单位统辖所有尺寸。容器内边距多数使用 `md`（24px），以保持品牌所追求的「开阔」感。
- **安全区域：** 侧边栏最小宽度为 260px；主内容区理想最大宽度为 1280px，以获得最佳阅读行长。

## 层级与深度
设计体系通过**色调分层**（Tonal Layers）与**环境阴影**（Ambient Shadows）组合，营造有序堆叠的空间感。

- **第 0 层（基础）：** 柔和珍珠白（#FDFCFB）。
- **第 1 层（卡片/侧边栏）：** 纯白表面，配 1px 边框（#E9ECEF），无阴影。
- **第 2 层（激活/悬停）：** 柔和弥散阴影（0px 4px 12px rgba(47, 62, 70, 0.05)），表示可交互。
- **第 3 层（模态框/弹出层）：** 更高层级，使用更深阴影（0px 12px 24px rgba(47, 62, 70, 0.12)），并对下层施加轻微背景模糊。

## 形状
与「海棠」主题一致，形状语言采用**圆角**风格。避免尖锐直角，以保持温暖、友好的专业氛围。

- **标准圆角：** 8px（0.5rem），用于按钮、输入框与小型组件。
- **大圆角：** 16px（1rem），用于卡片与主仪表盘容器。
- **全圆角：** 专用于标签、头像与搜索栏，在合适处提供视觉变化与「胶囊」形态。

## 组件
以下组件的一致实现，确保 CMS 呈现为统一、完整的工具。

- **按钮：**
  - *主要按钮：* 海棠红背景、白色文字；悬停时添加柔和阴影。
  - *次要按钮：* 透明背景，炭灰边框与文字。
  - *幽灵按钮：* 无边框，炭灰文字；悬停时显示珍珠白背景。
- **输入框：** 柔和珍珠白背景，1px 灰色（Grey-200）边框；聚焦时过渡为 2px 海棠红边框，并带轻微光晕。
- **卡片：** 白色背景，16px 圆角，1px 细边框；内部内容使用 24px 内边距。
- **芯片/标签：** 鼠尾草绿或海棠红以 10% 不透明度作为背景，文字使用对应色 100% 不透明度；用于状态指示与分类标签。
- **列表：** 简洁行布局，1px 底部分隔线；悬停状态使用轻微珍珠白着色，表示选中。
- **导航：** 侧边栏采用垂直堆叠的「图标 + 标签」组合；激活状态通过左侧 4px 宽的海棠红竖条标识。
