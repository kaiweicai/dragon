// use super::{
//     crud_service::CrudService, sys_role_data_scope_service::SysRoleDataScopeService,
//     sys_role_menu_service::SysRoleMenuService, sys_role_user_service::SysRoleUserService,
// };
// use crate::APPLICATION_CONTEXT;
// use cassie_common::error::Result;
// use cassie_common::utils::string::IsEmpty;
// use cassie_domain::entity::sys_entitys::CommonField;
// use cassie_domain::{
//     dto::sys_role_dto::SysRoleDTO, entity::sys_entitys::SysRole, request::SysRoleQuery,
// };
// use rbatis::rbatis::Rbatis;

// /**
// *struct:SysRoleService
// *desc:角色基础服务
// *author:String
// *email:348040933@qq.com
// */
// pub struct SysRoleService {
//     pub sys_role_user_service: SysRoleUserService,
//     pub sys_role_menu_service: SysRoleMenuService,
//     pub sys_role_data_scope_service: SysRoleDataScopeService,
// }
// impl Default for SysRoleService {
//     fn default() -> Self {
//         SysRoleService {
//             sys_role_user_service: Default::default(),
//             sys_role_menu_service: Default::default(),
//             sys_role_data_scope_service: Default::default(),
//         }
//     }
// }
// impl SysRoleService {
//     //根据用户id获取角色
//     pub async fn role_info(&self, uid_id: i64) -> Result<Vec<i64>> {
//         let mut query = SysRoleQuery::default();
//         query.set_user_id(Some(uid_id));
//         let user_role = self.sys_role_user_service.list(&query).await;
//         if let Ok(list) = user_role {
//             Ok(list
//                 .iter()
//                 .map(|f| f.role_id.clone().unwrap_or_default())
//                 .collect::<Vec<i64>>())
//         } else {
//             Ok(Vec::<i64>::new())
//         }
//     }
//     //删除角色
//     pub async fn delete_by_role_id(&self, id: String) {
//         self.del(&id).await;
//         self.sys_role_menu_service
//             .delete_by_role_id(id.parse::<i64>().unwrap())
//             .await;
//     }
//     //保存角色
//     pub async fn save_role(&self, sys_role: SysRoleDTO) {
//         let menu_id_list = sys_role.menuid_list.clone();
//         let mut entity: SysRole = sys_role.into();
//         //保存或更新角色
//         let id = if let Some(id) = entity.id {
//             self.update_by_id(id.to_string(), &entity).await;
//             id
//         } else {
//             let role_id = self.save(&mut entity).await;
//             role_id.unwrap()
//         };
//         //保存角色菜单关系
//         self.sys_role_menu_service
//             .save_or_update(id, menu_id_list.clone())
//             .await;
//     }
// }

// impl CrudService<SysRole, SysRoleDTO, SysRoleQuery> for SysRoleService {
//     fn get_wrapper(arg: &SysRoleQuery) -> rbatis::wrapper::Wrapper {
//         let rb = APPLICATION_CONTEXT.get::<Rbatis>();
//         rb.new_wrapper().do_if(!arg.name().is_empty(), |w| {
//             w.like(SysRole::name(), &arg.name())
//         })
//     }
//     fn set_save_common_fields(&self, common: CommonField, data: &mut SysRole) {
//         data.id = common.id;
//         data.creator = common.creator;
//         data.create_date = common.create_date;
//     }
// }
