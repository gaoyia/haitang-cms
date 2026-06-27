import { defineConfig, presetWind3, presetAttributify, presetIcons, presetTypography, transformerDirectives, transformerVariantGroup, } from "unocss";
export default defineConfig({
    shortcuts: [],
    presets: [
        presetWind3(),
        presetAttributify(),
        presetTypography(),
        presetIcons({ scale: 1.2, warn: true }),
    ],
    transformers: [transformerVariantGroup(), transformerDirectives()],
});
