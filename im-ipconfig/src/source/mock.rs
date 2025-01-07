use std::time::Duration;

use im_common::discovery::{EndpointInfo, Register};
use tokio::time;

pub(crate) fn start_mock() {
    let ports = ["6969", "6968", "6967"];
    let nodes = ["node1", "node2", "node3"];

    for (port, node) in ports.iter().zip(nodes.iter()) {
        let port = (*port).to_string();
        let node = (*node).to_string();
        tokio::spawn(test_register(port, node));
    }
}

async fn test_register(port: String, node: String) {
    let path = format!("/yim/ip_dispatcher/{node}");
    let mut ed = EndpointInfo {
        ip: "127.0.0.1".to_string(),
        port: port.clone(),
        metadata: Some(serde_json::json!({
            "connect_num": rand::random::<f64>() * 696_969.0,
            "message_bytes": rand::random::<f64>() * 69_696_969.0,
        })),
    };

    let mut r = match Register::new(&["localhost:2379"], 10, &path, &ed, 5000).await {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to create register: {:?}", e);
            return;
        }
    };

    r.listen_lease_keep_alive();

    loop {
        ed.metadata = Some(serde_json::json!({
            "connect_num": rand::random::<f64>() * 696_969.0,
            "message_bytes": rand::random::<f64>() * 69_696_969.0,
        }));
        if let Err(e) = r.update_value(&ed).await {
            log::error!("Failed to update value: {:?}", e);
        }
        time::sleep(Duration::from_secs(1)).await;
    }
}
