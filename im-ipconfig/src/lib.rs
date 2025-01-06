mod source;

use im_config::AppConfig;
use source::source_init;

pub async fn run_server(_config: &AppConfig) {
    source_init();
}
