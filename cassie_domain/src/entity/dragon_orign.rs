use serde::{Deserialize, Serialize};

//#[crud_table(table_name:user)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DragonOrigin {
    pub id: i64,
    pub content: String,
    pub create_date: Option<String>,
}

crud!(DragonOrigin {});

impl_field_name_method!(DragonOrigin {
    id,
    content,
    create_date,
});
