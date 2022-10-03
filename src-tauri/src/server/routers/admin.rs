



pub fn noneed_auth_routers() -> Router {
    Router::new()
        //-------------------------------------登录服务-------------------------------------------------------
        .route("/captcha/:uuid", get(sys_auth_resource::captcha_base64))
        .route("/captcha/png/:uuid", get(sys_auth_resource::captcha_png))
        .route("/login", post(sys_auth_resource::login))
}