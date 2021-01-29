mod com_service;

use com_service::ComService;

lazy_static! {
    pub static ref COM_SERVICE:ComService = ComService{};
}