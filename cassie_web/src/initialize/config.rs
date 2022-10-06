use crate::APPLICATION_CONTEXT;
use cassie_config::config::WebApplicationConfig;
use log::info;
use tokio::fs::{read_to_string,read_dir};

//初始化配置信息
pub async fn init_config() {
    let file = read_dir("./").await.unwrap().next_entry().await;
    info!("current file is:{:?}",file);
    let content = read_to_string("./application.yml").await.unwrap();
    let mut config = WebApplicationConfig::new(content.as_str());
    let mut list = match config.admin_auth_list_api().clone() {
        Some(e) => e,
        None => Vec::new(),
    };
    /*添加需要登录但是不需要权限的路由
     * 如果有额外的可以在application.yml中添加
     * admin_auth_list_api
     *  - XXXXXX
     *  - XXXXX
     * */
    list.push("/user/info".to_string());
    list.push("/dict/type/all".to_string());
    list.push("/menu/nav".to_string());
    config.set_admin_auth_list_api(Some(list));
    APPLICATION_CONTEXT.set::<WebApplicationConfig>(config);
}
