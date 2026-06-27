import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";
import vueSetupExtend from "vite-plugin-vue-setup-extend";
import { createSvgIconsPlugin } from "vite-plugin-svg-icons";
import Unocss from "unocss/vite";
import path from "path";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd());
  return {
    plugins: [
      vue(),
      Unocss(),
      vueSetupExtend(),
      createSvgIconsPlugin({
        iconDirs: [path.resolve(process.cwd(), "src/assets/icons")],
        symbolId: "icon-[dir]-[name]",
      }),
    ],
    resolve: {
      alias: {
        "@": path.resolve("./src"),
        "~": path.resolve("./src"),
      },
    },
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: '@use "@/styles/variable.scss" as *;',
          quietDeps: true,
        },
      },
    },
    server: {
      host: "0.0.0.0",
      port: 5174,
      proxy: {
        [env.VITE_WEB_BASE_API]: {
          target: env.VITE_SERVER,
          changeOrigin: true,
          rewrite: (p) => p.replace(new RegExp("^" + env.VITE_WEB_BASE_API), ""),
        },
      },
    },
    esbuild: {
      drop: env.VITE_DROP_CONSOLE === "false" ? [] : ["console", "debugger"],
    },
  };
});
