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
pub async fn query_system_order(search_date: String) -> Result<MerchantResult<PlanData<Plan>>> {
    let page_index = 1u64;

    let search_date = if search_date.is_empty() {
        let today = Local::now();
        date_utils::today_date_str(&today)
    } else {
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
pub async fn match_today_order(search_date: String) -> Result<Vec<(Plan, DragonDataDTO)>> {
    //加载今日用户下单数据
    let today = Local::now();
    let dragon_date = date_utils::today_dragon_str(&today);
    let mut dragon_today_order_list: Vec<DragonDataDTO> =
        dragon_data_service::list(&dragon_date).await?;
    info!(
        "dragon_today_order_list.len() is {:#?}",
        dragon_today_order_list.len()
    );
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
    let dragon_amount_sum: i64 = (&dragon_order_list).iter().map(|d| d.amount).sum();
    info!("dragon_amount_sum is:{}", dragon_amount_sum);
    //1. 加载系统订单数据
    let order_amount_sum: i64 = (&system_order_list).iter().map(|p| p.plan_price).sum();
    info!("order_amount_sum is:{}", order_amount_sum);
    if order_amount_sum > dragon_amount_sum {
        return Err(Error::from("user order not enough amount"));
    }

    //2.如果系统订单数量小于用户订单数量，则需要拆分系统订单
    // 9850    10000
    // 8467    5000
    // 5232    5000
    // 2341    5000
    //         5000
    //-------------------
    // 把9850订单拆成两个订单，后重新排序
    // 8467    10000
    // 5232    5000
    // 4925    5000
    // 4925    5000
    // 2341    5000
    divide_system_order(dragon_order_list, system_order_list);

    // 8467    10000
    // 5232    5000
    // 4925    5000
    // 4925    5000
    // 2341    5000
    for s in system_order_list.iter() {
        println!("{:?}", s.plan_price);
    }
    println!("------------");
    for d in dragon_order_list.iter() {
        println!("{:?}", d.amount);
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
            d.left_amount = d.left_amount - u.plan_price;
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
                .find(|d| d.prior.is_some() && d.left_amount >= not_match_order.plan_price)
            {
                my_match_dragon_order = match_dragon_order;
            } else if let Some(match_dragon_order) = dragon_order_list
                .iter()
                .find(|d| d.left_amount >= not_match_order.plan_price)
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

// 9850    10000
// 8467    5000
// 5232    5000
// 2341    5000
//         5000
//-------------------
// 检查是否系统订单多余用户接龙订单，如果系统订单多余用户接龙订单，则必须要拆单。
// 从金额最大的订单开始拆单，直到拆到系统的订单和用户的订单一样多为止。
// 把9850订单拆成两个订单，后重新排序
// 8467    10000
// 5232    5000
// 4925    5000
// 4925    5000
// 2341    5000
fn divide_system_order(
    dragon_order_list: &mut Vec<DragonDataDTO>,
    system_order_list: &mut Vec<Plan>,
) {
    sort_orders(dragon_order_list, system_order_list);
    if system_order_list.len() < dragon_order_list.len() {
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
            system_order_list.len() >= dragon_order_list.len(),
            "系统订单不匹配"
        );
    }
}

//经过divide_system_order检查后。
// 再检查是否有系统订单的金额超过用户的订单金额的匹对。如果有的话，检查用户订单的剩余金额是否有大于系统订单计划价格的订单。
// 如果满足条件，则增加一个amount为leftamount的用户接龙单，相当于一个用户支付两个订单。vec<>订单包含的个数需要加1。
// 此方法需要递归检查，如果一直存在，则一直减少amount为leftamount。
// 注意如果有下单量等于0的接龙单会死循环。如果有用户下单量为0的接龙单，需要排除。
// 5232    10000
// 4425    5000
// 4425    4000
// 4233    4000
// 4233    3000
// 2341    3000
fn over_plan_match_left_dradon(
    dragon_order_list: &mut Vec<DragonDataDTO>,
    system_order_list: &mut Vec<Plan>,
) {
    let mut dragon_default =DragonDataDTO::default();
    // 重新排序。
    sort_orders(dragon_order_list, system_order_list);
    //并检查订单金额是否符合要求
    let (over_system_order_id, dragon_order) = if let Some((system_order_index, dragon_order)) =
        system_order_list
            .iter()
            .zip(dragon_order_list.iter_mut())
            .map(|(u, d)| {
                d.left_amount = d.amount - u.plan_price; // 转变计算left_amount的量，修改了dragon_order_list内的dto的数据。
                d
            })
            .enumerate()
            .find(|(i, d)| d.left_amount < 0)
    {
        (system_order_index, dragon_order)
    } else {
        (0, &mut dragon_default)
    };

    if over_system_order_id > 0 {
        //先找有没有left能够容纳的订单。
        let dragon_over = if let Some((index, dragon)) = dragon_order_list
            .iter_mut()
            .enumerate()
            .find(|(index, dragon_order)| {
                system_order_list
                    .get(over_system_order_id)
                    .unwrap()
                    .plan_price <= dragon_order.left_amount
            }) {
            Some(dragon)
        } else {
            None
        };
        // 如果有能够容纳的订单，则用户订单包容这个新订单。
        if dragon_over.is_some() {
            let mut dragon_over = dragon_over.unwrap();
            let mut dragon_over_clone = dragon_over.clone();
            dragon_over.left_amount = dragon_over.left_amount - 4425;
            dragon_over_clone.amount = dragon_over_clone.left_amount;
            dragon_over_clone.match_plan_ids.push(over_system_order_id as u64);
            dragon_order_list.push(dragon_over_clone);
            over_plan_match_left_dradon(dragon_order_list, system_order_list);
        }
    }
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
    // ------------------继续拆单

    //如果还有订单价格大于用户出价的，则拆分成两个小订单。
    let over_system_order_id = if let Some((system_order_index, plan_order)) = system_order_list
        .iter_mut()
        .enumerate()
        .find(|(index, system_order)| {
            dragon_order_list.get(*index).is_some()
                && system_order.plan_price > dragon_order_list.get(*index).unwrap().amount
        }) {
        // info!("match_order is: {:?},buyer name is:{:?}", match_order.0.plan_price,match_order.0.buyer_name);
        // info!("system_order_list.len() is: {:?}", system_order_list.len());
        //需要继续拆单。
        // let system_order = plan_order;
        // let remove_order = system_order_list.remove(match_order.0);
        // let mut split_orders = plan_order.split();
        // system_order_list.append(&mut split_orders);
        system_order_index
        // system_order_list.drain_filter(|s| s.planid == system_order.planid);
        // 递归调用 重新检查是否仍有amount 大于plan的数据
    } else {
        0
    };
    if over_system_order_id > 0 {
        let removed_order = system_order_list.remove(0);
        let mut split_orders = removed_order.split();
        system_order_list.append(&mut split_orders);
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

    use crate::merchant_req::merchant_service::check_amount_bt_plan_price;

    use super::{divide_system_order, match_order, over_plan_match_left_dradon, sort_orders};

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    // 8850    10000
    // 8467    5000
    // 5232    4000
    // 2341    4000
    //         3000
    //         3000
    //-------------------
    // 把9850订单拆成两个订单，后重新排序
    // 5232    10000
    // 4425    5000
    // 4425    4000
    // 4233    4000
    // 4233    3000
    // 2341    3000
    #[test]
    fn test_divide_system_order() {
        let mut amount_vec = Vec::new();
        amount_vec.push(10000);
        amount_vec.push(5000);
        amount_vec.push(4000);
        amount_vec.push(4000);
        amount_vec.push(3000);
        amount_vec.push(3000);
        let mut dragon_order_list: Vec<DragonDataDTO> = mock_dragon_order_list(&amount_vec);

        //加载系统订单数据
        let mut plan_price_vec = Vec::new();
        plan_price_vec.push(8850);
        plan_price_vec.push(8467);
        plan_price_vec.push(5232);
        plan_price_vec.push(2341);
        let mut system_order_list: Vec<Plan> = mock_system_order_list(&plan_price_vec);

        divide_system_order(&mut dragon_order_list, &mut system_order_list);
        show_base_system_orders(&mut dragon_order_list, &mut system_order_list);
    }

    fn show_base_system_orders(
        dragon_order_list: &mut Vec<DragonDataDTO>,
        system_order_list: &mut Vec<Plan>,
    ) {
        sort_orders(dragon_order_list, system_order_list);
        system_order_list.iter().enumerate().for_each(|(i, p)| {
            print!("{:5?} ", p.plan_price);
            let dragon = dragon_order_list.get(i);
            print!(
                "{:5?} ",
                dragon.unwrap_or(&DragonDataDTO::default()).amount()
            );
            print!(
                "{:5?} ",
                dragon.unwrap_or(&DragonDataDTO::default()).left_amount
            );
            println!("")
        });
    }

    fn show_base_dragon_orders(
        dragon_order_list: &mut Vec<DragonDataDTO>,
        system_order_list: &mut Vec<Plan>,
    ) {
        sort_orders(dragon_order_list, system_order_list);
        dragon_order_list.iter().enumerate().for_each(|(i, d)| {
            let plan = system_order_list.get(i);
            print!("{:5?} ", plan.unwrap_or(&Plan::default()).plan_price);
            print!("{:5?} ", d.amount);
            print!("{:5?} ", d.left_amount);
            print!("{:5?} ", d.origin_amount);
            println!("")
        });
    }

    #[test]
    fn test_over_plan_match_left_dradon() {
        let mut amount_vec = Vec::new();
        amount_vec.push(10000);
        amount_vec.push(5000);
        amount_vec.push(4000);
        amount_vec.push(4000);
        amount_vec.push(3000);
        amount_vec.push(3000);
        let mut dragon_order_list: Vec<DragonDataDTO> = mock_dragon_order_list(&amount_vec);

        //加载系统订单数据
        let mut plan_price_vec = Vec::new();
        plan_price_vec.push(5232);
        plan_price_vec.push(4425);
        plan_price_vec.push(4425);
        plan_price_vec.push(4233);
        plan_price_vec.push(4233);
        plan_price_vec.push(2341);
        let mut system_order_list: Vec<Plan> = mock_system_order_list(&plan_price_vec);
        // 5232    10000
        // 4425    5000
        // 4425    4000
        // 4233    4000
        // 4233    3000
        // 2341    3000
        over_plan_match_left_dradon(&mut dragon_order_list, &mut system_order_list);
        show_base_dragon_orders(&mut dragon_order_list, &mut system_order_list);
    }

    // 5232 10000
    // 4425 5000
    // 4425 4000
    // 4233 4000
    // 4233 3000
    // 2341 3000
    #[test]
    fn test_amount_bt_plan_price() {
        let mut amount_vec = Vec::new();
        amount_vec.push(10000);
        amount_vec.push(5000);
        amount_vec.push(4000);
        amount_vec.push(4000);
        amount_vec.push(3000);
        amount_vec.push(3000);
        let mut dragon_order_list: Vec<DragonDataDTO> = mock_dragon_order_list(&amount_vec);

        //加载系统订单数据
        let mut plan_price_vec = Vec::new();
        plan_price_vec.push(5232);
        plan_price_vec.push(4425);
        plan_price_vec.push(4425);
        plan_price_vec.push(4233);
        plan_price_vec.push(4233);
        plan_price_vec.push(2341);
        let mut system_order_list: Vec<Plan> = mock_system_order_list(&plan_price_vec);

        check_amount_bt_plan_price(&mut dragon_order_list, &mut system_order_list);
        println!("system_order_list is:{:#?}", system_order_list);
        println!("dragon_order_list is:{:#?}", dragon_order_list);
    }

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
    //整体测试订单匹配
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

    fn mock_dragon_order_list(amount_vec: &Vec<i64>) -> Vec<DragonDataDTO> {
        let mut result = Vec::new();
        amount_vec.iter().for_each(|amount| {
            let mut dragon = DragonDataDTO::default();
            dragon.set_amount(*amount);
            dragon.set_left_amount(*amount);
            dragon.set_origin_amount(*amount);
            result.push(dragon);
        });
        result
    }

    fn mock_system_order_list(plan_price_vec: &Vec<i64>) -> Vec<Plan> {
        let mut result = Vec::new();
        plan_price_vec.iter().for_each(|plan_price| {
            let mut plan = Plan::default();
            plan.plan_price = *plan_price;
            result.push(plan);
        });
        result
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

        let dragon_amount_sum: i64 = (&dragon_order_list).iter().map(|d| d.amount).sum();
        println!("dragon_amount_sum is {:?}", dragon_amount_sum);
        //加载系统订单数据
        let mut system_order_list: Vec<Plan> = Default::default();
        let mut p1 = Plan::default();
        let mut p2 = Plan::default();
        p1.plan_price = 10;
        p2.plan_price = 20;
        system_order_list.push(p1);
        system_order_list.push(p2);
        let order_amount_sum: i64 = (&system_order_list).iter().map(|p| p.plan_price).sum();
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

        let dragon_amount_sum: i64 = (&dragon_order_list).iter().map(|d| d.amount).sum();
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
        let order_amount_sum: i64 = (&user_order_list).iter().map(|p| p.plan_price).sum();
        println!("order_amount_sum is {:?}", order_amount_sum);
        println!("user_order_list is {:#?}", user_order_list);
        println!("dragon_order_list is {:#?}", dragon_order_list);
        let merge_list: Vec<(&Plan, DragonDataDTO)> =
            user_order_list.iter().zip(dragon_order_list).collect();
        println!("merge_list is {:#?}", merge_list);
    }
}
