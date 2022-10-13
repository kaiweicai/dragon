use crate::service::dragon_origin_service::DragonService;
// use crate::service::cache_service::CacheService;
use crate::APPLICATION_CONTEXT;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_domain::dto::dragon_dto::DragonOriginDTO;
use chrono::{Datelike, Local};
use log::info;
use validator::Validate;

/// 用户登录接口。
pub async fn insert(Json(mut dragon): Json<DragonOriginDTO>) -> impl IntoResponse {
    let dragon_service = APPLICATION_CONTEXT.get::<DragonService>();
    if let Err(e) = dragon.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    // let content = dragon.content();
    base64::encode("".to_string());
    dragon.set_content(String::from_utf8(base64::decode(dragon.content()).unwrap()).unwrap());
    let now = Local::now();
    dragon.set_create_date(Some(format!(
        "{}-{:02}-{:02}",
        now.year(),
        now.month(),
        now.day()
    )));
    dragon_service.save(dragon).await;

    return RespVO::from(&"保存接龙数据成功".to_string()).resp_json();
}

//查询所有的列表
pub async fn list() -> impl IntoResponse {
    let dragon_service = APPLICATION_CONTEXT.get::<DragonService>();
    let dradon_list = dragon_service.dragon_list().await;
    return RespVO::from_result(&dradon_list).resp_json();
}

//根据id删除dragon数据
pub async fn del(Path(id): Path<String>) -> impl IntoResponse {
    info!("del dragon id {}", id);
    let dragon_service = APPLICATION_CONTEXT.get::<DragonService>();
    let dradon_list = dragon_service.del(id).await;
    return RespVO::from(&"delete dragon success!".to_string()).resp_json();
}

///根据id生成当日的dragondata数据
pub async fn gen_today_dragon_data(Path(id): Path<String>)-> impl IntoResponse{
    let today_dragon_data = DragonService::gen_today_dragon_data(id).await;
    return RespVO::from_result(&today_dragon_data).resp_json();
}


pub fn init_router() -> Router {
    Router::new()
        .route("/dragon/list", get(list))
        .route("/dragon", post(insert))
        .route("/dragon/:id", delete(del))
        .route("/dragon/todaydata/:id", get(gen_today_dragon_data))
}
