use serde::{Deserialize, Serialize};

use validator_derive::Validate;

#[derive(Clone, Debug, Serialize, Validate, Deserialize, Getters, Setters, Default)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub", set = "pub")]
pub struct MerchantLoginDTO {
    pub mobile: String,
    pub pwd: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LoginResponse {
    pub token: String,
    pub vicode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MerchantResult<T> {
    pub code:u64,
    pub message:String,
    pub data: T,
}

pub struct TokenDTO(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlanData<T> {
    pub plan_list: Vec<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Plan{
    pub planid: Option<u64>,
    pub plan_code: String,
    pub gsc_id:u64,
    pub gsc_name:String,
    pub gsc_img:String,
    pub plan_price:String,
    pub trade_price:Option<u64>,
    pub seller_id:u64,
    pub seller_name:String,
    pub seller_mobile:String,
    pub buyer_id:u64,
    pub buyer_name:String,
    pub buyer_mobile:String,
    pub buy_price:String,
    pub create_time:String,
    pub update_time:String,
    pub belong_time:String,
    pub plan_status:u64,
    pub plan_statusstr:String,
    pub list_log:Option<String>,
}