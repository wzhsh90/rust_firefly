use actix_web::{web, Responder};
use serde_json::json;
use actix_files::Files;
use actix_session::{Session};
use crate::router::SEESION_USER_NAME;
use crate::router::LoginUser;
use crate::utils::render;
use std::collections::HashMap;
use crate::model::RestResult;


async fn home() -> impl Responder {
    let data = json!({
        "title": "系统"
    });
    render::to_html("home", &data)
}

async fn login() -> impl Responder {
    let data = json!({
        "title": "账号登录"
    });
    render::to_html("login", &data)
}

async fn login_api(session: Session, arg: web::Form<HashMap<String, String>>) -> impl Responder {
    let mut rest = RestResult::<String>::default();
    rest.code = 1;
    if arg.get("account").is_none() {
        rest.msg = "账号不能为空!";
        return rest.to_json();
    }
    if arg.get("password").is_none() {
        rest.msg = "密码不能为空!";
        return rest.to_json();
    }
    let user_info = LoginUser {
        id: arg.get("account").unwrap().to_string(),
        name: arg.get("password").unwrap().to_string(),
    };
    session.set(SEESION_USER_NAME, user_info);
    rest.code = 0;
    rest.msg = "登录成功";
    rest.data = Some("/home".to_string());
    rest.to_json()
}

async fn logoff(session: Session) -> impl Responder {
    let mut rest = RestResult::<bool>::default();
    session.remove(SEESION_USER_NAME);
    rest.msg = "注销成功";
    rest.to_json()
}

async fn favicon() -> impl Responder {
    // actix_files::NamedFile::open("static/img/favicon.ico")
    ""
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(Files::new("/static", "static/"))

        .route("/", web::get().to(login))
        .route("/home", web::get().to(home))
        .route("/favicon", web::get().to(favicon))
        .route("/login", web::get().to(login))
        .route("/login_api", web::post().to(login_api))
        .route("/logff", web::post().to(logoff))

    ;
}