use rocket::State;
use rocket::serde::json::Json;

use crate::guards::AdminAuth;
use crate::models::{
    ApiResponse, CreateFriendLink, FriendLink, FriendLinkView, PageResult, UpdateFriendLink,
    friend_links_to_views, paginate_vec, validate_friend_link_image_url, validate_friend_link_url,
};
use crate::routes::page::PageQuery;

#[get("/api/admin/friend-links?<page..>")]
pub async fn list(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    page: PageQuery,
) -> Json<ApiResponse<PageResult<FriendLinkView>>> {
    let mut db = db.inner().clone();

    match FriendLink::all().exec(&mut db).await {
        Ok(links) => {
            let views = friend_links_to_views(links);
            let (p, ps) = page.resolve();
            Json(ApiResponse::success(paginate_vec(views, p, ps)))
        }
        Err(e) => Json(ApiResponse::error(500, format!("查询失败: {e}"))),
    }
}

#[get("/api/admin/friend-links/<id>")]
pub async fn get(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
) -> Json<ApiResponse<FriendLinkView>> {
    let mut db = db.inner().clone();

    match FriendLink::get_by_id(&mut db, &id).await {
        Ok(link) => Json(ApiResponse::success(link.to_view())),
        Err(_) => Json(ApiResponse::error(404, "友链不存在")),
    }
}

#[post("/api/admin/friend-links", data = "<input>")]
pub async fn create(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    input: Json<CreateFriendLink>,
) -> Json<ApiResponse<FriendLinkView>> {
    let mut db = db.inner().clone();

    let title = input.title.trim();
    if title.is_empty() {
        return Json(ApiResponse::error(400, "名称不能为空"));
    }

    let url = match validate_friend_link_url(&input.url) {
        Ok(u) => u,
        Err(msg) => return Json(ApiResponse::error(400, msg)),
    };

    let image_url = match input
        .image_url
        .as_deref()
        .map(validate_friend_link_image_url)
        .transpose()
    {
        Ok(Some(u)) => u,
        Ok(None) => return Json(ApiResponse::error(400, "友链图片不能为空")),
        Err(msg) => return Json(ApiResponse::error(400, msg)),
    };

    let sort = input.sort.unwrap_or(0);
    let status = input.status.unwrap_or(1);

    match FriendLink::create()
        .title(title)
        .url(&url)
        .image_url(&image_url)
        .sort(sort)
        .status(status)
        .exec(&mut db)
        .await
    {
        Ok(link) => Json(ApiResponse::success(link.to_view())),
        Err(e) => Json(ApiResponse::error(500, format!("创建失败: {e}"))),
    }
}

#[put("/api/admin/friend-links/<id>", data = "<input>")]
pub async fn update(
    _auth: AdminAuth,
    db: &State<toasty::Db>,
    id: i64,
    input: Json<UpdateFriendLink>,
) -> Json<ApiResponse<FriendLinkView>> {
    let mut db = db.inner().clone();

    let mut link = match FriendLink::get_by_id(&mut db, &id).await {
        Ok(l) => l,
        Err(_) => return Json(ApiResponse::error(404, "友链不存在")),
    };

    let mut builder = link.update();

    if let Some(ref title) = input.title {
        let title = title.trim();
        if title.is_empty() {
            return Json(ApiResponse::error(400, "名称不能为空"));
        }
        builder = builder.title(title);
    }

    if let Some(ref url) = input.url {
        match validate_friend_link_url(url) {
            Ok(u) => builder = builder.url(u.as_str()),
            Err(msg) => return Json(ApiResponse::error(400, msg)),
        }
    }

    if let Some(ref image_url) = input.image_url {
        match validate_friend_link_image_url(image_url) {
            Ok(u) => builder = builder.image_url(u.as_str()),
            Err(msg) => return Json(ApiResponse::error(400, msg)),
        }
    }

    if let Some(sort) = input.sort {
        builder = builder.sort(sort);
    }
    if let Some(status) = input.status {
        builder = builder.status(status);
    }

    match builder.exec(&mut db).await {
        Ok(_) => match FriendLink::get_by_id(&mut db, &id).await {
            Ok(updated) => Json(ApiResponse::success(updated.to_view())),
            Err(e) => Json(ApiResponse::error(500, format!("更新成功但查询失败: {e}"))),
        },
        Err(e) => Json(ApiResponse::error(500, format!("更新失败: {e}"))),
    }
}

#[delete("/api/admin/friend-links/<id>")]
pub async fn delete(_auth: AdminAuth, db: &State<toasty::Db>, id: i64) -> Json<ApiResponse<()>> {
    let mut db = db.inner().clone();

    match FriendLink::get_by_id(&mut db, &id).await {
        Ok(link) => match link.delete().exec(&mut db).await {
            Ok(_) => Json(ApiResponse {
                code: 0,
                message: "删除成功".to_string(),
                data: None,
            }),
            Err(e) => Json(ApiResponse::error(500, format!("删除失败: {e}"))),
        },
        Err(_) => Json(ApiResponse::error(404, "友链不存在")),
    }
}
