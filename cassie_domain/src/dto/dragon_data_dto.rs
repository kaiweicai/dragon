use serde::{Deserialize, Serialize};

//接龙
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DragonDataDTO {
    pub no: u32,
    pub name: String,
    pub amount: u32,
    pub prior: Option<bool>,
}