use std::str::FromStr;

// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use bson::{doc, oid::ObjectId, Document};
use futures::TryStreamExt;
use mongodb::{Client, Collection};
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    collection: Collection<Document>,
}
async fn create_item(Path(id): Path<String>, State(state): State<AppState>) -> String {
    let new_item = doc! {"name": id};
    let data = state.collection.clone();
    let res = data.insert_one(new_item, None).await.unwrap();
    format!("{:#?}", res)
}

async fn read_item(State(state): State<AppState>) -> String {
    println!("Inside GET call");
    let coll = state.collection.clone();
    let res = coll.find(doc! {}, None).await.unwrap();
    let contents = parse_document(res).await;
    println!("Response: \n{}", contents);
    contents
}

async fn parse_document(mut res: mongodb::Cursor<Document>) -> String {
    let mut result_vector: Vec<String> = Vec::new();
    while let Ok(Some(doc)) = res.try_next().await {
        // println!("{:?}", &doc);
        // let res_str = format!("{}", doc);
        let res_str = serde_json::to_string(&doc).expect("Error with seriailse document");
        // println!("{:?}", res_str);
        result_vector.push(res_str);
    }

    // let contents = result_vector.join("\n");
    let mut contents = result_vector.join("\n");
    // contents.push(']');
    // contents.insert_str(0, "[");
    // match contents {
    //     Ok(contents) => contents,
    //     Err(err) => format!("{err}"),
    // }
    contents
}

async fn delete_item(Path(id): Path<String>, State(state): State<AppState>) -> String {
    println!("Inside DELETE call");
    let data = state.collection.clone();
    println!("Passed in value: {}", &id);
    let mut _contents = String::new();
    if let Ok(document_id) = ObjectId::from_str(&id.to_string()) {
        let delete_result = data
            .delete_one(doc! {"_id": document_id}, None)
            .await
            .unwrap();
        _contents = format!("{:#?}", delete_result);
    } else {
        _contents = "Invalid Document ID".to_string();
    }
    println!("{_contents}");
    _contents
}

#[tokio::main]
async fn main() {
    let item_collection: Collection<Document> = connect_to_db().await.unwrap();

    let state = AppState {
        collection: item_collection.clone(),
    };
    let app = Router::new()
        .route("/items/", get(read_item))
        .route("/items/:id", post(create_item).delete(delete_item))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("172.19.1.128:7878")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn connect_to_db() -> mongodb::error::Result<Collection<Document>> {
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await?;
    let db = client.database("shopping_list");
    let item_collection = db.collection("items");
    Ok(item_collection)
}
