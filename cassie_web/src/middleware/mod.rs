use cassie_domain::vo::jwt::JWTToken;
pub mod auth_admin;
pub mod auth_api;
pub mod clean_context;
pub mod event;

use crate::APPLICATION_CONTEXT;
use cassie_common::error::Error;
use cassie_config::config::WebApplicationConfig;
use cassie_domain::request::RequestModel;
use std::sync::{Arc, Mutex};
use thread_local::ThreadLocal;

lazy_static! {
    static ref REQUEST_CONTEXT: Arc<Mutex<ThreadLocal<RequestModel>>> =
        Arc::new(Mutex::new(ThreadLocal::new()));
}
/**
 *method:checked_token
 *desc:校验token是否有效，未过期
 *author:String
 *email:348040933@qq.com
 */
pub fn checked_token(token: &str) -> Result<JWTToken, Error> {
    //check token alive
    let cassie_config = APPLICATION_CONTEXT.get::<WebApplicationConfig>();
    let token = JWTToken::verify(cassie_config.jwt_secret(), token);
    token
}

pub fn get_local() -> Option<RequestModel> {
    let req = REQUEST_CONTEXT.clone();
    let request_model = req.lock().unwrap();
    match request_model.get() {
        None => None,
        Some(e) => Some(e.clone()),
    }
}

pub fn set_local(data: JWTToken, path: String) {
    let req = REQUEST_CONTEXT.clone();
    let mut request_model = req.lock().unwrap();
    //先清除再创建
    request_model.clear();
    request_model.get_or(|| RequestModel::new(data, path));
}

pub fn set_local_for_model(data: RequestModel) {
    let req = REQUEST_CONTEXT.clone();
    let mut request_model = req.lock().unwrap();
    //先清除再创建
    request_model.clear();
    request_model.get_or(|| data);
}

pub fn clear_local() {
    let req = REQUEST_CONTEXT.clone();
    let mut request_model = req.lock().unwrap();
    request_model.clear();
}
