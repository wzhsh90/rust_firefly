mod boot;
pub mod logger;
use boot::BootConfig;

lazy_static!{
    pub static ref BOOT_CFG:BootConfig=BootConfig::default();
}
