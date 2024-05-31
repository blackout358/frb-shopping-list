use std::{collections, os::unix::net::SocketAddr, str::FromStr};

use actix_web::App;
// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use axum::{extract::{Path, State}, response::IntoResponse, routing::{get, post}, Extension, Router};
use bson::{doc, oid::ObjectId, Document};
use futures::TryStreamExt;
use mongodb::{Client, Collection};
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    collection: Collection<Document>,
}

// impl AppState {
//     pub fn new(mut self, value: Collection<Document>) -> Self {
//         self.collection = value;
//         self
//     }
// }
  
// async fn old_create_item(share_collection: web::Data<AppState>, path: web::Path<String>) -> impl Responder { 
//     let new_item = doc! {"name": path.to_string()};
//     let data = share_collection.collection.lock().unwrap();
//     let res = data.insert_one(new_item, None).await.unwrap();
//     let res_string = format!("{:#?}", res);
//     HttpResponse::Ok().body(res_string)
// }
 
async fn create_item(Path(id): Path<String>, State(state): State<AppState>) -> String{ 
    let new_item = doc! {"name": id};
    let data = state.collection.clone();
    // let data = share_collection.lock().unwrap();
    // let data = share_collection.collection.lock().unwrap();
    let res = data.insert_one(new_item, None).await.unwrap();
    format!("{:#?}", res)
    // HttpResponse::Ok().body(res_string)
}

async fn read_item(State(state): State<AppState>) -> String {
    println!("Inside GET call");
    // let data = share_collection.collection.lock().unwrap();
    let coll = state.collection.clone();
    let res = coll.find(doc!{}, None).await.unwrap();
    let result_vector:Vec<String> = Vec::new();
    let contents = parse_document(res, result_vector).await;
    // let contents = String::from("asd");
    // HttpResponse::Ok().body(contents)
    contents
}

async fn parse_document(mut res: mongodb::Cursor<Document>, mut result_vector: Vec<String>) -> String {
    while let Ok(Some(doc)) = res.try_next().await {
        let res_str = format!("{:#?}", doc); 
        result_vector.push(res_str); 
    }
    let contents = result_vector.join("\n"); 
    contents
}

// async fn update_item() -> impl Responder {
//     HttpResponse::Ok().body("Update item")
// }

async fn delete_item(Path(id): Path<String>, State(state): State<AppState>) -> String {
    println!("Inside DELETE call");
    // let data = share_collection.collection.lock().unwrap();
    let data = state.collection.clone();
    println!("Passed in value: {}", &id);

    
    let mut _contents= String::new();
    if let Ok(document_id)  = ObjectId::from_str(&id.to_string()){ 
        let delete_result = data.delete_one(doc!{"_id": document_id},None).await.unwrap(); 
        _contents = format!("{:#?}", delete_result);
    
    } else {
        _contents = "Invalid Document ID".to_string();
    }
    println!("{_contents}");
    _contents
    // HttpResponse::Ok().body(format!("{}\n", _contents)) 
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let item_collection:Collection<Document> = connect_to_db().await.unwrap();

//     let share_collection = web::Data::new(AppState {
//         collection: Mutex::new(item_collection),
//     });
//     HttpServer::new(move || {
//         App::new()
//         .app_data(share_collection.clone())
//         // Create
//         .route("/items/{id}", web::post().to(create_item))
//         // Read
//         .route("/items/", web::get().to(read_item))
//         // Update
//         .route("/items/{id}", web::put().to(update_item))
//         // Delete
//         .route("/items/{id}", web::delete().to(delete_item))
//     })
//     .bind("172.19.1.128:7878")?
//     .run()
//     .await
// }
#[tokio::main]
async fn main() {
    let item_collection:Collection<Document> = connect_to_db().await.unwrap();

    let state = AppState {collection: item_collection.clone()};

    // let share_collection = web::Data::new(AppState {
    //     collection: Mutex::new(item_collection),
    // });

    // let share_collection: Mutex<Collection<Document>> = Mutex::new(item_collection);
    // let share_collection = AppState {collection: Mutex::new(item_collection)};
    let app = Router::new()
    .route("/items/", get(read_item))
    
    .route("/items/:id", post(create_item).delete(delete_item))
    // .layer(Extension(share_collection));
    .with_state(state);

    // let addr = SocketAddr::from(([172,19,1,128], 7878));
    // println!("Listening on {:?}", &addr);
    // axum::Server::bind(&addr).serve
    let listener = tokio::net::TcpListener::bind("172.19.1.128:7878").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> String {
    "Hello, world".to_string()
}

async fn connect_to_db() -> mongodb::error::Result<Collection<Document>>  {
    // println!("We in here");
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await?;
    let db = client.database("shopping_list");
   let item_collection = db.collection("items"); 
    Ok(item_collection)
}