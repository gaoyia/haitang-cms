import initJieba, { tag, type Tag } from "jieba-wasm";

/** 结巴 WASM 单例初始化 */
let jiebaReady: Promise<void> | null = null;

async function ensureJiebaReady(): Promise<void> {
  if (!jiebaReady) {
    jiebaReady = initJieba().then(() => undefined);
  }
  await jiebaReady;
}

/** 常见虚词 / 功能词，不作为标签候选 */
const STOP_WORDS = new Set([
  "的", "了", "在", "是", "我", "有", "和", "就", "不", "人", "都", "一", "上", "也", "很",
  "到", "说", "要", "去", "你", "会", "着", "没有", "看", "好", "自己", "这", "那", "等",
  "中", "为", "与", "及", "或", "被", "从", "对", "以", "而", "但", "如果", "可以", "我们",
  "他们", "它", "这个", "那个", "什么", "怎么", "如何", "以及", "因为", "所以", "已经",
  "进行", "通过", "使用", "需要", "可能", "其中", "以下", "以上", "相关", "内容", "文章",
  "图片", "链接", "点击", "查看", "更多", "欢迎", "关于", "本站", "网站", "页面",
]);

/** 保留名词、形容词、动名词、成语、英文词等可作标签的词性 */
const KEYWORD_POS = /^(n|nr|ns|nt|nz|ng|vn|a|ad|an|eng|i|j|l)/;

/** 将 Markdown 正文粗略转为纯文本，供分词使用 */
export function markdownToPlainText(raw: string): string {
  let text = raw;
  text = text.replace(/```[\s\S]*?```/g, " ");
  text = text.replace(/`[^`]*`/g, " ");
  text = text.replace(/!\[[^\]]*]\([^)]*\)/g, " ");
  text = text.replace(/\[([^\]]+)]\([^)]*\)/g, "$1");
  text = text.replace(/^#{1,6}\s+/gm, "");
  text = text.replace(/^\s*[-*+]\s+/gm, "");
  text = text.replace(/^\s*\d+\.\s+/gm, "");
  text = text.replace(/(\*\*|__|\*|_|~~)/g, "");
  text = text.replace(/<[^>]+>/g, " ");
  text = text.replace(/\s+/g, " ");
  return text.trim();
}

function isKeywordCandidate(word: string, pos: string): boolean {
  const w = word.trim();
  if (!w || STOP_WORDS.has(w)) return false;
  if (/^[\d\s.,，。！？!?;；:：\-—]+$/.test(w)) return false;
  if (w.length === 1 && !/[\u4e00-\u9fff]/.test(w)) return false;
  if (w.length < 2 && /[\u4e00-\u9fff]/.test(w)) return false;
  if (pos === "eng" && w.length < 3) return false;
  return KEYWORD_POS.test(pos);
}

function rankKeywords(tokens: Tag[]): string[] {
  const freq = new Map<string, number>();
  for (const { word, tag: pos } of tokens) {
    if (!isKeywordCandidate(word, pos)) continue;
    freq.set(word, (freq.get(word) ?? 0) + 1);
  }

  return [...freq.entries()]
    .sort((a, b) => {
      if (b[1] !== a[1]) return b[1] - a[1];
      return b[0].length - a[0].length;
    })
    .map(([word]) => word);
}

/**
 * 使用结巴分词从正文纯文本中提取关键词。
 * @param text 待分析正文（Markdown 应先转为纯文本）
 * @param count 期望关键词数量
 * @param existing 已有标签，避免重复
 */
export async function extractKeywordsWithJieba(
  text: string,
  count: number,
  existing: string[] = [],
): Promise<string[]> {
  await ensureJiebaReady();

  const normalized = text.trim();
  if (!normalized) return [];

  const limit = Math.max(1, Math.min(30, Math.floor(count)));
  const existingSet = new Set(existing.map((t) => t.trim()).filter(Boolean));
  const ranked = rankKeywords(tag(normalized, true));

  const result: string[] = [];
  for (const word of ranked) {
    if (existingSet.has(word)) continue;
    result.push(word);
    existingSet.add(word);
    if (result.length >= limit) break;
  }
  return result;
}
