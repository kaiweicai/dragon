use crate::service::dragon_origin_service::DragonService;
// use crate::service::cache_service::CacheService;
use crate::service::sys_auth_service::SysAuthService;
use crate::APPLICATION_CONTEXT;

use axum::body::Body;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::Json;
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_domain::dto::sign_in::SignInDTO;
use cassie_domain::dto::sys_user_dto::DragonOriginDTO;
use validator::Validate;

/// 用户登录接口。
pub async fn insert(Json(dragon): Json<DragonOriginDTO>) -> impl IntoResponse {
    let dragon_service = APPLICATION_CONTEXT.get::<DragonService>();
    if let Err(e) = dragon.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    let vo = dragon_service.save(&dragon).await;

    return RespVO::from(&"保存接龙数据成功".to_string()).into();
}

pub async fn testd_ragon(Path(dragon): Path<String>) -> impl IntoResponse {
    return RespVO::from(&dragon).resp_json();
}

//本地应用，暂时不需要验证码。
pub async fn captcha_base64(Path(uuid): Path<String>) -> impl IntoResponse {
    // let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    // if uuid.is_empty() {
    //     return RespVO::<()>::from_error(&Error::from("uuid不能为空!")).resp_json();
    // }
    // let (captcha_str, png) = {
    //     let mut captcha = Captcha::new();
    //     captcha
    //         .add_chars(4)
    //         .apply_filter(Noise::new(0.1))
    //         .apply_filter(Wave::new(1.0, 10.0).horizontal())
    //         // .apply_filter(Wave::new(2.0, 20.0).vertical())
    //         .view(160, 60)
    //         .apply_filter(Dots::new(4));

    //     let png = captcha.as_base64();
    //     (captcha.chars_as_string().to_lowercase(), png)
    // };

    // let res = cache_service
    //     .set_string_ex(
    //         &format!("_captch:uuid_{}", uuid.clone()),
    //         captcha_str.as_str(),
    //         Some(std::time::Duration::from_secs(60 * 5)),
    //     )
    //     .await;
    // println!("{:?}", res);
    // return RespVO::from(&png.unwrap()).resp_json();
    return RespVO::from(&"".to_string()).resp_json();
}
pub async fn captcha_png(Path(uuid): Path<String>) -> Response<Body> {
    // let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    if uuid.is_empty() {
        return RespVO::<()>::from_error(&Error::from("uuid不能为空!")).resp_json();
    }
    let (captcha_str, png) = {
        let mut captcha = Captcha::new();
        captcha
            .add_chars(4)
            .apply_filter(Noise::new(0.1))
            .apply_filter(Wave::new(1.0, 10.0).horizontal())
            // .apply_filter(Wave::new(2.0, 20.0).vertical())
            .view(160, 60)
            .apply_filter(Dots::new(4));

        let png = captcha.as_png().unwrap();
        (captcha.chars_as_string().to_lowercase(), png)
    };

    // let res = cache_service
    //     .set_string_ex(
    //         &format!("_captch:uuid_{}", uuid.clone()),
    //         captcha_str.as_str(),
    //         Some(std::time::Duration::from_secs(60 * 5)),
    //     )
    //     .await;
    // println!("{:?}", res);
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Cache-Control", "no-cache")
        .header("Content-Type", "image/png")
        .body(Body::from(png))
        .unwrap()
}
