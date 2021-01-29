use crate::service::COM_SERVICE;
use actix_web::{web, Responder};
use crate::model::{Company, RestResult, PageForm};
use crate::utils::{gid, render, tool};
use serde_json::json;
use std::collections::HashMap;

pub async fn index() -> impl Responder {
    let data = json!({
        "title": "index"
    });
    render::to_html("company/index", &data)
}

pub async fn add(mut arg: web::Form<Company>) -> impl Responder {
    let mut rest = RestResult::<bool>::default();
    rest.code = 1;
    if arg.com_name.is_empty() {
        rest.msg = "资源名字不能为空!";
        return rest.to_json();
    }
    let exist_name = COM_SERVICE.exist_name(&arg.com_name).await;
    if exist_name.unwrap() == 1 {
        rest.msg = "资源名字已经存在!";
        return rest.to_json();
    }
    arg.id = Some(gid::new_id());
    let data = COM_SERVICE.add(&arg.0).await;
    RestResult::<bool>::from_db_add(&data, "添加成功", "添加失败").to_json()
}

pub async fn update(arg: web::Form<Company>) -> impl Responder {
    let mut rest = RestResult::<bool>::default();
    rest.code = 1;
    if arg.id.is_none() {
        rest.msg = "数据不合法!";
        return rest.to_json();
    }
    if arg.com_name.is_empty() {
        rest.msg = "资源名字不能为空!";
        return rest.to_json();
    }
    let id = arg.id.as_ref().unwrap();
    let org_data = COM_SERVICE.get(id).await;
    if org_data.is_err() {
        rest.msg = "数据已不存在!";
        return rest.to_json();
    }
    if org_data.unwrap().com_name.ne(&arg.com_name) {
        let exist_name = COM_SERVICE.exist_name(&arg.com_name).await;
        if exist_name.unwrap() == 1 {
            rest.msg = "资源名字已经存在!";
            return rest.to_json();
        }
    }
    let data = COM_SERVICE.update(&arg.0).await;
    RestResult::<bool>::from_db_update(&data, "修改成功", "修改失败").to_json()
}

pub async fn del(req_data: web::Form<HashMap<String, String>>) -> impl Responder {
    let id = &tool::get_opt_str(req_data.get("id"), "");
    let data = COM_SERVICE.del(id).await;
    RestResult::<bool>::from_db_del(&data, "删除成功", "删除失败").to_json()
}

pub async fn list(req_data: web::Form<PageForm>) -> impl Responder {
    let name = &tool::get_like_str(req_data.name.as_ref());
    let result = COM_SERVICE.list(&name.as_str(), req_data.page_index, req_data.page_size).await;
    render::to_json(&result.unwrap())
}
//上面方法等同
// pub async fn list2(arg: web::Form<PageForm>) -> impl Responder {
//     let req_data = arg.0;
//    let name = &tool::get_like_str(req_data.name.as_ref());
//     let result = COM_SERVICE.list(&name.as_str(), req_data.page_index, req_data.page_size).await;
//     render::to_json(&result.unwrap())
// }

pub async fn list_wrapper(req_data: web::Form<HashMap<String, String>>) -> impl Responder {
    // let req_data = arg.0;
    // let req_data = arg.deref();
    let name = &tool::get_like_str(req_data.get("name"));
    let page_index = tool::parse_uint(req_data.get("page_index"), "1");
    let page_size = tool::parse_uint(req_data.get("page_size"), "20");
    let result = COM_SERVICE.list_wrapper(name.as_str(), page_index, page_size).await;
    render::to_json(&result.unwrap())
}