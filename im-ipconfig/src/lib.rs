mod source;

use im_config::AppConfig;
use source::source_init;

pub fn run_server(config: &AppConfig) {
    source_init();
    println!("ipconfig");
    println!("{:?}", config);
}
