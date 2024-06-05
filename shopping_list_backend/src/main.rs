use std::str::FromStr;

// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{self, Response},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_helmet::{
    CrossOriginEmbedderPolicy, CrossOriginOpenerPolicy, CrossOriginResourcePolicy, Helmet,
    HelmetLayer,
};
use bson::{doc, oid::ObjectId, Document};
use futures::TryStreamExt;
use http::{
    header::{self, HeaderName, AUTHORIZATION, CONTENT_TYPE},
    HeaderMap, HeaderValue, Method, Request,
};
use mongodb::{Client, Collection};
use reqwest::header::ACCESS_CONTROL_ALLOW_HEADERS;
use tower::ServiceBuilder;
use tower_http::set_header::SetResponseHeaderLayer;
// use reqwest::Method;
// use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::services::ServeDir;
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

async fn hello_world() -> &'static str {
    "Test"
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
    // .expose_headers(vec![
    //     HeaderName::try_from("Cross-Origin-Opener-Policy").unwrap(),
    //     HeaderName::try_from("Cross-Origin-Embedder-Policy").unwrap(),
    // ])
}

// fn header_layer() -> HeaderMap {
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         "Cross-Origin-Opener-Policy",
//         HeaderValue::from_static("same-origin"),
//     );
//     headers.insert(
//         "Cross-Origin-Opener-Policy",
//         HeaderValue::from_static("require-corp"),
//     );
//     headers
// }

// async fn header_layer(request: Request, next: Next) -> Response {
//     let mut response = next.run(request).await;
//     response.headers_mut().insert(
//         header::CACHE_CONTROL,
//         HeaderValue::from_static("public, max-age=3600"),
//     );
//     response
// }

async fn set_static_cache_control(request: Request<Body>, next: Next) -> Response<Body> {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        HeaderName::from_static("cross-origin-opener-policy"),
        HeaderValue::from_static("same-origin"),
    );
    response
}

fn app_service() -> ServeDir {
    println!("Sending web app");
    let serve_dir = ServeDir::new("../shopping_list_frontend/build/web");

    serve_dir
}

#[tokio::main]
async fn main() {
    let item_collection: Collection<Document> = connect_to_db().await.unwrap();

    let state = AppState {
        collection: item_collection.clone(),
    };

    let helmet = HelmetLayer::new(
        Helmet::new()
            .add(CrossOriginOpenerPolicy::same_origin())
            .add(CrossOriginEmbedderPolicy::require_corp()),
    );
    let he = axum_helmet::Helmet::default();
    let _cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(Any)
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/items", get(read_item))
        .route("/items/:id", post(create_item).delete(delete_item))
        // .layer(he)
        .nest_service("/webapp", app_service())
        .layer(cors_layer())
        .layer(HelmetLayer::new(
            Helmet::new()
                .add(CrossOriginOpenerPolicy::same_origin())
                .add(CrossOriginEmbedderPolicy::require_corp()), // .add(CrossOriginResourcePolicy::cross_origin()),
        ))
        // .layer(header_layer())
        // .layer(TraceLayer::new_for_http())
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
