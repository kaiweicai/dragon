use crate::APPLICATION_CONTEXT;
use cassie_common::error::Result;
use cassie_domain::{dto::dragon_dto::DragonOriginDTO, entity::dragon_orign::DragonOrigin};
use rbatis::rbatis::Rbatis;

// use crate::cici_casbin::casbin_service::CasbinService;

/**
 *struct:DragonOriginService
 *desc:原始接龙
 *author:String
 *email:cloudweisz@gmail.com
 */
pub struct DragonService {}

impl DragonService {
    ///查询接龙列表
    pub async fn dragon_list(&self) -> Result<Vec<DragonOriginDTO>> {
        let mut rb = APPLICATION_CONTEXT.get::<Rbatis>();
        DragonOrigin::select_all(&mut rb)
            .await
            .map_err(|e| e.into())
            .map(|v| v.iter().map(|d| DragonOriginDTO::from(d.clone())).collect())
    }

    //保存用户
    pub async fn save(&self, dragon: DragonOriginDTO) {
        let mut rb = APPLICATION_CONTEXT.get::<Rbatis>();
        DragonOrigin::insert(&mut rb, &dragon.into()).await;
    }
    //删除
    pub async fn del(&self, id:String) {
        let mut rb = APPLICATION_CONTEXT.get::<Rbatis>();
        DragonOrigin::delete_by_column(&mut rb, "id", id).await;
    }
}
