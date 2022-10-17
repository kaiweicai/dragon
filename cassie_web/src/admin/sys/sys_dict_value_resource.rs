


use axum::routing::get;
use axum::Json;
use axum::{response::IntoResponse, Router};


use axum::extract::{Path, Query};

use cassie_domain::dto::sys_dict_dto::SysDictDataDTO;

use cassie_domain::request::SysDictQuery;

/**
 *method:/dict/type/page
 *desc:数据字典 分页查询
 *author:String
 *email:348040933@qq.com
 */

pub async fn page(arg: Option<Query<SysDictQuery>>) -> impl IntoResponse {
    // let sys_dict_value_service = APPLICATION_CONTEXT.get::<SysDictDataService>();
    // let arg = arg.unwrap();
    // let vo = sys_dict_value_service
    //     .page(
    //         &arg,
    //         PageData {
    //             page_no: arg.page().clone(),
    //             page_size: arg.limit().clone(),
    //         },
    //     )
    //     .await;

    // RespVO::from_result(&vo).resp_json()
    todo!()
}

/**
 *method:/dict/type/{id}
 *desc:数据字典id获取
 *author:String
 *email:348040933@qq.com
 */
pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    // let sys_dict_value_service = APPLICATION_CONTEXT.get::<SysDictDataService>();
    // let dto = sys_dict_value_service.get(id).await;
    // RespVO::from_result(&dto).resp_json()
}

/**
 *method:/dict/type/save
 *desc:数据字典保存
 *author:String
 *email:348040933@qq.com
 */
pub async fn save(Json(arg): Json<SysDictDataDTO>) -> impl IntoResponse {
    // let sys_dict_value_service = APPLICATION_CONTEXT.get::<SysDictDataService>();
    // let mut entity = arg.into();
    // let vo = sys_dict_value_service.save(&mut entity).await;
    // RespVO::from_result(&vo).resp_json()
    todo!()
}

pub async fn edit(Json(arg): Json<SysDictDataDTO>) -> impl IntoResponse {
    // let sys_dict_value_service = APPLICATION_CONTEXT.get::<SysDictDataService>();
    // let id = arg.id().clone();
    // let mut entity = arg.into();
    // sys_dict_value_service
    //     .update_by_id(id.unwrap().to_string(), &mut entity)
    //     .await;
    // RespVO::from(&"更新成功".to_string()).resp_json()
    todo!()
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    // let cassie_config = APPLICATION_CONTEXT.get::<WebApplicationConfig>();
    // if *cassie_config.is_demo() {
    //     return RespVO::from(&"演示删除成功".to_string()).resp_json();
    // }
    // let sys_dict_value_service = APPLICATION_CONTEXT.get::<SysDictDataService>();
    // sys_dict_value_service.del(&id).await;
    // RespVO::from(&"删除成功".to_string()).resp_json()
    todo!()
}

pub fn init_router() -> Router {
    Router::new()
        .route("/dict/value", get(page).post(save).put(edit))
        .route("/dict/value/:id", get(get_by_id).delete(delete))
}
