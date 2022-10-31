use std::collections::HashMap;

use crate::{service::dragon_data_service, APPLICATION_CONTEXT};
use cassie_common::{
    error::{Error, Result},
    utils::date_utils,
};
use cassie_domain::dto::{
    dragon_data_dto::DragonDataDTO,
    merchant_dto::{LoginResponse, MerchantResult, Plan, PlanData, TokenDTO},
};
use chrono::Local;
use log::info;

use serde_json;

static LOGIN_API_URL: &'static str = "http://mxbhnty13app.miaoxiu993.com/api/account/Loginpwd";

static ORDER_URL: &'static str = "http://mxbhnty13app.miaoxiu993.com/api/own/GetUserPlan";
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
            let marchant_result =
                serde_json::from_str::<MerchantResult<LoginResponse>>(&s).unwrap();
            info!("token is {:?}", marchant_result.data.token);
            let token = TokenDTO(marchant_result.data.token);
            APPLICATION_CONTEXT.set::<TokenDTO>(token);
        }
        Err(e) => info!("link nacos  error: {:?}", e),
    }
}

// search_date 10-21-2022
pub async fn query_system_order(search_date:String) -> Result<MerchantResult<PlanData<Plan>>> {
    let page_index = 1u64;

    
    let search_date = if search_date.is_empty(){
        let today = Local::now();
        date_utils::today_date_str(&today)
    }else{
        search_date
    };
    

    let mut merchant_result = query_order_by_page(page_index, &search_date).await?;

    let plan_data = &merchant_result.data;
    let total_data = plan_data.total_data;
    let page_size = plan_data.page_size;
    let remainder = total_data % page_size;
    let i = if remainder > 0 {
        total_data / page_size + 1
    } else {
        total_data / page_size
    };
    info!("merchant_result is {:?}", i);
    if i > 0 {
        for j in 2..i + 1 {
            let mut result = query_order_by_page(j, &search_date).await?;
            merchant_result
                .data
                .plan_list
                .append(&mut result.data.plan_list)
        }
    }
    info!(
        "merchant_result.data.plan_list.len() is {:?}",
        merchant_result.data.plan_list.len()
    );
    Ok(merchant_result)
}

async fn query_order_by_page(
    page_index: u64,
    search_date: &str,
) -> Result<MerchantResult<PlanData<Plan>>> {
    let page_index_string = page_index.to_string();
    let mut search_map = HashMap::<&str, &str>::new();
    search_map.insert("page_index", &page_index_string);
    search_map.insert("search_date", &search_date);
    search_map.insert("search_orderby", "1");
    search_map.insert("sign", "0");
    let token = &APPLICATION_CONTEXT.get::<TokenDTO>().0;
    let client = reqwest::Client::new();

    let token = &APPLICATION_CONTEXT.get::<TokenDTO>().0;
    let body = client
        .post(ORDER_URL.to_string())
        .header("TokenValue", token)
        .json(&search_map)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    match body {
        Ok(s) => {
            let merchant_result =
                serde_json::from_str::<MerchantResult<PlanData<Plan>>>(&s).unwrap();
            Ok(merchant_result)
            // let token = TokenDTO(token.data.token);
            // APPLICATION_CONTEXT.set::<TokenDTO>(token);
        }
        Err(e) => Err(Error::E(e.to_string())),
    }
}

///开始匹配当日订单。
pub async fn match_today_order(search_date:String) -> Result<Vec<(Plan, DragonDataDTO)>> {
    //加载今日用户下单数据
    let today = Local::now();
    let dragon_date = date_utils::today_dragon_str(&today);
    let mut dragon_today_order_list: Vec<DragonDataDTO> =
        dragon_data_service::list(&dragon_date).await?;
    info!("dragon_today_order_list.len() is {:#?}", dragon_today_order_list.len());
    let mut system_order_list: Vec<Plan> = query_system_order(search_date).await?.data.plan_list;
    
    info!("system_order_list.len() is {:#?}", system_order_list.len());
    return match_order(&mut dragon_today_order_list, &mut system_order_list).await;
}

