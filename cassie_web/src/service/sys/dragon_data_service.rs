use std::collections::BTreeMap;

use crate::APPLICATION_CONTEXT;
use cassie_common::error::Result;
use cassie_domain::{dto::dragon_data_dto::DragonDataDTO, entity::dragon_data_entity::DragonData};
use log::info;
use rbatis::rbatis::Rbatis;

///查询接龙列表
pub async fn list(create_date: &str) -> Result<Vec<DragonDataDTO>> {
    let mut rb = APPLICATION_CONTEXT.get::<Rbatis>();
    let dragon_data_list: Vec<DragonDataDTO> =
        DragonData::select_by_column(&mut rb, "create_date", create_date)
            .await?
            .iter()
            .filter(|item| item.amount != 0)
            .map(|d| d.clone().into())
            .collect();
    info!("list dragon_data_list: {:?}", dragon_data_list.get(0));
    Ok(dragon_data_list)
}

//保存用户
pub async fn save_or_update_dragon_data(dragon: DragonDataDTO) -> Result<()> {
    let mut rb = APPLICATION_CONTEXT.get::<Rbatis>();
    let entity: DragonData = dragon.into();
    if let Some(id) = entity.id {
        DragonData::update_by_column(&mut rb, &entity, "id").await?;
    } else {
        DragonData::insert(&mut rb, &entity).await?;
    }
    Ok(())
}
// //删除
// pub async fn del(&self, id: String) {
//     let mut rb = APPLICATION_CONTEXT.get::<Rbatis>();
//     DragonOrigin::delete_by_column(&mut rb, "id", id).await;
// }

///获取当天接龙列表
pub async fn save_today_dragon_data(content: String, create_date: Option<String>) {
    let mut rb = APPLICATION_CONTEXT.get::<Rbatis>();
    let dragons = content.split('\n').collect::<Vec<&str>>();
    let mut invest_map = BTreeMap::<i64, Vec<DragonData>>::new();
    for line in dragons.iter() {
        let dragon_data: DragonData = (*line).try_into().unwrap();

        if let None = invest_map.get(&dragon_data.amount) {
            invest_map.insert(dragon_data.amount, Vec::new());
        }
        invest_map
            .get_mut(&dragon_data.amount)
            .unwrap()
            .push(dragon_data.clone());
    }
    for (_, dto_vec) in invest_map.iter_mut() {
        for dragon_data in dto_vec.iter_mut() {
            dragon_data.create_date = create_date.clone();
            DragonData::insert(&mut rb, &dragon_data).await;
        }
    }
}
