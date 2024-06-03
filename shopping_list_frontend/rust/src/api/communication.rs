use std::fmt::format;

use flutter_rust_bridge::frb;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use crate::api::item_model::{Item, Oid};

pub async fn get_items() -> Vec<Item> {
    let res = reqwest::get("http://172.19.1.128:7878/items/").await;
    println!("Respone: {:?}", res);
    let mut items: Vec<Item> = Vec::new();
    match res {
        Ok(res) => {
            println!("we inside");
            let _temp = (res.text()).await;
            match _temp {
                Ok(data) => {
                    println!("{data}");
                    let parts = data.split("\n");

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
                        oid: "Error".to_string(),
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
                    oid: "Error".to_string(),
                },
                name: err.to_string(),
            });
            items
        }
    }
}

pub async fn delete_item(id: String) {
    let client = reqwest::Client::new();
    let res = client
        .delete(format!("http://172.19.1.128:7878/items/{}", id))
        .send()
        .await;
    println!("{:?}", res);
}