/// 开始配单
/// 加载今日用户下单数据,加载系统订单数据
/// 计算当日下单数据金额总和，计算订单金额总和，保证订单金额总和大于用户下单。
/// 如果系统订单数量小于用户订单数量，则需要拆分系统订单。
/// 如果系统订单数量大于用户订单数量 ，则不需要拆单，只需要一个用户买多个单即可。
/// 依据以上规则生成相同的系统订单数和用户订单数量即可进行配单。
/// 将配单展示给用户。
pub async fn match_order(
    dragon_order_list: &mut Vec<DragonDataDTO>,
    system_order_list: &mut Vec<Plan>,
) -> Result<Vec<(Plan, DragonDataDTO)>> {
    let dragon_amount_sum: u64 = (&dragon_order_list).iter().map(|d| d.amount).sum();
    info!("dragon_amount_sum is:{}", dragon_amount_sum);
    //加载系统订单数据
    // let mut system_order_list:Vec<Plan> = query_order().await?.data.plan_list;
    let order_amount_sum: u64 = (&system_order_list).iter().map(|p| p.plan_price).sum();
    info!("order_amount_sum is:{}", order_amount_sum);
    if order_amount_sum > dragon_amount_sum {
        return Err(Error::from("user order not enough amount"));
    }

    // dragon_order_list.sort_by(|a,b|b.amount.cmp(&a.amount));
    // system_order_list.sort_by(|a,b|b.plan_price.cmp(&a.plan_price));
    sort_orders(dragon_order_list, system_order_list);
    // info!("system_order_list is:{:?}", system_order_list);
    // info!("dragon_order_list is:{:?}", dragon_order_list);
    //如果系统订单数量小于用户订单数量，则需要拆分系统订单
    if system_order_list.len() < dragon_order_list.len() {
        // 9850    10000
        // 8467    5000
        // 5232    5000
        // 2341    5000
        //         5000
        //
        // 8467    10000
        // 5232    5000
        // 4925    5000
        // 4925    5000
        // 2341    5000

        //开始系统拆单。
        //计算多出来的订单。
        let more_order_amount = dragon_order_list.len() - system_order_list.len();
        for _ in 0..more_order_amount {
            let system_order = system_order_list.get(0).unwrap();
            let mut split_orders = system_order.split();
            system_order_list.append(&mut split_orders);
            system_order_list.remove(0);
        }
        //确保订单是的数量一致。
        assert!(
            system_order_list.len() == system_order_list.len(),
            "系统订单不匹配"
        );
    }

    // 8467    10000
    // 5232    5000
    // 4925    5000
    // 4925    5000
    // 2341    5000
    for s in system_order_list.iter(){
        println!("{:?}",s.plan_price);
    }
    println!("------------");
    for d in dragon_order_list.iter(){
        println!("{:?}",d.amount);
    }
    // 检查当前排序中是否存在系统订单金额大于用户订单的情况，如果有，则进行拆单。
    check_amount_bt_plan_price(dragon_order_list, system_order_list);

    // 如果用户订单数量大于系统订单数量。
    // if system_order_list.len() < dragon_order_list.len() {
    // 9850    10000
    // 8468    5000
    // 5232    5000
    // 2341    5000
    //         5000
    //
    // 8468    10000
    // 5232    5000
    // 4925    5000
    // 4925    5000
    // 2341    5000

    //0 5232   10000
    //1 4925   5000
    //2 4925   5000
    //3 4234   5000
    //4 4234   5000
    //5 2341
    // let mut merge_dragon_order = dragon_order_list.clone();
    info!("start merge_dragon_order");
    let mut merge_list: Vec<(Plan, DragonDataDTO)> = system_order_list
        .iter()
        .zip(dragon_order_list.iter_mut())
        .map(|(u, d)| {
            info!("d is:{:#?}", d);
            info!("u is:{:#?}", u);
            d.left_amount = Some(d.amount - u.plan_price);
            (u.clone(), d.clone())
        })
        .collect();

    // 将多余的订单配置给优先的金额有富余的用户。
    //判断是否有多余的订单。
    if system_order_list.len() > merge_list.len() {
        //获取所有的优先订单.
        // 优先订单排序
        dragon_order_list.sort_by(|a, b| b.left_amount.cmp(&a.left_amount));
        // 将未分配的订单分配给优先订单。
        let not_match_order_size = system_order_list.len() - merge_list.len();
        println!("not_match_order_size is:{:?}", not_match_order_size);
        for i in 0..not_match_order_size {
            println!("i is:{:?}", i);
            let len_of_system_order_list = system_order_list.len();
            let not_match_order = system_order_list
                .get(len_of_system_order_list - (not_match_order_size - i))
                .unwrap();
            //分配未匹配的订单。先找出所有剩余数量大于待分配量的订单，再找出有优先级的且剩余数量最大的订单，如果没有这样的订单则找到优先级最大的订单即可。
            let my_match_dragon_order;
            if let Some(match_dragon_order) = dragon_order_list
                .iter()
                .find(|d| d.prior.is_some() && d.left_amount.unwrap() >= not_match_order.plan_price)
            {
                my_match_dragon_order = match_dragon_order;
            } else if let Some(match_dragon_order) = dragon_order_list
                .iter()
                .find(|d| d.left_amount.unwrap() >= not_match_order.plan_price)
            {
                my_match_dragon_order = match_dragon_order;
            } else {
                panic!("order not match");
            }
            merge_list.push((not_match_order.clone(), my_match_dragon_order.clone()));
        }
    }

    // }
    Ok(merge_list)
}

