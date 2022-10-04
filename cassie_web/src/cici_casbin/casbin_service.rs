// #![allow(clippy::type_complexity)]

// use std::sync::Arc;

// use crate::{cici_casbin::cici_match, APPLICATION_CONTEXT};
// use async_std::sync::RwLock;
// use casbin::{
//     prelude::{TryIntoAdapter, TryIntoModel},
//     CachedEnforcer, CoreApi, DefaultModel, Result as CasbinResult,
// };
// use cassie_casbin_adapter::cici_adapter::CICIAdapter;
// use cassie_config::config::ApplicationConfig;
// use cassie_orm::dao::init_rbatis;
// use log::info;

// #[derive(Clone)]
// pub struct CasbinVals {
//     pub uid: String,
//     pub agency_code: Option<String>,
// }

// /**
//  *struct:CasbinService
//  *desc:casbin 权限处理核心service
//  *author:String
//  *email:348040933@qq.com
//  */
// #[derive(Clone)]
// pub struct CasbinService {
//     pub enforcer: Arc<RwLock<CachedEnforcer>>,
// }

// impl CasbinService {
//     /**
//     sign_in
//      *method:default
//      *desc:初始化casbin 上下文 加载model 初始化 CICIAdapter casbin适配器
//      *author:String
//      *email:348040933@qq.com
//      */
//     pub async fn default() -> Self {
//         /*加载模型文件*/
//         let m = DefaultModel::from_str(MODEL_AUTH).await.unwrap();
//         info!("casbin model init success!");

//         let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
//         let a = CICIAdapter::new(init_rbatis(config).await);
//         let mut cached_enforcer = CachedEnforcer::new(m, a).await.unwrap();
//         /* 添加自定义验证方法 */
//         cached_enforcer.add_function("ciciMatch", cici_match);
//         info!("casbin init success");
//         Self {
//             enforcer: Arc::new(RwLock::new(cached_enforcer)),
//         }
//     }

//     pub async fn new<M: TryIntoModel, A: TryIntoAdapter>(m: M, a: A) -> CasbinResult<Self> {
//         let enforcer: CachedEnforcer = CachedEnforcer::new(m, a).await?;
//         Ok(CasbinService {
//             enforcer: Arc::new(RwLock::new(enforcer)),
//         })
//     }

//     pub fn get_enforcer(&self) -> Arc<RwLock<CachedEnforcer>> {
//         self.enforcer.clone()
//     }

//     pub fn set_enforcer(e: Arc<RwLock<CachedEnforcer>>) -> CasbinService {
//         CasbinService { enforcer: e }
//     }
//     /**
//      *method:call
//      *desc:核心验证方法 path ,action ,vals
//      *author:String
//      *email:348040933@qq.com
//      */
//     pub async fn call(&self, path: String, action: String, vals: CasbinVals) -> bool {
//         /*获取验证器*/
//         let cloned_enforcer = self.get_enforcer();
//         let uid = vals.uid.clone();
//         /*获取对应的 用户 用户为空直接返回false*/
//         if !vals.uid.is_empty() {
//             /*判断是否是多租户模型*/
//             let vecs = if let Some(agency_code) = vals.agency_code {
//                 vec![uid, agency_code, path, action]
//             } else {
//                 vec![uid, path, action]
//             };
//             let mut lock = cloned_enforcer.write().await;
//             match lock.enforce_mut(vecs) {
//                 Ok(true) => {
//                     drop(lock);
//                     true
//                 }
//                 Ok(false) => {
//                     drop(lock);
//                     false
//                 }
//                 Err(_) => {
//                     drop(lock);
//                     false
//                 }
//             }
//         } else {
//             false
//         }
//     }
// }

// const MODEL_AUTH: &str = r#"
//          [request_definition]
//         r = sub, dom, obj, act

//         [policy_definition]
//         p = sub, dom, obj, act

//         [role_definition]
//         g = _, _, _

//         [policy_effect]
//         e = some(where (p.eft == allow))

//         [matchers]
//         m = ciciMatch(r.sub,r.obj) || (g(r.sub, p.sub, r.dom) && r.dom == p.dom && keyMatch2(r.obj, p.obj) && r.act == p.act)
// "#;
