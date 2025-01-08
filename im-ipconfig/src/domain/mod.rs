use dispatcher::Dispatcher;
use tokio::sync::{Mutex, OnceCell};

use crate::source;

mod dispatcher;
mod endport;
mod stat;
mod window;

pub(crate) static DP: OnceCell<Mutex<Dispatcher>> = OnceCell::const_new();

pub(crate) fn init() {
    DP.set(Mutex::new(Dispatcher::default())).unwrap();

    tokio::spawn(async move {
        let mut rx = source::EVENT_CHAN.get().unwrap().lock().await;
        while let Some(event) = rx.recv().await {
            let mut dp = DP.get().unwrap().lock().await;
            match event.typ {
                source::EventType::AddNodeEvent => dp.add_node_event(event).await,
                source::EventType::DelNodeEvent => dp.del_node_event(event).await,
            }
        }
    });
}