// 8468    10000
// 5232    5000
// 4925    5000
// 4925    5000
// 2341    5000
// 检查当前排序中是否存在系统订单金额大于用户订单的情况，如果有，则进行拆单。
fn check_amount_bt_plan_price(
    dragon_order_list: &mut Vec<DragonDataDTO>,
    system_order_list: &mut Vec<Plan>,
) {
    // 重新排序。
    sort_orders(dragon_order_list, system_order_list);
    let mut over_system_order_id = 0;
    //并检查订单金额是否符合要求
    if let Some((system_order_index, plan_order)) =
        system_order_list
            .iter_mut()
            .enumerate()
            .find(|(index, system_order)| {
                dragon_order_list.get(*index).is_some()
                    && system_order.plan_price > dragon_order_list.get(*index).unwrap().amount
            })
    {
        // info!("match_order is: {:?},buyer name is:{:?}", match_order.0.plan_price,match_order.0.buyer_name);
        // info!("system_order_list.len() is: {:?}", system_order_list.len());
        //需要继续拆单。
        // let system_order = plan_order;
        // let remove_order = system_order_list.remove(match_order.0);
        let mut split_orders = plan_order.split();
        system_order_list.append(&mut split_orders);
        over_system_order_id = system_order_index;
        // system_order_list.drain_filter(|s| s.planid == system_order.planid);
        // 递归调用 重新检查是否仍有amount 大于plan的数据
    }
    if over_system_order_id > 0 {
        system_order_list.remove(over_system_order_id);
        check_amount_bt_plan_price(dragon_order_list, system_order_list);
    }
}

fn sort_orders(dragon_order_list: &mut Vec<DragonDataDTO>, system_order_list: &mut Vec<Plan>) {
    dragon_order_list.sort_by(|a, b| b.amount.cmp(&a.amount));
    system_order_list.sort_by(|a, b| b.plan_price.cmp(&a.plan_price));
}

#[cfg(test)]
mod tests {
    use cassie_domain::dto::{dragon_data_dto::DragonDataDTO, merchant_dto::Plan};

    use crate::merchant_req::merchant_service::{check_amount_bt_plan_price};

    use super::{match_order, sort_orders};

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    //   #[test]
    //   fn test_match_today_order(){
    //     let result = aw!(match_today_order());
    //     info!("result: {:?}", result);
    //   }

