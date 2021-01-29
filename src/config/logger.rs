use fast_log::appender::{LogAppender, FastLogFormatRecord};
use fast_log::filter::ModuleFilter;
use fast_log::plugin::console::ConsoleAppender;
use fast_log::plugin::file_split::{FileSplitAppender, RollingType};

use crate::config::BOOT_CFG;
use fast_log::consts::LogSize;

pub fn init() {
    //自定义日志追加器
    let mut appender_vec: Vec<Box<dyn LogAppender>> = vec![
        Box::new(FileSplitAppender::new(BOOT_CFG.log_path.as_str(), LogSize::MB(100), RollingType::KeepNum(5), true, 100))
    ];
    if BOOT_CFG.debug {
        appender_vec.push(Box::new(ConsoleAppender {}));
    }
    //自定义日志过滤
    fast_log::init_custom_log(appender_vec,
                              1000,
                              log::Level::Info,
                              Box::new(ModuleFilter::new_exclude(vec!["sqlx".to_string()])), Box::new(FastLogFormatRecord {}),
    );
}


