


use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use cassie_domain::{
    dto::sys_group_data_dto::SysGroupDataDTO, request::SysGroupDataQuery,
};


pub async fn page(arg: Option<Query<SysGroupDataQuery>>) -> impl IntoResponse {
    // let service = APPLICATION_CONTEXT.get::<SysGroupDataService>();
    // let arg = arg.unwrap();
    // let vo = service
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

pub async fn list(arg: Option<Query<SysGroupDataQuery>>) -> impl IntoResponse {
    // let service = APPLICATION_CONTEXT.get::<SysGroupDataService>();
    // let arg = arg.unwrap();
    // let vo = service.list(&arg).await;
    // RespVO::from_result(&vo).resp_json()
    todo!()
}

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    // let service = APPLICATION_CONTEXT.get::<SysGroupDataService>();
    // let dto = service.get(id).await;
    // RespVO::from_result(&dto).resp_json()
    todo!()
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    // let service = APPLICATION_CONTEXT.get::<SysGroupDataService>();
    // service.del(&id).await;
    // RespVO::from(&"删除成功".to_string()).resp_json()
    todo!()
}

pub async fn save(Json(arg): Json<SysGroupDataDTO>) -> impl IntoResponse {
    // let service = APPLICATION_CONTEXT.get::<SysGroupDataService>();
    // let mut entity = arg.into();
    // let vo = service.save(&mut entity).await;
    // RespVO::from_result(&vo).resp_json()
    todo!()
}

pub async fn edit(Json(arg): Json<SysGroupDataDTO>) -> impl IntoResponse {
    // let service = APPLICATION_CONTEXT.get::<SysGroupDataService>();
    // let id = arg.id().clone();
    // let mut entity = arg.into();
    // service
    //     .update_by_id(id.unwrap().to_string(), &mut entity)
    //     .await;
    // RespVO::from(&"更新成功".to_string()).resp_json()
    todo!()
}

pub fn init_router() -> Router {
    Router::new()
        .route("/sys_group_data", get(page).post(save).put(edit))
        .route("/sys_group_data/list", get(list))
        .route("/sys_group_data/:id", get(get_by_id).delete(delete))
}
