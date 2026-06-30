use rocket::State;
use rocket::serde::json::Json;

use crate::models::{ApiResponse, PublicFriendLink, get_public_friend_links};

/// 获取启用的友情链接列表（公开）
#[get("/api/friend-links")]
pub async fn list(db: &State<toasty::Db>) -> Json<ApiResponse<Vec<PublicFriendLink>>> {
    let mut db = db.inner().clone();
    Json(ApiResponse::success(get_public_friend_links(&mut db).await))
}
