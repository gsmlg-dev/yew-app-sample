use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "session")]
pub struct ConfigStore {
    pub name: String,
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self {
            name: "yew app sample".to_string(),
        }
    }
}
