use serde_json::json;
use rbatis::core::Result;
use crate::db::BASE_DB;
use crate::model::Company;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::core::db::DBExecResult;
use crate::utils::yml;
use std::collections::HashMap;

lazy_static! {
    static ref SQL_MAP:HashMap<String,String> = yml::read_sql("company");
}
fn load_str(key: &str) -> &str {
    let sql = SQL_MAP.get(key);
    sql.unwrap().as_str()
}


pub struct ComService {}


impl ComService {
    pub async fn exist_name(&self, name: &String) -> Result<u64> {
        let exist_name_sql = SQL_MAP.get("exist_name");
        BASE_DB.py_fetch("", exist_name_sql.unwrap().as_str(), &json!({"name": name })).await
    }
    pub async fn get(&self, id: &String) -> Result<Company> {
        BASE_DB.fetch_by_id("", id).await
    }
    pub async fn del(&self, id: &String) -> Result<u64> {
        BASE_DB.remove_by_id::<Company>("", id).await
    }

    pub async fn add(&self, arg: &Company) -> Result<DBExecResult> {
        BASE_DB.save("", arg).await
    }
    pub async fn update(&self, arg: &Company) -> Result<DBExecResult> {
        let update_sql = SQL_MAP.get("update");
        BASE_DB.py_exec("", update_sql.unwrap().as_str(), arg).await
    }

    pub async fn list(&self, name: &str, page_index: u64, page_size: u64) -> Result<Page<Company>> {
        let list_sql = load_str("list");
        let param = &json!({"name": name });
        BASE_DB.py_fetch_page("", list_sql, param, &PageRequest::new(page_index, page_size)).await
    }
    pub async fn list_wrapper(&self, name: &str, page_index: u64, page_size: u64) -> Result<Page<Company>> {
        let wrapper = BASE_DB.new_wrapper().eq("com_name", name);
        BASE_DB.fetch_page_by_wrapper("", &wrapper, &PageRequest::new(page_index, page_size)).await
    }
}