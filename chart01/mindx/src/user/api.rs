///目前的接口提供给测试验证使用，后续需要按需调整
#[allow(unused)]
use std::sync::Arc;

use actix_web::{
    web::{self, Data},
    Responder,
};
use serde::{Deserialize, Serialize};

use crate::common::appdata::AppShareData;

use super::{
    model::{UserDo},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserVo {
    pub username: Arc<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageParams {
    like_username: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>,
    is_rev: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageResult<T> {
    size: usize,
    list: Vec<T>,
}

impl From<UserDo> for UserVo {
    fn from(value: UserDo) -> Self {
        Self {
            username: Arc::new(value.username),
            password: Some(value.password),
            nickname: Some(value.nickname),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResult<T> {
    pub success: bool,
    pub msg: Option<String>,
    pub data: Option<T>,
}
