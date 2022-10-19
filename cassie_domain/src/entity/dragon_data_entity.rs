use regex::Regex;
use serde::{Deserialize, Serialize};

//接龙
#[derive(Clone, Debug, Serialize, Deserialize,Getters,Setters)]
pub struct DragonData {
    pub id:Option<u64>,
    pub no: u64,
    pub name: String,
    pub amount: u64,
    pub prior: Option<u8>,
    pub disable:Option<u8>,
    pub create_date:Option<String>,
}

crud!(DragonData {});

impl_update!(DragonData{update_by_name(name:&str) => "`where id = 1`"});

impl_field_name_method!(DragonData {
    no,
    name,
    amount,
});

impl TryFrom<&str> for DragonData {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let mut data: [&str; 3] = Default::default();
        let head_regred = Regex::new(r"^[0-9]*\.").unwrap();
        let tail_reggred = Regex::new(r"[0-9]*$").unwrap();
        data[0] = head_regred.find(value).unwrap().as_str();
        data[2] = tail_reggred.find(value).unwrap().as_str();
        data[1] = value
            .strip_prefix(data[0])
            .unwrap()
            .strip_suffix(data[2])
            .unwrap()
            .trim();
        let dragon = Self {
            id:None,
            no: match data[0].strip_suffix(".").unwrap().parse() {
                Ok(n) => n,
                Err(e) => {
                    println!("value is:{:?},error data is {:?},parse error: {:?}",value, data[0], e);
                    panic!("error parsing data");
                }
            },
            name: data[1].to_string(),
            amount: match data[2].parse() {
                Ok(n) => n,
                Err(e) => {
                    println!("value is:{:?},error data is {:?},parse error: {:?}",value, data[2], e);
                    0
                    // panic!("error parsing data[2]");
                }
            },
            prior: Some(0),
            disable:Some(0),
            create_date:None,
        };
        Ok(dragon)
    }
}
