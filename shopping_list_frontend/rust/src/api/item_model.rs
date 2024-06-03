use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

#[derive(Debug, Serialize, Deserialize)]
pub struct Oid {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub _id: Oid,
    pub name: String,
}
