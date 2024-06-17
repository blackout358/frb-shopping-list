use std::fmt::format;

use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use crate::api::item_model::{Item, Oid};

pub struct CommunicationRemote {
    client: reqwest::Client,
}

impl CommunicationRemote {
    #[frb(sync)]
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        CommunicationRemote {
            client: reqwest::Client::builder().build().unwrap(),
        }
    }
    #[frb(sync)]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        CommunicationRemote {
            client: reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        }
    }

    pub async fn get_items(&self) -> Vec<Item> {
        // let client = reqwest::Client::builder()
        //     // .danger_accept_invalid_certs(true)
        //     .build()
        //     .unwrap();
        let res = self
            .client
            .get("https://172.19.1.128:7878/items")
            .send()
            .await;
        println!("Respone: {:?}", res);
        let mut items: Vec<Item> = Vec::new();
        // items
        match res {
            Ok(res) => {
                println!("we inside");
                let _temp = (res.text()).await;
                match _temp {
                    Ok(data) => {
                        println!("{data}");
                        let parts = data.split("\n");

                        // let collection = parts;

                        for part in parts {
                            match serde_json::from_str::<Item>(&part) {
                                Ok(value) => items.push(value),
                                Err(_) => println!("ERROR PARSING DOCUMENT: {}", &part),
                            }
                        }
                        println!("{:?}", items);
                    }
                    Err(err) => items.push(Item {
                        _id: Oid {
                            oid: "Result Error".to_string(),
                        },
                        name: err.to_string(),
                    }),
                }
                items
            }
            Err(err) => {
                println!("{err}");
                items.push(Item {
                    _id: Oid {
                        oid: "REQUEST ERROR".to_string(),
                    },
                    name: err.to_string(),
                });
                items
            }
        }
    }
    pub async fn delete_item(&self, id: String) {
        let res = self
            .client
            .delete(format!("https://172.19.1.128:7878/items/{}", id))
            .send()
            .await;
        println!("{:?}\n", res);
    }
    pub async fn add_item(&self, name: String) {
        let res = self
            .client
            .post(format!("https://172.19.1.128:7878/items/{}", name))
            .send()
            .await;
        println!("{:?}", res);
    }
    pub async fn update_item(&self, id: String, new_name: String) {
        let res = self
            .client
            .put(format!(
                "https://172.19.1.128:7878/items/{}/{}",
                id, new_name
            ))
            .send()
            .await;
        println!("{:?}", res);
    }
}
