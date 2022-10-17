use std::vec;

use crate::service::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use cassie_common::error::Result;
use cassie_domain::dto::sys_menu_dto::SysMenuDTO;
use cassie_domain::entity::sys_entitys::{SysMenu};
use cassie_domain::request::tree::TreeService;


/**
*struct:SysMenuService
*desc:菜单基础服务
*author:String
*email:348040933@qq.com
*/
pub struct SysMenuService {}

impl SysMenuService {
    //保存或者更新
    pub async fn save_or_update(&self, dto: SysMenuDTO) {
        // let mut entity: SysMenu = dto.into();
        // //保存或更新菜单
        // let id = if let Some(id) = entity.id {
        //     self.update_by_id(id.to_string(), &entity).await;
        //     id
        // } else {
        //     entity.del_flag = Option::Some(0);
        //     let role_id = self.save(&mut entity).await;
        //     role_id.unwrap()
        // };
        todo!()
    }
    //获取所有的菜单
    pub async fn menu_list(&self) -> Result<Vec<SysMenuDTO>> {
        // let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        // let result = menu_list(&mut rb.as_executor(), "").await.unwrap();
        Ok(Default::default())
    }
}
impl Default for SysMenuService {
    fn default() -> Self {
        SysMenuService {}
    }
}
// impl CrudService<SysMenu, SysMenuDTO, SysMenuQuery> for SysMenuService {
//     fn get_wrapper(arg: &SysMenuQuery) -> Wrapper {
//         let rb = APPLICATION_CONTEXT.get::<Rbatis>();
//         let mut wrapper = rb.new_wrapper();
//         if let Some(id_list) = &arg.ids() {
//             wrapper = wrapper.r#in(SysMenu::id(), id_list);
//         }
//         if let Some(id_list) = &arg.pids() {
//             wrapper = wrapper.r#in(SysMenu::pid(), id_list);
//         }
//         wrapper
//     }
//     fn set_save_common_fields(&self, common: CommonField, data: &mut SysMenu) {
//         data.id = common.id;
//         data.creator = common.creator;
//         data.create_date = common.create_date;
//     }
// }
impl TreeService<SysMenu, SysMenuDTO> for SysMenuService {
    fn set_children(&self, arg: &mut SysMenuDTO, childs: Option<Vec<SysMenuDTO>>) {
        arg.set_children(childs);
    }
}

pub async fn get_user_menu_list(
    uid: String,
    super_admin: i32,
    agency_code: String,
) -> Result<Vec<SysMenuDTO>> {
    // let rb = APPLICATION_CONTEXT.get::<Rbatis>();
    // print!("{}", super_admin);
    // let result = if super_admin > 0 {
    //     menu_list(&mut rb.as_executor(), "0").await.unwrap()
    // } else {
    //     user_menu_list(
    //         &mut rb.as_executor(),
    //         uid.as_str(),
    //         "0",
    //         agency_code.as_str(),
    //     )
    //     .await
    //     .unwrap()
    // };
    let result = Some(vec![
        SysMenu {
            id: Some(1),
            pid: Some(0),
            url: None,
            name: Some("接龙管理".to_string()),
            menu_type: Some(0),
            icon: None,
            permissions: Some("".to_string()),
            sort: Some(1),
            del_flag: Some(0),
            creator: Some(1),
            updater: Some(1),
            method: Some("".to_string()),
            path: None,
            create_date: None,
            update_date: None,
        },
        SysMenu {
            id: Some(2),
            pid: Some(1),
            url: Some("dragon/list".to_string()),
            name: Some("添加接龙".to_string()),
            menu_type: Some(0),
            icon: None,
            permissions: Some("".to_string()),
            sort: Some(1),
            del_flag: Some(0),
            creator: Some(1),
            updater: Some(1),
            method: Some("".to_string()),
            path: None,
            create_date: None,
            update_date: None,
        },
    ]);

    let sys_menu_service = APPLICATION_CONTEXT.get::<SysMenuService>();
    Ok(sys_menu_service.build(result.unwrap()))
}
