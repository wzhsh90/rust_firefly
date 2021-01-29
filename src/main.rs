use actix_web::{App, HttpServer};
use rust_firefly::config::BOOT_CFG;
use rust_firefly::router::{company, app};
use rust_firefly::middleware::session;
// use actix_web::middleware::Logger;
use actix_session::{CookieSession};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    rust_firefly::config::logger::init();
    rust_firefly::db::init().await;
    //路由
    HttpServer::new(|| {
        //checklogin 要在session 前面
        App::new()
            .wrap(session::CheckLogin)
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // .wrap(Logger::default())
            .configure(app::init)
            .configure(company::init)
    })
        .bind(&BOOT_CFG.server)?
        .run()
        .await
}
