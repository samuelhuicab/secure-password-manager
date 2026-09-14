use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ItemType {
    Server,
    Database,
    Email,
    Website,
    Api,
    License,
    Note,
    Env,
}