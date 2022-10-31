use std::collections::HashMap;

use crate::merchant_req::merchant_service;


// use crate::service::cache_service::CacheService;


use axum::extract::Path;
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
    merchant_service::login(merchant_login_map).await;
    return RespVO::from(&"保存接龙数据成功".to_string()).resp_json();
}

// 10-21-2022
pub async fn query_order(Path(search_date): Path<String>) -> impl IntoResponse {
    let result = merchant_service::query_system_order(search_date).await;
    return RespVO::from_result(&result).resp_json();
}

pub fn init_router() -> Router {
    Router::new()
    .route("/merchant/login", post(merchant_login))
    .route("/merchant/query_order/:search_date", get(query_order))
}
