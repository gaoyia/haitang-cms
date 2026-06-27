import fs from "node:fs";
import path from "node:path";

/** 与根目录 build.rs 相同：public/fileicon → static/fileicon */
export function syncFileIconsToStatic(rootDir: string): void {
  const src = path.resolve(rootDir, "public/fileicon");
  const dst = path.resolve(rootDir, "../static/fileicon");

  if (!fs.existsSync(src)) {
    console.warn(`[fileicon] 源目录不存在，已跳过: ${src}`);
    return;
  }

  fs.mkdirSync(dst, { recursive: true });
  for (const name of fs.readdirSync(src)) {
    if (!name.endsWith(".svg")) continue;
    fs.copyFileSync(path.join(src, name), path.join(dst, name));
  }
}

/** Vite 开发 / 构建时同步文件图标到 static（与 cargo build.rs 目标一致） */
export function syncFileIconsPlugin() {
  const rootDir = process.cwd();
  return {
    name: "sync-file-icons",
    configureServer() {
      syncFileIconsToStatic(rootDir);
    },
    closeBundle() {
      syncFileIconsToStatic(rootDir);
    },
  };
}
