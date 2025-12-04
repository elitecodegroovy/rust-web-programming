#![allow(dead_code, unused_imports)]

use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ApiResultOld<T>
where
    T: Sized + Default,
{
    pub data: Option<T>,
    pub success: bool,
    pub code: Option<String>,
    pub msg: Option<String>,
}

impl<T> ApiResultOld<T>
where
    T: Sized + Default,
{
    pub fn success(data: Option<T>) -> Self {
        Self {
            data,
            success: true,
            code: None,
            msg: None,
        }
    }

    pub fn error(code: String, msg: Option<String>) -> Self {
        Self {
            data: None,
            success: false,
            code: Some(code),
            msg,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PageResultOld<T> {
    pub size: usize,
    pub list: Vec<T>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AR<T>
where
    T: Sized + Default,
{
    pub data: Option<T>,
    pub success: bool,
    pub code: u32,
    pub msg: Option<String>,
}

impl<T> AR<T>
where
    T: Sized + Default
{
    pub fn success(data: Option<T>) -> Self {
        Self {
            data,
            success: true,
            code: 200,
            msg: Option::from("操作成功!".to_string()),
        }
    }

    pub fn success_flag(data: Option<T>, success: bool) -> Self {
        Self {
            data,
            success,
            code: 200,
            msg: Option::from("操作成功!".to_string()),
        }
    }

    pub fn error(code: u32, msg: Option<String>) -> Self {
        Self {
            code,
            success: false,
            msg,
            data: None,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PR<T> {
    pub total_count: usize,
    pub list: Vec<T>,
}


#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    pub total_count: usize,
    pub list: Vec<T>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct UserSession {
    pub username: Arc<String>,
    pub nickname: Option<String>,
    pub roles: Vec<Arc<String>>,
    pub extend_infos: HashMap<String, String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TokenSession {
    pub username: Arc<String>,
    pub roles: Vec<Arc<String>>,
    pub extend_infos: HashMap<String, String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ApiResult<T>
where
    T: Sized + Default,
{
    pub data: Option<T>,
    pub success: bool,
    pub code: Option<String>,
    pub message: Option<String>,
}

impl<T> crate::common::model::ApiResult<T>
where
    T: Sized + Default,
{
    pub fn success(data: Option<T>) -> Self {
        Self {
            data,
            success: true,
            code: None,
            message: None,
        }
    }

    pub fn error(code: String, message: Option<String>) -> Self {
        Self {
            data: None,
            success: false,
            code: Some(code),
            message,
        }
    }
}
