use serde::{Deserialize, Serialize};

pub const SEESION_USER_NAME: &str = "SEESION_USER_NAME";

pub mod company;
pub mod app;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginUser {
    pub id: String,
    pub name: String,
}
