use crate::observe::event::CassieEvent;
use crate::service::fire_event;
use crate::APPLICATION_CONTEXT;
use cassie_common::error::Error;
use cassie_common::error::Result;
use cassie_common::utils::password_encoder::PasswordEncoder;
use cassie_config::config::WebApplicationConfig;
use cassie_domain::dto::sign_in::SignInDTO;
use cassie_domain::dto::sys_log::SysLogLoginDto;
use cassie_domain::entity::sys_entitys::SysUser;
use cassie_domain::vo::jwt::JWTToken;
use cassie_domain::vo::sign_in::SignInVO;
use rbatis::rbatis::Rbatis;

use rbson::DateTime;
const REDIS_KEY_RETRY: &'static str = "login:login_retry";
/**
*struct:SysAuthService
*desc:用户权限服务  登录 错误重试
*author:String
*email:348040933@qq.com
*/
pub struct SysAuthService {}

impl Default for SysAuthService {
    fn default() -> Self {
        SysAuthService {}
    }
}

impl SysAuthService {
    /**sign_in
     *method:
     *desc:登陆后台
     *author:String
     *email:348040933@qq.com
     */
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        /*验证码 验证*/
        let mut rb = APPLICATION_CONTEXT.get::<Rbatis>();
        // let user: Option<SysUser> = rb
        //     .fetch_by_wrapper(rb.new_wrapper().eq(SysUser::username(), &arg.username()))
        //     .await?;
        let user: Option<SysUser> =
            SysUser::select_by_username(&mut rb, arg.username().clone().unwrap()).await?;
        let user = user.ok_or_else(|| {
            Error::from(format!(
                "账号:{} 不存在!",
                arg.username().clone().unwrap_or_default()
            ))
        })?;
        if user.status.eq(&Some(0)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;
        if !PasswordEncoder::verify(
            user.password
                .as_ref()
                .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            arg.password()
                .as_ref()
                .ok_or_else(|| Error::from("密码不能为空"))?,
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        if error.is_some() {
            return Err(error.unwrap());
        }
        let sign_in_vo = self.get_user_info(&user).await?;
        let mut login_log = SysLogLoginDto::default();
        login_log.set_operation(Some("0".to_string()));
        login_log.set_user_agent(Some("admin".to_string()));
        login_log.set_creator_name(arg.username().clone());
        login_log.set_creator(user.id.clone());
        let event = CassieEvent::LogLogin(login_log);
        fire_event(event).await;
        return Ok(sign_in_vo);
    }

    /**
     *method:get_user_info_by_token
     *desc:根据token获取 暂时没用到
     *author:String
     *email:348040933@qq.com
     */
    pub async fn get_user_info_by_token(&self, token: &JWTToken) -> Result<SignInVO> {
        // let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        // let user: Option<SysUser> = rb
        //     .fetch_by_wrapper(rb.new_wrapper().eq(SysUser::id(), &token.id()))
        //     .await?;
        // let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", token.username())))?;
        // return self.get_user_info(&user).await;
        todo!()
    }
    /**
     *method:get_user_info
     *desc:获取用户信息
     *author:String
     *email:348040933@qq.com
     */
    pub async fn get_user_info(&self, user: &SysUser) -> Result<SignInVO> {
        let cassie_config = APPLICATION_CONTEXT.get::<WebApplicationConfig>();
        //去除密码，增加安全性
        let mut user = user.clone();
        user.password = None;
        let agency_code = user.agency_code.clone();
        let user_id = user
            .id
            .clone()
            .ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;
        let mut sign_vo = SignInVO::default();
        sign_vo.set_user(Some(user.clone().into()));
        //提前查找所有权限，避免在各个函数方法中重复查找
        let mut jwt_token = JWTToken::default();
        jwt_token.set_id(user_id);
        jwt_token.set_super_admin(user.super_admin.clone().unwrap_or_default());
        jwt_token.set_from(Default::default());
        jwt_token.set_username(user.username.clone().unwrap_or_default());
        jwt_token.set_agency_code(agency_code.clone().unwrap_or_default());
        jwt_token.set_exp(DateTime::now().timestamp_millis() as usize);
        sign_vo.set_access_token(jwt_token.create_token(cassie_config.jwt_secret())?);
        return Ok(sign_vo);
    }
}
