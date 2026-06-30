import { computed, type MaybeRefOrGetter, toValue } from "vue";
import { useBreakpoints } from "@vueuse/core";
import { breakpointsEnum } from "@/hooks/screen/index.ts";

/** 桌面端固定宽度，手机端（<768px）全屏 */
export function useResponsiveDrawerSize(desktopSize: MaybeRefOrGetter<string | number>) {
  const isMobile = useBreakpoints(breakpointsEnum).smaller("sm");
  return computed(() => (isMobile.value ? "100%" : String(toValue(desktopSize))));
}
