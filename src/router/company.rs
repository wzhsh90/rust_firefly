use actix_web::{web};
use crate::controller::com_controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/company")
            .route("/index", web::get().to(com_controller::index))
            .route("/add", web::post().to(com_controller::add))
            .route("/update", web::post().to(com_controller::update))
            .route("/del", web::post().to(com_controller::del))
            .route("/list", web::post().to(com_controller::list))
            .route("/list_wrapper", web::post().to(com_controller::list_wrapper))
    );
}