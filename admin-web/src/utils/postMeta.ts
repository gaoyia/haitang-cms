/** 招聘岗位 meta_json 字段 */
export interface RecruitmentPostMeta {
  salary: string;
  location: string;
  employment_type: string;
  department: string;
}

export function emptyRecruitmentMeta(): RecruitmentPostMeta {
  return {
    salary: "",
    location: "",
    employment_type: "",
    department: "",
  };
}

export function parseRecruitmentMeta(raw?: string): RecruitmentPostMeta {
  if (!raw?.trim()) return emptyRecruitmentMeta();
  try {
    const obj = JSON.parse(raw) as Record<string, unknown>;
    if (!obj || typeof obj !== "object") return emptyRecruitmentMeta();
    return {
      salary: String(obj.salary ?? ""),
      location: String(obj.location ?? ""),
      employment_type: String(obj.employment_type ?? ""),
      department: String(obj.department ?? ""),
    };
  } catch {
    return emptyRecruitmentMeta();
  }
}

/** 序列化为 meta_json 字符串（空字段省略） */
export function buildRecruitmentMetaJson(meta: RecruitmentPostMeta): string {
  const payload: Record<string, string> = {};
  for (const [key, value] of Object.entries(meta)) {
    const trimmed = value.trim();
    if (trimmed) payload[key] = trimmed;
  }
  return JSON.stringify(payload);
}

/** 关于我们 meta_json 字段 */
export interface AboutPostMeta {
  highlight: string;
  founded: string;
  location: string;
  contact: string;
}

export function emptyAboutMeta(): AboutPostMeta {
  return {
    highlight: "",
    founded: "",
    location: "",
    contact: "",
  };
}

export function parseAboutMeta(raw?: string): AboutPostMeta {
  if (!raw?.trim()) return emptyAboutMeta();
  try {
    const obj = JSON.parse(raw) as Record<string, unknown>;
    if (!obj || typeof obj !== "object") return emptyAboutMeta();
    return {
      highlight: String(obj.highlight ?? ""),
      founded: String(obj.founded ?? ""),
      location: String(obj.location ?? ""),
      contact: String(obj.contact ?? ""),
    };
  } catch {
    return emptyAboutMeta();
  }
}

/** 序列化为 meta_json 字符串（空字段省略） */
export function buildAboutMetaJson(meta: AboutPostMeta): string {
  const payload: Record<string, string> = {};
  for (const [key, value] of Object.entries(meta)) {
    const trimmed = value.trim();
    if (trimmed) payload[key] = trimmed;
  }
  return JSON.stringify(payload);
}
