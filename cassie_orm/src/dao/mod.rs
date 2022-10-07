use cassie_config::config::WebApplicationConfig;
use rbatis::rbatis::Rbatis;
pub mod mapper;

///实例化 rbatis orm 连接池
pub async fn init_rbatis(cassie_config: &WebApplicationConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if cassie_config.debug().eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    //连接数据库
    println!(
        "rbatis link database ({})...",
        cassie_config.sqlitebase_url().clone()
    );
    rbatis
        .link(SqliteDriver {},&cassie_config.sqlitebase_url())
        .await
        .expect("rbatis link database fail!");
    println!("rbatis link database success!");

    return rbatis;
}

use mongodb::{options::ClientOptions, Client, Database};
use rbdc_sqlite::driver::SqliteDriver;

pub async fn init_mongodb(cassie_config: &WebApplicationConfig) -> Database {
    let client_options = ClientOptions::parse(cassie_config.mongodb_url().clone().as_str())
        .await
        .expect(" mongodb link database fail!");
    println!(
        "mongodb link database ({})...",
        cassie_config.mongodb_url().clone()
    );
    let client = Client::with_options(client_options).unwrap();
    println!("mongodb link database success!");
    let db = client.database("cassie");
    db
}
