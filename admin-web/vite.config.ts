import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";
import vueSetupExtend from "vite-plugin-vue-setup-extend";
import { createSvgIconsPlugin } from "vite-plugin-svg-icons";
import Unocss from "unocss/vite";
import path from "path";

/** 规范化 Vite base（必须以 / 开头和结尾） */
function normalizeBase(raw: string | undefined): string {
  if (!raw?.trim() || raw.trim() === "/") {
    return "/";
  }
  let base = raw.trim();
  if (!base.startsWith("/")) {
    base = `/${base}`;
  }
  if (!base.endsWith("/")) {
    base = `${base}/`;
  }
  return base;
}

function resolveOutDir(raw: string | undefined): string {
  const dir = raw?.trim() || "dist";
  return path.isAbsolute(dir) ? dir : path.resolve(process.cwd(), dir);
}

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd());
  const base = normalizeBase(env.VITE_BASE);
  return {
    base,
    build: {
      outDir: resolveOutDir(env.VITE_BUILD_OUT_DIR),
      emptyOutDir: true,
    },
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
