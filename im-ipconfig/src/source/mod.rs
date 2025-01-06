use im_common::discovery::{Discovery, EndpointInfo};
use std::sync::Arc;
use tokio::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex, OnceCell,
    },
    task,
};
pub(crate) mod mock;

pub(crate) static EVENT_CHAN: OnceCell<Mutex<Receiver<Event>>> = OnceCell::const_new();

#[derive(Debug)]
pub(crate) struct Event {
    pub ip: String,
    pub port: String,
    pub typ: EventType,
    pub connect_num: f64,
    pub message_bytes: f64,
}

impl Event {
    fn new(ed: &EndpointInfo) -> Result<Self, Box<dyn std::error::Error>> {
        if ed.metadata.is_none() {
            return Err("endpoint metadata should not be None".into());
        }

        let metadata = ed.metadata.as_ref().unwrap();

        let connect_num = metadata
            .get("connect_num")
            .and_then(|v| v.as_f64())
            .ok_or("endpoint metadata connect_num cannot be converted to f64")?;

        let message_bytes = metadata
            .get("message_bytes")
            .and_then(|v| v.as_f64())
            .ok_or("endpoint metadata message_bytes cannot be converted to f64")?;

        Ok(Event {
            ip: ed.ip.clone(),
            port: ed.port.clone(),
            typ: EventType::AddNodeEvent,
            connect_num,
            message_bytes,
        })
    }
}

#[derive(Debug)]
pub(crate) enum EventType {
    AddNodeEvent,
    DelNodeEvent,
}

pub(crate) fn init() {
    let (tx, rx) = mpsc::channel(100);
    EVENT_CHAN.set(Mutex::new(rx)).unwrap();
    task::spawn(data_handler(Arc::new(tx)));
}

async fn data_handler(event_tx: Arc<Sender<Event>>) {
    let dis = match Discovery::new(&["localhost:2379"], 10).await {
        Ok(dis) => dis,
        Err(e) => {
            log::error!("Failed to create discovery: {:?}", e);
            return;
        }
    };

    let set_event_tx = Arc::clone(&event_tx);
    let set_fn = move |_key: &str, value: &str| {
        let v = EndpointInfo::deserialize(value.as_bytes());
        if let Err(e) = v {
            log::error!("{:?}", e);
            return;
        }
        let ed = v.unwrap();

        let event_tx = Arc::clone(&set_event_tx);
        tokio::spawn(async move {
            let mut e = Event::new(&ed).unwrap();
            e.typ = EventType::AddNodeEvent;
            event_tx.send(e).await.unwrap();
        });
    };

    let del_event_tx = Arc::clone(&event_tx);
    let del_fn = move |_key: &str, value: &str| {
        let v = EndpointInfo::deserialize(value.as_bytes());
        if let Err(e) = v {
            log::error!("{:?}", e);
            return;
        }
        let ed = v.unwrap();

        let event_tx = Arc::clone(&del_event_tx);
        tokio::spawn(async move {
            let mut e = Event::new(&ed).unwrap();
            e.typ = EventType::DelNodeEvent;
            event_tx.send(e).await.unwrap();
        });
    };

    if let Err(e) = dis.watch("/yim/ip_dispatcher", set_fn, del_fn).await {
        log::error!("Failed to watch path: {:?}", e);
    }
}
