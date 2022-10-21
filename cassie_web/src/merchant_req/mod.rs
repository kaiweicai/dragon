use std::collections::HashMap;

use crate::APPLICATION_CONTEXT;
use cassie_domain::dto::merchant_dto::{MerchantResult, TokenDTO, LoginResponse, Plan, PlanData};
use log::info;

use serde_json;

static LOGIN_API_URL: &'static str = "http://mxhy9app.iaie83.com/api/account/Loginpwd";

static ORDER_URL: &'static str = "http://mxhy9app.iaie83.com/api/own/GetUserPlan";
//拿到token数据
pub async fn login(login_dto: HashMap<String, String>) {
    let login_api_url = LOGIN_API_URL.to_string();
    let client = reqwest::Client::new();
    let body = client
        .post(login_api_url)
        .json(&login_dto)
        .send()
        .await
        .unwrap()
        .text()
        .await;
    match body {
        Ok(s) => {
            let marchant_result = serde_json::from_str::<MerchantResult<LoginResponse>>(&s).unwrap();
            info!("token is {:?}", marchant_result.data.token);
            let token = TokenDTO(marchant_result.data.token);
            APPLICATION_CONTEXT.set::<TokenDTO>(token);
        }
        Err(e) => info!("link nacos  error: {:?}", e),
    }
}

pub async fn query_order() {
    let mut search_map = HashMap::new();
    search_map.insert("page_index", "1");
    search_map.insert("search_date", "10-21-2022");
    search_map.insert("search_orderby", "1");
    search_map.insert("sign", "0");
    let token = &APPLICATION_CONTEXT.get::<TokenDTO>().0;
    let client = reqwest::Client::new();

    let body = client
        .post(ORDER_URL.to_string())
        .header("TokenValue",token)
        .json(&search_map)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    match body {
        Ok(s) => {
            let merchant_result = serde_json::from_str::<MerchantResult<PlanData<Plan>>>(&s).unwrap();
            info!("merchant_result is {:?}", merchant_result);
            // let token = TokenDTO(token.data.token);
            // APPLICATION_CONTEXT.set::<TokenDTO>(token);
        }
        Err(e) => info!("link nacos  error: {:?}", e),
    }
}
