<template>
  <img
    v-if="display.kind === 'image'"
    class="koi-global-icon koi-global-icon--img"
    :src="display.value"
    :alt="alt"
    :style="boxStyle"
  />
  <span
    v-else-if="display.kind === 'emoji'"
    class="koi-global-icon koi-global-icon--emoji"
    :style="emojiStyle"
    aria-hidden="true"
  >{{ display.value }}</span>
  <KoiSvgIcon
    v-else-if="display.kind === 'icon' && isLocalSvg"
    class="koi-global-icon koi-global-icon--svg"
    :name="display.value"
    :width="props.size"
    :height="props.size"
  />
  <el-icon v-else-if="display.kind === 'icon' && epIconComponent" :size="props.size">
    <component :is="epIconComponent" />
  </el-icon>
</template>

<script setup lang="ts">
import { computed, resolveComponent, type Component } from "vue";
import { SVG_PREFIX } from "@/config/index.ts";
import { parseIconDisplay } from "@/utils/iconDisplay.ts";
import { isLocalIconName } from "@/utils/localIcons.ts";

interface IGlobalIconProps {
  name?: string;
  size?: number | string;
  alt?: string;
}

const props = withDefaults(defineProps<IGlobalIconProps>(), {
  name: "",
  size: "18",
  alt: "",
});

const display = computed(() => parseIconDisplay(props.name));

const isLocalSvg = computed(
  () =>
    display.value.kind === "icon" &&
    display.value.value.startsWith(SVG_PREFIX) &&
    isLocalIconName(display.value.value),
);

/** Element Plus 图标：未注册时不渲染，避免空占位 */
const epIconComponent = computed((): Component | null => {
  if (display.value.kind !== "icon" || isLocalSvg.value) return null;
  const resolved = resolveComponent(display.value.value);
  return typeof resolved === "string" ? null : (resolved as Component);
});

const sizePx = computed(() => {
  const n = Number(props.size);
  return Number.isFinite(n) ? n : 18;
});

const boxStyle = computed(() => ({
  width: `${sizePx.value}px`,
  height: `${sizePx.value}px`,
}));

const emojiStyle = computed(() => ({
  fontSize: `${Math.round(sizePx.value * 0.92)}px`,
  lineHeight: `${sizePx.value}px`,
  width: `${sizePx.value}px`,
  height: `${sizePx.value}px`,
}));
</script>

<style lang="scss" scoped>
.koi-global-icon--img {
  display: inline-block;
  flex-shrink: 0;
  object-fit: contain;
  vertical-align: middle;
}

.koi-global-icon--emoji,
.koi-global-icon--svg {
  display: inline-flex;
  flex-shrink: 0;
  align-items: center;
  justify-content: center;
  vertical-align: middle;
}

.koi-global-icon--emoji {
  user-select: none;
}
</style>
