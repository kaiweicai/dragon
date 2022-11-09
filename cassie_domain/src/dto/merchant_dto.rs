use log::info;
use serde::{Deserialize, Serialize, de::Error};

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
    pub code: u64,
    pub message: String,
    pub data: T,
}

pub struct TokenDTO(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlanData<T> {
    pub plan_list: Vec<T>,
    pub total_data: u64,
    pub page_index: u64,
    pub page_size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default,Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Plan {
    pub planid: Option<u64>,
    pub plan_code: Option<String>,
    pub gsc_id: u64,
    pub gsc_name: String,
    pub gsc_img: String,
    #[serde(deserialize_with = "de_u64_from_str")]
    pub plan_price: i64,
    pub trade_price: Option<u64>,
    pub seller_id: u64,
    pub seller_name: String,
    pub seller_mobile: String,
    pub buyer_id: u64,
    pub buyer_name: String,
    pub buyer_mobile: String,
    pub buy_price: String,
    pub create_time: String,
    pub update_time: String,
    pub belong_time: String,
    pub plan_status: u64,
    pub plan_statusstr: String,
    pub list_log: Option<String>,
    pub origin_plan_code: Option<String>, //拆单后原始的订单plan_code
}

fn de_u64_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s:String = String::deserialize(deserializer)?;
    s.parse::<i64>().map_err(Error::custom)
}

impl Plan {
    //把一份订单拆分成两个订单。
    pub fn split(&self) -> Vec<Self> {
        info!("被拆的订单号为:{:?}",self.planid);
        let mut p1 = self.clone();
        p1.origin_plan_code = self.plan_code.clone();
        p1.plan_price = self.plan_price / 2;
        // p1.plan_code = None;
        let mut p2 = self.clone();
        p2.origin_plan_code = self.plan_code.clone();
        p2.plan_price = self.plan_price / 2;
        // p2.plan_code = None;
        vec![p1, p2]
    }
}

#[cfg(test)]
mod tests {
    use super::Plan;

    #[test]
    fn test_split() {
        let mut p = Plan::default();
        let split_order = p.split();
        println!("split_order is :{:#?}", split_order);
    }
}
