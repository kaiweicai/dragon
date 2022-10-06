use crate::service::api::user_service::{UserService, WechatUserService};
use crate::service::asi::asi_service::AsiGroupService;
// use crate::service::cache_service::CacheService;
use crate::service::event_service::EventConfigService;
use crate::service::log::log_service::{LogLoginService, LogOperationService};
use crate::service::sys_auth_service::SysAuthService;
use crate::service::sys_dict_service::{SysDictDataService, SysDictTypeService};
use crate::service::sys_menu_service::SysMenuService;
use crate::service::sys_params_service::SysParamsService;
// use crate::service::sys_role_service::SysRoleService;
// use crate::service::sys_user_service::SysUserService;
use crate::APPLICATION_CONTEXT;
use cassie_config::config::WebApplicationConfig;
// use cassie_storage::upload::upload_service::UploadService;
use log::info;

pub async fn init_service() {
    let config = APPLICATION_CONTEXT.get::<WebApplicationConfig>();
    // APPLICATION_CONTEXT.set::<CacheService>(CacheService::new().unwrap());
    info!("CacheService init success!");
    APPLICATION_CONTEXT.set::<SysAuthService>(SysAuthService::default());
    // info!("SysUserService init success!");
    // APPLICATION_CONTEXT.set::<SysUserService>(SysUserService::default());
    // info!("SysRoleService init success!");
    // APPLICATION_CONTEXT.set::<SysRoleService>(SysRoleService::default());
    info!("SysMenuService init success!");
    APPLICATION_CONTEXT.set::<SysMenuService>(SysMenuService::default());
    info!("SysMenuService init success!");
    APPLICATION_CONTEXT.set::<SysParamsService>(SysParamsService::default());
    info!("SysParamsService init success!");
    APPLICATION_CONTEXT.set::<SysDictTypeService>(SysDictTypeService::default());
    info!("SysDictTypeService init success!");
    APPLICATION_CONTEXT.set::<SysDictDataService>(SysDictDataService::default());
    info!("SysDictDataService init success!");
    APPLICATION_CONTEXT.set::<AsiGroupService>(AsiGroupService::default());
    info!("AsiGroupService init success!");
    // APPLICATION_CONTEXT.set::<UploadService>(UploadService::new(config).unwrap());
    APPLICATION_CONTEXT.set::<LogLoginService>(LogLoginService::default());
    info!("LogLoginService init success!");
    APPLICATION_CONTEXT.set::<LogOperationService>(LogOperationService::default());
    info!("LogOperationService init success!");
    APPLICATION_CONTEXT.set::<EventConfigService>(EventConfigService {});
    info!("EventConfigService init success!");
    //apis  用户服务
    APPLICATION_CONTEXT.set::<UserService>(UserService {});
    info!("UserService init success!");
    APPLICATION_CONTEXT.set::<WechatUserService>(WechatUserService {});
    info!("WechatUserService init success!");
}
