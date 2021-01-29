use rbatis::rbatis::Rbatis;
use rbatis::core::db::DBPoolOptions;
use crate::config::BOOT_CFG;
use std::time::Duration;

lazy_static! {
    pub static ref BASE_DB:Rbatis= Rbatis::new();
}
pub async fn init() {
    let mut opt = DBPoolOptions::new();
    opt.connect_timeout = Duration::from_secs(30);
    let _ = BASE_DB.link_opt(BOOT_CFG.mysql.as_str(), &opt).await;
}