use std::collections::HashMap;

use crate::merchant_req;


// use crate::service::cache_service::CacheService;


use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};

use cassie_common::RespVO;


use cassie_domain::dto::merchant_dto::MerchantLoginDTO;





/// 用户登录接口。
pub async fn merchant_login(Json(merchant_login_dto): Json<MerchantLoginDTO>) -> impl IntoResponse {
    let mut merchant_login_map = HashMap::new();
    merchant_login_map.insert("mobile".to_string(), merchant_login_dto.mobile);
    merchant_login_map.insert("pwd".to_string(), merchant_login_dto.pwd);
    merchant_req::login(merchant_login_map).await;
    return RespVO::from(&"保存接龙数据成功".to_string()).resp_json();
}

pub async fn query_order() -> impl IntoResponse {
    merchant_req::query_order().await;
    return RespVO::from(&"保存接龙数据成功".to_string()).resp_json();
}

pub fn init_router() -> Router {
    Router::new()
    .route("/merchant/login", post(merchant_login))
    .route("/merchant/query_order", get(query_order))
}
