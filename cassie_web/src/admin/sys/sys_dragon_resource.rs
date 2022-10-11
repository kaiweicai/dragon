use crate::service::dragon_origin_service::DragonService;
// use crate::service::cache_service::CacheService;
use crate::APPLICATION_CONTEXT;

use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_domain::dto::dragon_dto::DragonOriginDTO;
use validator::Validate;

/// 用户登录接口。
pub async fn insert(Json(dragon): Json<DragonOriginDTO>) -> impl IntoResponse {
    let dragon_service = APPLICATION_CONTEXT.get::<DragonService>();
    if let Err(e) = dragon.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    // let content = dragon.content();
    // dragon.set_content(base64::decode(dragon.content()));
    dragon_service.save(dragon).await;

    return RespVO::from(&"保存接龙数据成功".to_string()).resp_json();
}

//查询所有的列表
pub async fn list() -> impl IntoResponse {
    let dragon_service = APPLICATION_CONTEXT.get::<DragonService>();
    let dradon_list = dragon_service.dragon_list().await;
    return RespVO::from_result(&dradon_list).resp_json();
}

pub fn init_router() -> Router {
    Router::new()
        .route("/dragon/list", get(list))
        .route("/dragon/insert", post(insert))
}
