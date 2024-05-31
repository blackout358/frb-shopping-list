// use std::{
//     io::{prelude::*, BufReader},
//     net::{TcpListener, TcpStream},
// };
// use futures::TryStreamExt;
// use mongodb::{bson::{doc, Document}, options::{ClientOptions, Collation}, Client, Collection, Database};
// use bson::{bson, Bson};
// use serde_json::json;

// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("172.19.1.128:7878").unwrap();
//     let mut item_collection: Collection<Document> = connect_to_db().await.unwrap();
//     // let uri = "mongodb://localhost:27017";
//     // let db = connect_to_db();
//     // let client = Client::with_uri_str(uri).await?;
//     // let db = client.database("shopping_list");
//     // let coll:Collection<Document> = db.collection("items");
//     // let res = coll.find
//     // let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    
    
//     for stream in listener.incoming() {
//         // connect_to_db();
//         let stream = stream.unwrap();

//         let _res = handle_connection(stream).await;
//     }
// }




// async fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();
//     println!("{request_line}");

//     match request_line {
//         "GET / HTTP/1.1". => println!("asd")
//         _ => println!("error")
//     }
//     if request_line == "GET / HTTP/1.1" {
    //         let _res = connect_to_db().await.unwrap();
    //         println!("{:#?}", _res);
    //         let status_line = "HTTP/1.1 200 OK"; 
    //         let contents = serde_json::to_string_pretty(&_res).unwrap();
//         let length = contents.len();

//         let response = format!(
    //             "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    //         ); 
    
    //         stream.write_all(response.as_bytes()).unwrap();
    //     } else {
        //         // some other request
        //     }
        // }
        
        // // fn handle_connection(mut stream: TcpStream) {
            // //     let buf_reader = BufReader::new(&mut stream);
// //     let http_request: Vec<_> = buf_reader
// //         .lines()
// //         .map(|result| result.unwrap())
// //         .take_while(|line| !line.is_empty())
// //         .collect();

// //     println!("Request: {:#?}", http_request);
// //     let response  = "Hello";
// //     stream.write_all(response.as_bytes());
// // }


use std::{str::FromStr, sync::Mutex};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use bson::{doc, oid::ObjectId, Document};
use futures::TryStreamExt;
use mongodb::{Client, Collection};

struct AppState {
    collection: Mutex<Collection<Document>>, // You can store any type, String is just an example
}
 

// struct QueryPat

// impl AppState {
//     fn new() -> Self {}
// }

// Handler functions
async fn create_item(share_collection: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    
    // let new_item = Item::new(path);
    let new_item = doc! {"name": path.to_string()};
    let data = share_collection.collection.lock().unwrap();
    
    let res = data.insert_one(new_item, None).await.unwrap();
    let res_string = format!("{:#?}", res);
    HttpResponse::Ok().body(res_string)
}

async fn read_item(share_collection: web::Data<AppState>) -> impl Responder {
    println!("Inside GET call");
    let data = share_collection.collection.lock().unwrap();
    let res = data.find(doc!{}, None).await.unwrap();


    
    let result_vector:Vec<String> = Vec::new();
    let contents = parse_document(res, result_vector).await;
    HttpResponse::Ok().body(contents)
}

async fn parse_document(mut res: mongodb::Cursor<Document>, mut result_vector: Vec<String>) -> String {
    while let Ok(Some(doc)) = res.try_next().await {
        let res_asd = format!("{:#?}", doc);
        // let json_string = serde_json::to_string(&doc).unwrap(); 
        result_vector.push(res_asd); 
    }

    let contents = result_vector.join("\n");

    
    // let  contents = serde_json::to_string(&result_vector).unwrap();
    // contents.replace("\\", "")
    // let res_string = format!("{:#?}", contents);
    // res_string
    contents
    
}

async fn update_item() -> impl Responder {
    HttpResponse::Ok().body("Update item")
}

async fn delete_item(share_collection: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    println!("Inside DELETE call");
    let data = share_collection.collection.lock().unwrap();
    println!("Passed in value: {}", &path);

    
    let mut _contents= String::new();
    if let Ok(document_id)  = ObjectId::from_str(&path.to_string()){

        // let res = data.find(doc! {"_id": document_id}, None).await.unwrap();
        let delete_result = data.delete_one(doc!{"_id": document_id},None).await.unwrap();
        // let result_vector = Vec::new();
        // contents = parse_document(res, result_vector).await;
        // println!("{}", parsedelete_result);
        _contents = format!("{:#?}", delete_result);
    
    } else {
        _contents = "Invalid Document ID".to_string();
    }
    println!("{_contents}");
    
    HttpResponse::Ok().body(format!("{}\n", _contents))
    // println!("{res}");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let item_collection:Collection<Document> = connect_to_db().await.unwrap();

    let share_collection = web::Data::new(AppState {
        collection: Mutex::new(item_collection),
    });
    HttpServer::new(move || {
        App::new()
        .app_data(share_collection.clone())
        // Create
        .route("/items/{id}", web::post().to(create_item))
        // Read
        .route("/items", web::get().to(read_item))
        // Update
        .route("/items/{id}", web::put().to(update_item))
        // Delete
        .route("/items/{id}", web::delete().to(delete_item))
    })
    .bind("172.19.1.128:7878")?
    .run()
    .await
}

async fn connect_to_db() -> mongodb::error::Result<Collection<Document>>  {
    // println!("We in here");
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await?;
    let db = client.database("shopping_list");
   let item_collection = db.collection("items");
    // let mut res = item_collection.find(doc!{}, None).await?;
    // let mut result_vector:Vec<String> = Vec::new();
    // while let Some(doc) = res.try_next().await? {
    //     let json_string = serde_json::to_string(&doc).unwrap();

    //     // let string:Bson = doc.clone().into();
    //     result_vector.push(json_string);
    //     // result_vector.push(json_string.clone());
    //     // println!("{}", json_string);
    // }
    // Ok(result_vector)
    Ok(item_collection)
}