    // 9850     10000
    // 8468     5000
    // 5232     5000
    // 2341     5000
    //          5000
    //          2000
    //          2000

    //0 8468   10000
    //1 4925   5000
    //2 4925   5000
    //3 5232   5000
    //4 2341   5000
    #[test]
    fn test_match_order() {
        let mut dragon_order_list: Vec<DragonDataDTO> = Default::default();
        let mut d1 = DragonDataDTO::default();
        let mut d2 = DragonDataDTO::default();
        let mut d3 = DragonDataDTO::default();
        let mut d4 = DragonDataDTO::default();
        let mut d5 = DragonDataDTO::default();
        d1.amount = 10000;
        d2.amount = 5000;
        d3.amount = 5000;
        d4.amount = 5000;
        d5.amount = 5000;
        dragon_order_list.push(d1);
        dragon_order_list.push(d2);
        dragon_order_list.push(d3);
        dragon_order_list.push(d4);
        dragon_order_list.push(d5);

        // let dragon_amount_sum: u64 = (&dragon_order_list).iter().map(|d| d.amount).sum();
        // println!("dragon_amount_sum is {:?}", dragon_amount_sum);
        //加载系统订单数据
        let mut system_order_list: Vec<Plan> = Default::default();
        let mut p1 = Plan::default();
        let mut p2 = Plan::default();
        let mut p3 = Plan::default();
        let mut p4 = Plan::default();
        // let mut p5 = Plan::default();
        p1.plan_price = 9850;
        p2.plan_price = 8468;
        p3.plan_price = 5232;
        p4.plan_price = 2341;
        system_order_list.push(p1);
        system_order_list.push(p2);
        system_order_list.push(p3);
        system_order_list.push(p4);
        let result = aw!(match_order(&mut dragon_order_list, &mut system_order_list));
        println!("result is {:#?}", result);
    }

    // 8467    10000
    // 5232    5000
    // 5125    5000
    // 4925    5000
    // 2341    5000
    #[test]
    fn test_amount_bt_plan_price() {
        let mut dragon_order_list: Vec<DragonDataDTO> = Default::default();
        let mut d1 = DragonDataDTO::default();
        let mut d2 = DragonDataDTO::default();
        let mut d3 = DragonDataDTO::default();
        let mut d4 = DragonDataDTO::default();
        let mut d5 = DragonDataDTO::default();
        d1.amount = 10000;
        d2.amount = 5000;
        d3.amount = 5000;
        d4.amount = 5000;
        d5.amount = 5000;
        dragon_order_list.push(d1);
        dragon_order_list.push(d2);
        dragon_order_list.push(d3);
        dragon_order_list.push(d4);
        dragon_order_list.push(d5);

        // let dragon_amount_sum: u64 = (&dragon_order_list).iter().map(|d| d.amount).sum();
        // println!("dragon_amount_sum is {:?}", dragon_amount_sum);
        //加载系统订单数据
        let mut system_order_list: Vec<Plan> = Default::default();
        let mut p1 = Plan::default();
        let mut p2 = Plan::default();
        let mut p3 = Plan::default();
        let mut p4 = Plan::default();
        let mut p5 = Plan::default();
        p1.plan_price = 8467;
        p2.plan_price = 5232;
        p3.plan_price = 5125;
        p4.plan_price = 4925;
        p5.plan_price = 2341;
        system_order_list.push(p1);
        system_order_list.push(p2);
        system_order_list.push(p3);
        system_order_list.push(p4);
        system_order_list.push(p5);

        check_amount_bt_plan_price(&mut dragon_order_list, &mut system_order_list);
        println!("system_order_list is:{:#?}", system_order_list);
        println!("dragon_order_list is:{:#?}", dragon_order_list);
    }

