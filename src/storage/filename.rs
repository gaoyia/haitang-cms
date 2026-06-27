//! 上传文件名与 MIME 解析（补全缺失扩展名）

use std::path::Path;

/// 上传文件元信息（MIME 与校验用展示名）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadFileMeta {
    pub mime_type: String,
    /// 仅用于类型校验的虚拟文件名（如 `file.png`）
    pub validate_name: String,
}

/// 根据 Content-Type、文件内容与原始文件名，解析 MIME 与校验用文件名
pub fn resolve_upload_file_meta(raw_name: &str, content_type: &str, data: &[u8]) -> UploadFileMeta {
    let mime_type = resolve_mime(content_type, data, raw_name);
    let validate_name = extension_for(&mime_type, data)
        .map(|ext| format!("file.{ext}"))
        .unwrap_or_else(|| "file".to_string());
    UploadFileMeta {
        mime_type,
        validate_name,
    }
}

/// 解析 MIME：优先使用浏览器声明；声明含糊或无扩展名时用魔数嗅探
pub fn resolve_mime(content_type: &str, data: &[u8], filename: &str) -> String {
    let declared = parse_mime_type(content_type);
    let Some(kind) = infer::get(data) else {
        return declared;
    };
    let sniffed = kind.mime_type().to_string();
    if is_generic_mime(&declared) || !has_meaningful_extension(filename) {
        return sniffed;
    }
    declared
}

/// 若文件名缺少扩展名，则根据 MIME 与文件内容补全
pub fn ensure_filename_extension(filename: &str, mime: &str, data: &[u8]) -> String {
    let base = sanitize_upload_name(filename);
    if has_meaningful_extension(&base) {
        return base;
    }
    let stem = base
        .rsplit_once('.')
        .map(|(s, _)| s)
        .unwrap_or(base.as_str());
    match extension_for(mime, data) {
        Some(ext) => format!("{stem}.{ext}"),
        None => base,
    }
}

/// 文件名是否已有可用扩展名
pub fn has_meaningful_extension(filename: &str) -> bool {
    let Some((stem, ext)) = filename.rsplit_once('.') else {
        return false;
    };
    if stem.is_empty() || ext.is_empty() {
        return false;
    }
    ext.len() <= 10 && ext.chars().all(|c| c.is_ascii_alphanumeric())
}

/// 由 MIME 与文件内容推断扩展名（不含点）
pub fn extension_for(mime: &str, data: &[u8]) -> Option<String> {
    infer::get(data)
        .filter(|k| k.mime_type() == mime)
        .map(|k| k.extension().to_string())
        .filter(|ext| !ext.is_empty())
        .or_else(|| mime_to_ext(mime))
}

/// 清洗用户上传原文件名（仅入库展示，不用于磁盘路径；保留中文等 Unicode）
pub fn sanitize_upload_name(name: &str) -> String {
    let base = Path::new(name)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let mut out = String::new();
    for ch in base.chars() {
        if ch.is_control() || matches!(ch, '/' | '\\') {
            continue;
        }
        out.push(ch);
    }
    let out = out.trim().trim_matches('.').to_string();
    if out.is_empty() {
        return "file".to_string();
    }
    const MAX_LEN: usize = 255;
    if out.chars().count() > MAX_LEN {
        out.chars().take(MAX_LEN).collect()
    } else {
        out
    }
}

fn parse_mime_type(content_type: &str) -> String {
    let base = content_type
        .split(';')
        .next()
        .unwrap_or(content_type)
        .trim();
    if base.is_empty() {
        "application/octet-stream".to_string()
    } else {
        base.to_ascii_lowercase()
    }
}

fn is_generic_mime(mime: &str) -> bool {
    matches!(
        mime,
        "application/octet-stream" | "binary/octet-stream" | "application/unknown"
    )
}

/// 常见 MIME → 扩展名（与 policy 允许的类型对齐）
fn mime_to_ext(mime: &str) -> Option<String> {
    Some(
        match mime {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/webp" => "webp",
            "image/gif" => "gif",
            "video/mp4" => "mp4",
            "video/webm" => "webm",
            "video/quicktime" => "mov",
            "video/x-msvideo" => "avi",
            "video/x-matroska" => "mkv",
            "application/pdf" => "pdf",
            "application/zip" => "zip",
            "application/x-rar-compressed" | "application/vnd.rar" => "rar",
            "application/x-7z-compressed" => "7z",
            "application/msword" => "doc",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => "docx",
            "application/vnd.ms-excel" => "xls",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => "xlsx",
            "application/vnd.ms-powerpoint" => "ppt",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation" => "pptx",
            "text/plain" => "txt",
            "text/markdown" => "md",
            _ => return None,
        }
        .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserve_chinese_upload_name() {
        let data = [0x25, 0x50, 0x44, 0x46, 0x2D];
        let name = ensure_filename_extension("季度报告.pdf", "application/pdf", &data);
        assert_eq!(name, "季度报告.pdf");
    }

    #[test]
    fn strip_path_from_upload_name() {
        assert_eq!(
            sanitize_upload_name("../../docs/说明文档.md"),
            "说明文档.md"
        );
    }

    #[test]
    fn append_png_when_unnamed() {
        let data = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
        ];
        let name = ensure_filename_extension("unnamed", "image/png", &data);
        assert_eq!(name, "unnamed.png");
    }

    #[test]
    fn keep_existing_jpg() {
        let data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        let name = ensure_filename_extension("photo.jpg", "image/jpeg", &data);
        assert_eq!(name, "photo.jpg");
    }

    #[test]
    fn append_jpg_when_no_ext() {
        let data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        let name = ensure_filename_extension("banner", "image/jpeg", &data);
        assert_eq!(name, "banner.jpg");
    }

    #[test]
    fn sniff_png_from_bytes() {
        let data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00];
        let mime = resolve_mime("application/octet-stream", &data, "unnamed");
        assert_eq!(mime, "image/png");
    }

    #[test]
    fn resolve_meta_for_clipboard_paste() {
        let data = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
        ];
        let meta = resolve_upload_file_meta("unnamed", "application/octet-stream", &data);
        assert_eq!(meta.mime_type, "image/png");
        assert_eq!(meta.validate_name, "file.png");
    }

    #[test]
    fn declared_mime_when_name_has_ext() {
        let data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        let meta = resolve_upload_file_meta("a.jpg", "image/jpeg", &data);
        assert_eq!(meta.mime_type, "image/jpeg");
        assert_eq!(meta.validate_name, "file.jpg");
    }

    #[test]
    fn object_key_uses_date_user_uuid_and_ext() {
        use crate::storage::AssetStorage;
        use crate::storage::local::LocalStorage;

        let key = LocalStorage::new(
            std::path::PathBuf::from("static/uploads"),
            "/static/uploads".to_string(),
        )
        .new_object_key(42, Some("png"));
        let parts: Vec<&str> = key.split('/').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].len(), 10);
        assert_eq!(parts[0].as_bytes()[4], b'-');
        assert_eq!(parts[1], "42");
        assert!(parts[2].ends_with(".png"));
        assert_eq!(parts[2].len(), 32 + 1 + 3);
    }
}
