use serde::{Deserialize, Serialize};
use serde_json::{self, json};

pub async fn get_items() -> String {
    let res = reqwest::get("http://172.19.1.128:7878/items/").await;
    println!("Respone: {:?}", res);
    match res {
        Ok(res) => {
            println!("we inside");
            // let mut body = String::new();
            // res.read_to_string(&mut body);
            let _temp = (res.text()).await;
            match _temp {
                Ok(data) => {
                    let ser_data = json!(data);
                    println!("{:#}", ser_data);
                    return data;
                }
                Err(err) => return err.to_string(),
            }
        }
        Err(err) => {
            println!("{err}");
            "Nothing".to_string()
        }
    }
    // "Test".to_string()
}
