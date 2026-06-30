use rocket::Route;
use rocket::State;
use rocket::http::{ContentType, Status};
use rocket::http::uri::Host;
use rocket::request::Request;
use rocket::response::{self, Redirect, Responder, Response};
use rocket_dyn_templates::Template;

use crate::models::dict::{get_site_default_locale, get_site_locales};
use crate::models::load_public_banners_by_code;
use crate::models::locale::{encode_uri_path, is_supported_locale, locale_path, normalize_lang};
use crate::models::category::{
    CategoryMeta, category_detail_tera_template, category_list_archive_enabled,
    category_list_tera_template, category_to_view,
    resolve_category_id_from_public_key,
};
use crate::models::post::{PostMeta, post_to_view, resolve_post_id_from_public_key};
use crate::models::site_page_context;
use crate::rss::build_posts_rss_feed;
use crate::storage::StorageService;

/// 汇总页面路由
pub fn routes() -> Vec<Route> {
    routes![
        root_redirect,
        index_lang,
        index_lang_no_slash,
        category_archive_lang,
        post_detail_lang,
        posts_index_redirect,
        rss_lang,
        about_lang,
    ]
}

/// 根路径重定向到默认语言首页
#[get("/")]
pub async fn root_redirect(db: &State<toasty::Db>) -> Redirect {
    let mut db = db.inner().clone();
    let default = get_site_default_locale(&mut db).await;
    Redirect::to(locale_path(&default, ""))
}

/// 多语言首页（带尾斜杠）
#[get("/<lang>/")]
pub async fn index_lang(lang: &str, db: &State<toasty::Db>) -> Result<Template, Redirect> {
    render_public_page(db, lang, "").await
}

/// 多语言首页（无尾斜杠，重定向到带尾斜杠）
#[get("/<lang>", rank = 2)]
pub async fn index_lang_no_slash(lang: &str, db: &State<toasty::Db>) -> Result<Redirect, Redirect> {
    let mut db = db.inner().clone();
    let resolved = resolve_public_lang(&mut db, lang).await?;
    Ok(Redirect::to(locale_path(&resolved, "")))
}

/// 多语言文章详情页（支持数字 ID 或 SEO slug）
#[get("/<lang>/posts/<key>")]
pub async fn post_detail_lang(
    lang: &str,
    key: &str,
    db: &State<toasty::Db>,
) -> Result<Template, PostDetailError> {
    let mut db = db.inner().clone();
    let resolved = resolve_public_lang(&mut db, lang)
        .await
        .map_err(PostDetailError::LangRedirect)?;

    let post_id = match resolve_post_id_from_public_key(&mut db, &resolved, key).await {
        Ok(Some(id)) => id,
        Ok(None) => return Err(PostDetailError::NotFound),
        Err(e) if e.contains("对应多篇文章") => return Err(PostDetailError::Conflict),
        Err(_) => return Err(PostDetailError::NotFound),
    };

    let meta = PostMeta::get_by_id(&mut db, &post_id)
        .await
        .map_err(|_| PostDetailError::NotFound)?;
    let view = post_to_view(&mut db, &meta, Some(&resolved))
        .await
        .map_err(|_| PostDetailError::NotFound)?;

    // 数字 ID 访问且已配置 SEO 路径时，301 到 canonical URL
    if key.parse::<i64>().is_ok() && !view.route_path.is_empty() {
        let current = format!("/{resolved}/posts/{key}");
        if view.route_path != current {
            return Err(PostDetailError::CanonicalRedirect(Redirect::to(
                encode_uri_path(&view.route_path),
            )));
        }
    }

    let current_path = format!("/{resolved}/posts/{key}");
    let tera_name = CategoryMeta::get_by_id(&mut db, &meta.category_id)
        .await
        .map(|cat| category_detail_tera_template(&cat.detail_template))
        .unwrap_or("post-detail");

    let mut ctx = site_page_context(&mut db, "post-detail", &current_path, Some(&resolved)).await;
    if let Some(obj) = ctx.as_object_mut() {
        obj.insert("post_id".to_string(), serde_json::json!(post_id));
        obj.insert("title".to_string(), serde_json::json!(view.title));
        if !view.description.is_empty() {
            obj.insert("post_description".to_string(), serde_json::json!(view.description));
        }
        obj.insert("category_name".to_string(), serde_json::json!(view.category_name));
        obj.insert(
            "category_route_path".to_string(),
            serde_json::json!(view.category_route_path),
        );
    }
    Ok(Template::render(tera_name, ctx))
}

/// 文章详情页错误响应（语言重定向 / SEO 重定向 / 404）
pub(crate) enum PostDetailError {
    LangRedirect(Redirect),
    CanonicalRedirect(Redirect),
    NotFound,
    Conflict,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for PostDetailError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Self::LangRedirect(r) | Self::CanonicalRedirect(r) => r.respond_to(req),
            Self::NotFound => Status::NotFound.respond_to(req),
            Self::Conflict => Status::Conflict.respond_to(req),
        }
    }
}

