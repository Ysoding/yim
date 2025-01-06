mod source;

use im_config::AppConfig;

pub async fn run_server(_config: &AppConfig) {
    source::init();
    source::mock::start_mock();
    let mut rx = source::EVENT_CHAN.get().unwrap().lock().await;
    while let Some(resp) = rx.recv().await {
        log::info!("{:?}", resp);
    }
}