    #[test]
    fn test_sort_order() {
        let mut dragon_order_list: Vec<DragonDataDTO> = Default::default();
        let mut d1 = DragonDataDTO::default();
        d1.amount = 100;
        let mut d2 = DragonDataDTO::default();
        d2.amount = 200;
        dragon_order_list.push(d1);
        dragon_order_list.push(d2);

        //加载系统订单数据
        let mut system_order_list: Vec<Plan> = Default::default();
        let mut p1 = Plan::default();
        let mut p2 = Plan::default();
        p1.plan_price = 20;
        p2.plan_price = 10;
        system_order_list.push(p1);
        system_order_list.push(p2);

        sort_orders(&mut dragon_order_list, &mut system_order_list);
        println!("system_order_list is:{:#?}", system_order_list);
        println!("dragon_order_list is:{:#?}", dragon_order_list);
    }

    #[test]
    pub fn test_order_amount() {
        let mut dragon_order_list: Vec<DragonDataDTO> = Default::default();
        let mut d1 = DragonDataDTO::default();
        d1.amount = 100;
        let mut d2 = DragonDataDTO::default();
        d2.amount = 200;
        dragon_order_list.push(d1);
        dragon_order_list.push(d2);

        let dragon_amount_sum: u64 = (&dragon_order_list).iter().map(|d| d.amount).sum();
        println!("dragon_amount_sum is {:?}", dragon_amount_sum);
        //加载系统订单数据
        let mut system_order_list: Vec<Plan> = Default::default();
        let mut p1 = Plan::default();
        let mut p2 = Plan::default();
        p1.plan_price = 10;
        p2.plan_price = 20;
        system_order_list.push(p1);
        system_order_list.push(p2);
        let order_amount_sum: u64 = (&system_order_list).iter().map(|p| p.plan_price).sum();
        println!("order_amount_sum is {:?}", order_amount_sum);
    }

    // 5232   10000
    // 4925   5000
    // 4925   5000
    // 4234   5000
    // 4234   5000
    // 2341
    #[test]
    pub fn test_match_dragon_order() {
        let mut dragon_order_list: Vec<DragonDataDTO> = Default::default();
        let mut d1 = DragonDataDTO::default();
        let mut d2 = DragonDataDTO::default();
        let mut d3 = DragonDataDTO::default();
        let mut d4 = DragonDataDTO::default();
        let mut d5 = DragonDataDTO::default();
        d1.amount = 10000;
        d2.amount = 5000;
        d3.amount = 5000;
        d4.amount = 5000;
        d5.amount = 5000;
        dragon_order_list.push(d1);
        dragon_order_list.push(d2);
        dragon_order_list.push(d3);
        dragon_order_list.push(d4);
        dragon_order_list.push(d5);

        let dragon_amount_sum: u64 = (&dragon_order_list).iter().map(|d| d.amount).sum();
        println!("dragon_amount_sum is {:?}", dragon_amount_sum);
        //加载系统订单数据
        let mut user_order_list: Vec<Plan> = Default::default();
        let mut p1 = Plan::default();
        let mut p2 = Plan::default();
        let mut p3 = Plan::default();
        let mut p4 = Plan::default();
        let mut p5 = Plan::default();
        let mut p6 = Plan::default();
        p1.plan_price = 5232;
        p2.plan_price = 4925;
        p3.plan_price = 4925;
        p4.plan_price = 4234;
        p5.plan_price = 4234;
        p6.plan_price = 2341;
        user_order_list.push(p1);
        user_order_list.push(p2);
        user_order_list.push(p3);
        user_order_list.push(p4);
        user_order_list.push(p5);
        user_order_list.push(p6);
        let order_amount_sum: u64 = (&user_order_list).iter().map(|p| p.plan_price).sum();
        println!("order_amount_sum is {:?}", order_amount_sum);
        println!("user_order_list is {:#?}", user_order_list);
        println!("dragon_order_list is {:#?}", dragon_order_list);
        let merge_list: Vec<(&Plan, DragonDataDTO)> =
            user_order_list.iter().zip(dragon_order_list).collect();
        println!("merge_list is {:#?}", merge_list);
    }
}
