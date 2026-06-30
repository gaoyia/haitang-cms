//! 上传用途与 MIME 类型限制

/// 资源上传用途（写入 asset.purpose）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetPurpose {
    /// 封面图：仅图片
    Cover,
    /// 正文插图：仅图片
    Content,
    /// 轮播图：仅图片
    Banner,
    /// 友链图：仅图片
    FriendLink,
    /// 附件：文档、压缩包、图片、视频等
    Attachment,
}

impl AssetPurpose {
    pub fn parse(raw: &str) -> Result<Self, String> {
        match raw.trim().to_lowercase().as_str() {
            "cover" => Ok(Self::Cover),
            "content" => Ok(Self::Content),
            "banner" => Ok(Self::Banner),
            "friend_link" => Ok(Self::FriendLink),
            "attachment" => Ok(Self::Attachment),
            _ => Err("purpose 须为 cover、content、banner、friend_link 或 attachment".to_string()),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cover => "cover",
            Self::Content => "content",
            Self::Banner => "banner",
            Self::FriendLink => "friend_link",
            Self::Attachment => "attachment",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Cover => "封面图",
            Self::Content => "正文插图",
            Self::Banner => "轮播图",
            Self::FriendLink => "友链",
            Self::Attachment => "附件",
        }
    }

    /// 校验 MIME；扩展名作为兜底
    pub fn validate_file(self, mime: &str, filename: &str) -> Result<(), String> {
        let mime = mime.trim().to_lowercase();
        let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();

        match self {
            Self::Cover | Self::Content | Self::Banner | Self::FriendLink => {
                if mime.starts_with("image/")
                    || matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif")
                {
                    Ok(())
                } else {
                    Err(format!(
                        "{}仅允许上传图片（jpg、png、webp、gif）",
                        self.label()
                    ))
                }
            }
            Self::Attachment => {
                if mime.starts_with("image/")
                    || mime.starts_with("video/")
                    || mime.starts_with("application/")
                    || mime.starts_with("text/")
                    || matches!(
                        ext.as_str(),
                        "zip"
                            | "rar"
                            | "7z"
                            | "pdf"
                            | "doc"
                            | "docx"
                            | "xls"
                            | "xlsx"
                            | "ppt"
                            | "pptx"
                            | "txt"
                            | "md"
                            | "mp4"
                            | "webm"
                            | "mov"
                            | "mkv"
                            | "avi"
                            | "jpg"
                            | "jpeg"
                            | "png"
                            | "webp"
                            | "gif"
                    )
                {
                    Ok(())
                } else {
                    Err(
                        "附件允许图片、视频、zip、rar、7z、pdf、Office 文档及纯文本等格式".to_string(),
                    )
                }
            }
        }
    }
}

/// 轮播图与资源的关联角色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerAssetRole {
    Image,
}

impl BannerAssetRole {
    pub fn parse(raw: &str) -> Result<Self, String> {
        match raw.trim().to_lowercase().as_str() {
            "image" => Ok(Self::Image),
            _ => Err("role 须为 image".to_string()),
        }
    }

    pub fn as_str(self) -> &'static str {
        "image"
    }

    pub fn accepts_purpose(self, purpose: &str) -> bool {
        purpose == AssetPurpose::Banner.as_str()
    }
}

/// 文章与资源的关联角色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostAssetRole {
    Cover,
    Attachment,
}

impl PostAssetRole {
    pub fn parse(raw: &str) -> Result<Self, String> {
        match raw.trim().to_lowercase().as_str() {
            "cover" => Ok(Self::Cover),
            "attachment" => Ok(Self::Attachment),
            _ => Err("role 须为 cover 或 attachment".to_string()),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cover => "cover",
            Self::Attachment => "attachment",
        }
    }

    /// 关联时校验资源 purpose 是否匹配
    pub fn accepts_purpose(self, purpose: &str) -> bool {
        match self {
            Self::Cover => purpose == AssetPurpose::Cover.as_str(),
            Self::Attachment => purpose == AssetPurpose::Attachment.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_accepts_jpeg() {
        assert!(
            AssetPurpose::Banner
                .validate_file("image/jpeg", "a.jpg")
                .is_ok()
        );
    }

    #[test]
    fn friend_link_accepts_jpeg() {
        assert!(
            AssetPurpose::FriendLink
                .validate_file("image/jpeg", "a.jpg")
                .is_ok()
        );
    }

    #[test]
    fn cover_rejects_zip() {
        assert!(
            AssetPurpose::Cover
                .validate_file("application/zip", "a.zip")
                .is_err()
        );
    }

    #[test]
    fn attachment_accepts_png() {
        assert!(
            AssetPurpose::Attachment
                .validate_file("image/png", "a.png")
                .is_ok()
        );
    }

    #[test]
    fn attachment_accepts_mp4() {
        assert!(
            AssetPurpose::Attachment
                .validate_file("video/mp4", "clip.mp4")
                .is_ok()
        );
    }

    #[test]
    fn attachment_rejects_unknown() {
        assert!(
            AssetPurpose::Attachment
                .validate_file("chemical/x-unknown", "a.xyz")
                .is_err()
        );
    }

    #[test]
    fn cover_accepts_jpeg() {
        assert!(
            AssetPurpose::Cover
                .validate_file("image/jpeg", "a.jpg")
                .is_ok()
        );
    }
}
