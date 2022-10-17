use axum::{response::IntoResponse, routing::get, Router};




//获取用户信息
pub async fn get_user_info() -> impl IntoResponse {
    // let request_model = get_local().unwrap();
    // let user_service = APPLICATION_CONTEXT.get::<UserService>();
    // let user = user_service.get(request_model.uid().to_string()).await;
    // return RespVO::from_result(&user).resp_json();
    todo!()
}

pub fn init_router() -> Router {
    Router::new().route("/user", get(get_user_info))
}
