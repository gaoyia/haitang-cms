/** 公开页首选 URL：有 SEO 路径时用 SEO，否则用 ID 路径 */
export function primaryPublicUrl(idUrl: string, seoUrl?: string | null): string {
  const seo = seoUrl?.trim();
  return seo || idUrl;
}
