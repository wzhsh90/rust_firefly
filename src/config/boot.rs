use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};
use std::env;

///服务启动配置
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BootConfig {
    pub debug: bool,
    ///当前服务地址
    pub server: String,
    ///日志路径
    pub log_path: String,
    ///mysql地址
    pub mysql: String,
}

///默认配置
impl Default for BootConfig {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut profile = "dev";
        if args.len() == 2 {
            profile = args[1].as_str()
        }
        let mut yml_data = String::new();
        let file_path = format!("{}{}{}", "resource/", &profile, ".yml");
        let expect_msg = format!("{}{}", file_path.clone(), " not exist!");
        File::open(file_path)
            .expect(expect_msg.as_str())
            .read_to_string(&mut yml_data);
        let cfg: BootConfig = serde_yaml::from_str(yml_data.as_str()).expect("config yml deserialize error");
        return cfg;
    }
}