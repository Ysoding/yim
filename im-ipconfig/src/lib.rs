mod domain;
mod source;

use im_config::AppConfig;

pub fn run_server(_config: &AppConfig) {
    source::init();
    source::mock::start_mock();
    domain::init();
}
