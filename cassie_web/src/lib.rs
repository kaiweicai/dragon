#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate cached;
//配置
pub mod config;
//管理端
pub mod admin;
//权限模块
pub mod cici_casbin;
//中间件
pub mod middleware;
//nacos注册中心默认实现
pub mod nacos;
//路由
pub mod routers;
//总服务入口
pub mod service;
//前端接口
pub mod api;
pub mod initialize;
pub mod interceptor;
pub mod observe;
pub mod ws;

use std::collections::HashMap;
use std::time::Duration;

// use crate::initialize::casbin::init_casbin;
use crate::initialize::config::init_config;
use crate::initialize::database::init_database;
use crate::initialize::event::init_event_bus;
// use crate::initialize::rules::init_rules;
use crate::initialize::service::init_service;
use crate::interceptor::interceptor::AgencyInterceptor;
use crate::nacos::register_service;
use crate::ws::ws_server::init_ws;
// use crate::{cici_casbin::casbin_service::CasbinService, config::log::init_log};
use crate::{ config::log::init_log};
use axum::http::Uri;
use axum::response::IntoResponse;
use axum::{Router, Server};
use cassie_config::config::WebApplicationConfig;
pub use deno_runtime::deno_core;
use log::info;
use log::warn;
use middleware::get_local;
use observe::event::CassieEvent;
use observe::event::CustomEvent;
use service::fire_event;
use state::Container;
use tower_http::cors::{Any, CorsLayer};
use cassie_common::RespVO;
/*
整个项目上下文ApplicationContext
包括：
ApplicationConfig 配置
Database mongodb数据库
Rbatis  mysql orm
ServiceContext 服务上下文
CasbinService 权限服务
*/

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();
/*初始化环境上下文*/
pub async fn init_context() {
    print_banner();
    // //第一步加载配置
    init_config().await;
    //tauri 已经加载了log，故此处无需加载。
    // init_log();
    // info!("ConfigContext init complete");
    //第二步初始化数据源
    init_database().await;
    info!("DataBase init complete");
    // //第三步初始化所有的 服务类
    init_service().await;
    // info!("ServiceContext init complete");
    // //第三步初始化casbinCContext
    // init_casbin().await;
    // info!("CasbinService init complete");
    // init_rules().await;
    // info!("RulesContext init complete");
    init_event_bus().await;
    // info!("EventBus init complete");
    // tokio::spawn(init_ws());
    // //nacos 服务注册
    // register_service().await;
    // let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    // info!(
    //     " - Local:   http://{}:{}",
    //     cassie_config
    //         .server()
    //         .host()
    //         .replace("0.0.0.0", "127.0.0.1"),
    //     cassie_config.server().port()
    // );
}

async fn fire_script_event(
    params_values: HashMap<String, serde_json::Value>,
    return_values: serde_json::Value,
) {
    let request = get_local();
    let cus = CustomEvent {
        params_values: Some(params_values),
        return_values,
        request_model: request,
    };
    let event = CassieEvent::Custom(cus);
    fire_event(event).await;
}
fn print_banner() {
    let banner = r#"
  _____              _                                                     _           _       
 / ____|            (_)          /\                               /\      | |         (_)      
| |     __ _ ___ ___ _  ___     /  \   __  ___   _ _ __ ___      /  \   __| |_ __ ___  _ _ __  
| |    / _` / __/ __| |/ _ \   / /\ \  \ \/ / | | | '_ ` _ \    / /\ \ / _` | '_ ` _ \| | '_ \ 
| |___| (_| \__ \__ \ |  __/  / ____ \  >  <| |_| | | | | | |  / ____ \ (_| | | | | | | | | | |
 \_____\__,_|___/___/_|\___| /_/    \_\/_/\_\\__,_|_| |_| |_| /_/    \_\__,_|_| |_| |_|_|_| |_|

"#;
    println!("{}", banner);
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    let msg = format!("资源不存在：{}", uri);
    warn!("{}", msg.clone());
    RespVO::<String> {
        code: Some(-1),
        msg: Some(msg),
        data: None,
    }
    .resp_json()
}

//初始化一个本地server
pub fn init_server() {
    tokio::spawn(async {
        info!("start initialize the web server");
        //初始化上环境下文
        init_context().await;

        let cassie_config = APPLICATION_CONTEXT.get::<WebApplicationConfig>();
        let server = format!(
            "{}:{}",
            cassie_config.server().host(),
            cassie_config.server().port()
        );
        info!("start initialize the web server is:{:?}",&server);
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(Any)
            .allow_headers(Any)
            .max_age(Duration::from_secs(60) * 10);
        //绑定端口 初始化 路由
        let app = Router::new()
            .nest("/admin", routers::admin::routers())
            // .nest("/api", api::routers())
            .layer(cors);
            // .fallback(fallback.into_service());
        // 启动服务
        Server::bind(&server.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
        info!("end initialize the web server");
    });
}