/// 多语言分类归档页（支持数字 ID 或 SEO slug）
#[get("/<lang>/categories/<key>")]
pub async fn category_archive_lang(
    lang: &str,
    key: &str,
    db: &State<toasty::Db>,
) -> Result<Template, CategoryArchiveError> {
    let mut db = db.inner().clone();
    let resolved = resolve_public_lang(&mut db, lang)
        .await
        .map_err(CategoryArchiveError::LangRedirect)?;

    let category_id = match resolve_category_id_from_public_key(&mut db, &resolved, key).await {
        Ok(Some(id)) => id,
        Ok(None) => return Err(CategoryArchiveError::NotFound),
        Err(e) if e.contains("对应多个分类") => return Err(CategoryArchiveError::Conflict),
        Err(_) => return Err(CategoryArchiveError::NotFound),
    };

    let meta = CategoryMeta::get_by_id(&mut db, &category_id)
        .await
        .map_err(|_| CategoryArchiveError::NotFound)?;
    if !category_list_archive_enabled(&meta.list_template) {
        return Err(CategoryArchiveError::NotFound);
    }

    let default = get_site_default_locale(&mut db).await;
    if key.parse::<i64>().is_ok()
        && let Ok(view) = category_to_view(&mut db, &meta, &resolved, &default).await
        && !view.route_path.is_empty()
    {
        let current = format!("/{resolved}/categories/{key}");
        if view.route_path != current {
            return Err(CategoryArchiveError::CanonicalRedirect(Redirect::to(
                encode_uri_path(&view.route_path),
            )));
        }
    }

    let tera_name = category_list_tera_template(&meta.list_template);

    let current_path = format!("/{resolved}/categories/{key}");
    let mut ctx = site_page_context(
        &mut db,
        "category-archive",
        &current_path,
        Some(&resolved),
    )
    .await;
    if let Some(obj) = ctx.as_object_mut() {
        obj.insert("category_id".to_string(), serde_json::json!(category_id));
    }
    Ok(Template::render(tera_name, ctx))
}

/// 分类归档页错误响应
pub(crate) enum CategoryArchiveError {
    LangRedirect(Redirect),
    CanonicalRedirect(Redirect),
    NotFound,
    Conflict,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for CategoryArchiveError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Self::LangRedirect(r) | Self::CanonicalRedirect(r) => r.respond_to(req),
            Self::NotFound => Status::NotFound.respond_to(req),
            Self::Conflict => Status::Conflict.respond_to(req),
        }
    }
}

/// 原全站文章列表路径，重定向至首页
#[get("/<lang>/posts")]
pub async fn posts_index_redirect(lang: &str, db: &State<toasty::Db>) -> Redirect {
    let mut db = db.inner().clone();
    match resolve_public_lang(&mut db, lang).await {
        Ok(resolved) => Redirect::to(locale_path(&resolved, "")),
        Err(r) => r,
    }
}

/// 多语言 RSS 订阅
#[get("/<lang>/rss")]
pub async fn rss_lang(
    lang: &str,
    db: &State<toasty::Db>,
    storage: &State<StorageService>,
    host: &Host<'_>,
) -> Result<RssResponse, Redirect> {
    let mut db = db.inner().clone();
    let resolved = resolve_public_lang(&mut db, lang).await?;
    let origin = request_site_origin(host);
    let feed = build_posts_rss_feed(&mut db, storage.inner(), &resolved, &origin)
        .await
        .map_err(|_| Redirect::to(locale_path(&resolved, "")))?;
    Ok(RssResponse(Template::render("rss", feed)))
}

/// 多语言关于页
#[get("/<lang>/about")]
pub async fn about_lang(lang: &str, db: &State<toasty::Db>) -> Result<Template, Redirect> {
    render_public_page(db, lang, "about").await
}

/// 校验并解析公开页语言，不支持时重定向到默认语言
async fn resolve_public_lang(db: &mut toasty::Db, lang: &str) -> Result<String, Redirect> {
    let normalized = normalize_lang(lang);
    let default = get_site_default_locale(db).await;
    let supported = get_site_locales(db).await;

    if is_supported_locale(&normalized, &supported) {
        Ok(normalized)
    } else {
        Err(Redirect::to(locale_path(&default, "")))
    }
}

/// 渲染多语言公开页
async fn render_public_page(
    db: &State<toasty::Db>,
    lang: &str,
    page_slug: &str,
) -> Result<Template, Redirect> {
    let mut db = db.inner().clone();
    let resolved = resolve_public_lang(&mut db, lang).await?;
    let current_path = locale_path(&resolved, page_slug);
    let mut ctx = site_page_context(&mut db, page_slug, &current_path, Some(&resolved)).await;

    if page_slug.is_empty()
        && let Some(obj) = ctx.as_object_mut()
    {
        let default_lang = get_site_default_locale(&mut db).await;
        let banners = load_public_banners_by_code(&mut db, "home_banner", &resolved, &default_lang)
            .await
            .unwrap_or_default();
        obj.insert(
            "banners".to_string(),
            serde_json::to_value(&banners).unwrap_or_else(|_| serde_json::json!([])),
        );
        obj.insert(
            "banner_heroes_json".to_string(),
            serde_json::Value::String(
                serde_json::to_string(&banners).unwrap_or_else(|_| "[]".to_string()),
            ),
        );
    }

    Ok(match page_slug {
        "about" => Template::render("about", ctx),
        _ => Template::render("index", ctx),
    })
}

/// RSS 响应（application/rss+xml）
pub(crate) struct RssResponse(Template);

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for RssResponse {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(ContentType::new("application", "rss+xml"))
            .merge(self.0.respond_to(req)?)
            .ok()
    }
}

fn request_site_origin(host: &Host<'_>) -> String {
    format!("http://{host}")
}
