use std::collections::HashMap;

use crate::source::Event;

use super::endport::Endport;

#[derive(Default, Debug)]
pub(crate) struct Dispatcher {
    candidate_map: HashMap<String, Endport>,
}

impl Dispatcher {
    pub(crate) fn dispatch(&mut self) -> Vec<&Endport> {
        let res: Vec<&mut Endport> = self.candidate_map.values_mut().collect();

        vec![]
    }
    pub(crate) async fn add_node_event(&mut self, event: Event) {
        let key = event.key();

        if !self.candidate_map.contains_key(&key) {
            let ep = Endport::new(event.ip, event.port);
            self.candidate_map.insert(key.clone(), ep);
        }

        let ep = self.candidate_map.get_mut(&key).unwrap();
        ep.update_stat(super::stat::Stat {
            connect_num: event.connect_num,
            message_bytes: event.message_bytes,
        })
        .await;
    }

    pub(crate) fn del_node_event(&mut self, event: Event) {
        let key = event.key();
        self.candidate_map.remove(&key);
    }
}
