use actix_web::HttpResponse;
use rbatis::core::Error;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use rbatis::core::db::DBExecResult;

#[crud_enable(table_name: sys_company_t)]
#[derive(Debug, Clone)]
pub struct Company {
    pub id: Option<String>,
    pub com_desc: Option<String>,
    pub com_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageForm {
    pub name: Option<String>,
    pub page_index: u64,
    pub page_size: u64,
}

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestResult<'a, T> {
    pub code: i64,
    pub msg: &'a str,
    pub data: Option<T>,
}

impl<T> RestResult<'_, T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_db_add(
        arg: &Result<DBExecResult, Error>,
        success: &'static str,
        fail: &'static str,
    ) -> Self {
        let mut rest = Self::default();
        rest.code = 1;
        if arg.is_ok() {
            let add_e = arg.as_ref().unwrap().rows_affected;
            rest.msg = if add_e == 1 {
                rest.code = 0;
                success
            } else {
                fail
            }
        } else {
            rest.msg = fail;
        }
        return rest;
    }
    pub fn from_db_del(
        arg: &Result<u64, Error>,
        success: &'static str,
        fail: &'static str,
    ) -> Self {
        let mut rest = Self::default();
        rest.code = 1;
        if arg.is_ok() {
            let add_e = *arg.as_ref().unwrap();
            rest.msg = if add_e == 1 {
                rest.code = 0;
                success
            } else {
                fail
            }
        } else {
            rest.msg = fail;
        }
        return rest;
    }
    pub fn from_db_update(
        arg: &Result<DBExecResult, Error>,
        success: &'static str,
        fail: &'static str,
    ) -> Self {
        let mut rest = Self::default();
        rest.code = 1;
        if arg.is_ok() {
            rest.code = 0;
            rest.msg = success;
        } else {
            rest.msg = fail;
        }
        return rest;
    }
    pub fn from_msg(code: i64, info: &'static str) -> Self {
        let mut rest = Self::default();
        rest.msg = info;
        return rest;
    }
    pub fn to_json(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl<T> ToString for RestResult<'_, T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl<T> Default for RestResult<'_, T> {
    fn default() -> Self {
        Self {
            code: 0,
            msg: "",
            data: None,
        }
    }
}