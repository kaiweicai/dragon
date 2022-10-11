use crate::entity::{ dragon_orign::DragonOrigin};
use serde::{Deserialize, Serialize};
use validator_derive::Validate;
#[derive(Clone, Debug, Serialize, Validate, Deserialize, Getters, Setters, Default)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub", set = "pub")]
pub struct DragonOriginDTO {
    id: Option<i64>,
    content: String,
    create_date: Option<String>,
}

impl Into<DragonOrigin> for DragonOriginDTO {
    fn into(self) -> DragonOrigin {
        DragonOrigin {
            id: self.id,
            content: self.content,
            create_date: self.create_date,
        }
    }
}

impl From<DragonOrigin> for DragonOriginDTO {
    fn from(arg: DragonOrigin) -> Self {
        Self {
            id: arg.id,
            content: arg.content,
            create_date: arg.create_date,
        }
    }
}
