use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    str,
    str::FromStr,
};

// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use axum::body::BoxBody;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{self, Response},
    middleware::{self, Next},
    routing::{get, post, Route},
    Router,
};

// use axum_server::tls_rustls::RustlsConfig;
use axum_helmet::{CrossOriginEmbedderPolicy, CrossOriginOpenerPolicy, Helmet, HelmetLayer};
use bson::{doc, oid::ObjectId, ser, Document};
use futures::TryStreamExt;
use http::{header::HeaderName, HeaderValue, Method, Request};
use hyper::client::HttpConnector;
use hyper::Client as HTTP_Client;
use mongodb::{Client, Collection};
use native_tls::{Identity, TlsAcceptor};
use tokio_rustls::rustls::{self, server::NoClientAuth, ServerConfig};
use tower::ServiceExt;

use std::io::BufReader;
// use reqwest::Method;
// use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::ServeDir;
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    services::ServeFile,
};
// use tower_http::trace::TraceLayer;
// use tower_http::BoxError:
// use tower::service_fn::
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
    let contents = result_vector.join("\n");
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

async fn load_webapp() -> Result<reqwest::Response, reqwest::Error> {
    let request = reqwest::get("http://127.0.0.1:9800/webapp").await;
    println!("{:?}", request);
    // let response_builder = ;
    match request {
        Ok(content) => Ok(content),
        Err(err) => Err(err),
    }
    // Ok(request.unwrap())
}

async fn hello_world() -> String {
    "Hello world from rust".to_string()
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
}

fn app_service() -> ServeDir {
    let serve_dir = ServeDir::new("../shopping_list_frontend/build/web");

    serve_dir
}

#[tokio::main]
async fn main() {
    // Define the routes and handlers
    let item_collection: Collection<Document> = connect_to_db().await.unwrap();

    let state = AppState {
        collection: item_collection.clone(),
    };
    let _cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let app: Router = Router::new()
        .route("/", get(hello_world))
        .route("/items", get(read_item))
        .route("/items/:id", post(create_item).delete(delete_item))
        .nest_service("/webapp", app_service())
        // .nest_service("/testapp", test_app)
        .layer(cors_layer())
        .layer(HelmetLayer::new(
            Helmet::new()
                .add(CrossOriginOpenerPolicy::SameOrigin)
                .add(CrossOriginEmbedderPolicy::RequireCorp),
        ))
        // .layer(header_layer())
        // .layer(TraceLayer::new_for_http())
        .with_state(state);
    // Load the TLS configuration
    // let config = match axum_server::tls_rustls::RustlsConfig::from_pem_file(
    //     "certs/cert.pem",
    //     "certs/key.pem",
    // )

    let config = match axum_server::tls_rustls::RustlsConfig::from_pem_file(
        "certs/172.19.1.128.pem",
        "certs/172.19.1.128-key.pem",
    )
    .await
    {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading TLS config: {}", e);
            return;
        }
    };

    // Define the address to listen on
    // let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    let addr = SocketAddr::from(([172, 19, 1, 128], 7878));
    println!("Listening on https://{}", addr);

    // Start the server with Rustls
    if let Err(e) = axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("Server error: {}", e);
    }
}

async fn connect_to_db() -> mongodb::error::Result<Collection<Document>> {
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await?;
    let db = client.database("shopping_list");
    let item_collection = db.collection("items");
    Ok(item_collection)
}
