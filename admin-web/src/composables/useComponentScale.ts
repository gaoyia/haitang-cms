import { computed } from "vue";
import { storeToRefs } from "pinia";
import useGlobalStore from "@/stores/modules/global.ts";
import {
  componentScaleToCssVars,
  getComponentScale,
  type ComponentScaleTokens,
} from "@/utils/componentScale.ts";

/** 读取全局组件尺寸档位，并输出对应布局尺度 */
export function useComponentScale() {
  const { dimension } = storeToRefs(useGlobalStore());

  const scale = computed<ComponentScaleTokens>(() => getComponentScale(dimension.value));

  const cssVars = computed(() => componentScaleToCssVars(scale.value));

  const cardBodyStyle = computed(() => ({ padding: scale.value.cardPadding }));

  return { dimension, scale, cssVars, cardBodyStyle };
}
