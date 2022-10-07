use crate::entity::log::{SysLogLogin, SysLogOperation};
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default, PartialEq)]
#[getset(get = "pub", set = "pub")]
pub struct SysLogLoginDto {
    id: Option<i64>,
    operation: Option<String>,
    user_agent: Option<String>,
    ip: Option<String>,
    creator_name: Option<String>,
    creator: Option<i64>,
    create_date: Option<DateTime>,
}

impl From<SysLogLogin> for SysLogLoginDto {
    fn from(arg: SysLogLogin) -> Self {
        Self {
            id: arg.id,
            operation: arg.operation,
            user_agent: arg.user_agent,
            ip: arg.ip,
            creator_name: arg.creator_name,
            creator: arg.creator,
            create_date: arg.create_date,
        }
    }
}

impl Into<SysLogLogin> for SysLogLoginDto {
    fn into(self) -> SysLogLogin {
        SysLogLogin {
            id: self.id,
            operation: self.operation,
            user_agent: self.user_agent,
            ip: self.ip,
            creator_name: self.creator_name,
            creator: self.creator,
            create_date: self.create_date,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default, PartialEq)]
#[getset(get = "pub", set = "pub")]
pub struct SysLogOperationDto {
    id: Option<i64>,
    operation: Option<String>,
    request_uri: Option<String>,
    ip: Option<String>,
    creator_name: Option<String>,
    request_params: Option<String>,
    request_method: Option<String>,
    request_time: Option<String>,
    status: Option<i8>,
    creator: Option<i64>,
    create_date: Option<DateTime>,
}
impl Into<SysLogOperation> for SysLogOperationDto {
    fn into(self) -> SysLogOperation {
        SysLogOperation {
            id: self.id,
            operation: self.operation,
            request_uri: self.request_uri,
            ip: self.ip,
            creator_name: self.creator_name,
            request_params: self.request_params,
            request_method: self.request_method,
            request_time: self.request_time,
            status: self.status,
            creator: self.creator,
            create_date: self.create_date,
        }
    }
}
impl From<SysLogOperation> for SysLogOperationDto {
    fn from(arg: SysLogOperation) -> Self {
        Self {
            id: arg.id,
            operation: arg.operation,
            request_uri: arg.request_uri,
            ip: arg.ip,
            creator_name: arg.creator_name,
            request_params: arg.request_params,
            request_method: arg.request_method,
            request_time: arg.request_time,
            status: arg.status,
            creator: arg.creator,
            create_date: arg.create_date,
        }
    }
}
