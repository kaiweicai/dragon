use serde::{Deserialize, Serialize};

use crate::entity::dragon_data_entity::DragonData;

//接龙
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DragonDataDTO {
    pub no: u64,
    pub name: String,
    pub amount: u64,
    pub prior: Option<bool>,
    pub disable: Option<bool>,
    pub create_date: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DragonDataVecDTO {
    pub amount: u64,
    pub dragonDataVec: Vec<DragonData>,
}

impl From<DragonData> for DragonDataDTO {
    fn from(dragon_data: DragonData) -> Self {
        Self {
            no: dragon_data.no,
            name: dragon_data.name,
            amount: dragon_data.amount,
            prior: match dragon_data.prior {
                Some(0) => Some(false),
                None => Some(false),
                _ => Some(true),
            },
            disable: match dragon_data.disable {
                Some(0) => Some(false),
                None => Some(false),
                _ => Some(true),
            },
            create_date: dragon_data.create_date,
        }
    }
}

impl Into<DragonData> for DragonDataDTO {
    fn into(self) -> DragonData {
        DragonData {
            no: self.no,
            name: self.name,
            amount: self.amount,
            prior: match self.prior {
                Some(false) => Some(0),
                None => Some(0),
                _ => Some(1),
            },
            disable: match self.disable {
                Some(false) => Some(0),
                None => Some(0),
                _ => Some(1),
            },
            create_date: self.create_date,
        }
    }
}
