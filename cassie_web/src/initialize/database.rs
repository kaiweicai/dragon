use crate::{AgencyInterceptor, APPLICATION_CONTEXT};
use cassie_config::config::WebApplicationConfig;
use cassie_orm::dao::{init_mongodb, init_rbatis};
use log::info;
use mongodb::Database;
use rbatis::rbatis::Rbatis;

pub async fn init_database() {
    let config = APPLICATION_CONTEXT.get::<WebApplicationConfig>();

    let rbatis = init_rbatis(config).await;
    // rbatis.add_sql_intercept(AgencyInterceptor {
    //     enable: config.tenant().enable().clone(),
    //     column: config.tenant().column().clone(),
    //     ignore_table: config.tenant().ignore_table().clone(),
    // });
    info!("link database success!{}", config.database_url());
    APPLICATION_CONTEXT.set::<Rbatis>(rbatis);

    let mdb = init_mongodb(config).await;
    APPLICATION_CONTEXT.set::<Database>(mdb);
    info!("mongodb link database ({})...", config.mongodb_url());
}
