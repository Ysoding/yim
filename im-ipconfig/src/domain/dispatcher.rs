use std::collections::HashMap;

use super::endpoint::Endpoint;

pub struct Dispatcher {
    pub candidate_map: HashMap<String, Endpoint>,
}
