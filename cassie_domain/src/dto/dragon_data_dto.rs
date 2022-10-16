use serde::{Deserialize, Serialize};

use crate::entity::dragon_data_entity::DragonData;

//接龙
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DragonDataDTO {
    pub no: u64,
    pub name: String,
    pub amount: u64,
    pub prior: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DragonDataVecDTO {
    pub amount: u64,
    pub dragonDataVec:Vec<DragonData>,
}

