use std::collections::HashMap;

use crate::source::Event;

use super::{endport::Endport, stat::Stat};

#[derive(Default, Debug)]
pub(crate) struct Dispatcher {
    candidate_map: HashMap<String, Endport>,
}

impl Dispatcher {
    pub(crate) async fn dispatch(&mut self) -> Vec<Endport> {
        let mut res: Vec<&mut Endport> = self.candidate_map.values_mut().collect();
        for e in &mut res {
            e.update_score().await;
        }

        res.sort_by(|a, b| {
            b.active_score
                .partial_cmp(&a.active_score)
                .unwrap_or(std::cmp::Ordering::Equal) // Handle NaN gracefully
                .then_with(|| {
                    b.static_score
                        .partial_cmp(&a.static_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        res.into_iter().map(|e| e.clone()).collect()
    }

    pub(crate) async fn add_node_event(&mut self, event: Event) {
        let key = event.key();

        if !self.candidate_map.contains_key(&key) {
            let ep = Endport::new(event.ip, event.port);
            self.candidate_map.insert(key.clone(), ep);
        }

        let ep = self.candidate_map.get_mut(&key).unwrap();
        ep.update_stat(Stat {
            connect_num: event.connect_num,
            message_bytes: event.message_bytes,
        })
        .await;
    }

    pub(crate) async fn del_node_event(&mut self, event: Event) {
        let key = event.key();
        self.candidate_map.remove(&key);
    }
}
