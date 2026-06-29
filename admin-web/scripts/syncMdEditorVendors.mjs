/**
 * 将 md-editor-v3 默认 unpkg 依赖同步到 public/md-editor-vendors/。
 * 版本须与 node_modules/md-editor-v3 内置 CDN 地址一致（升级 md-editor 后请重新执行）。
 */
import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const outDir = path.join(root, "public", "md-editor-vendors");

/** @type {{ url: string; file: string }[]} */
const MANIFEST = [
  {
    url: "https://unpkg.com/@highlightjs/cdn-assets@11.11.1/highlight.min.js",
    file: "highlight.min.js",
  },
  {
    url: "https://unpkg.com/@highlightjs/cdn-assets@11.11.1/styles/github.min.css",
    file: "styles/github.min.css",
  },
  {
    url: "https://unpkg.com/@highlightjs/cdn-assets@11.11.1/styles/github-dark.min.css",
    file: "styles/github-dark.min.css",
  },
  {
    url: "https://unpkg.com/katex@0.16.33/dist/katex.min.js",
    file: "katex.min.js",
  },
  {
    url: "https://unpkg.com/katex@0.16.33/dist/katex.min.css",
    file: "katex.min.css",
  },
  {
    url: "https://unpkg.com/mermaid@11.12.3/dist/mermaid.min.js",
    file: "mermaid.min.js",
  },
  {
    url: "https://unpkg.com/echarts@6.0.0/dist/echarts.min.js",
    file: "echarts.min.js",
  },
  {
    url: "https://unpkg.com/cropperjs@1.6.2/dist/cropper.min.js",
    file: "cropper.min.js",
  },
  {
    url: "https://unpkg.com/cropperjs@1.6.2/dist/cropper.min.css",
    file: "cropper.min.css",
  },
  {
    url: "https://unpkg.com/prettier@3.8.1/standalone.js",
    file: "prettier.standalone.js",
  },
  {
    url: "https://unpkg.com/prettier@3.8.1/plugins/markdown.js",
    file: "prettier-markdown.js",
  },
  {
    url: "https://unpkg.com/screenfull@5.2.0/dist/screenfull.js",
    file: "screenfull.js",
  },
];

async function downloadOne({ url, file }) {
  const dest = path.join(outDir, file);
  await mkdir(path.dirname(dest), { recursive: true });
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(`下载失败 ${url}: HTTP ${res.status}`);
  }
  const buf = Buffer.from(await res.arrayBuffer());
  await writeFile(dest, buf);
  console.log(`OK ${file} (${(buf.length / 1024).toFixed(1)} KiB)`);
}

async function main() {
  console.log(`同步到 ${outDir}`);
  for (const item of MANIFEST) {
    await downloadOne(item);
  }
  console.log("完成。请提交 public/md-editor-vendors/ 以便离线构建。");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
