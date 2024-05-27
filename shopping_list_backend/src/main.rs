use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use futures::TryStreamExt;
use mongodb::{bson::{doc, Document}, options::{ClientOptions, Collation}, Client, Collection, Database};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("172.19.1.128:7878").unwrap();
    let uri = "mongodb://localhost:27017";
    // let db = connect_to_db();
    // let client = Client::with_uri_str(uri).await?;
    // let db = client.database("shopping_list");
    // let coll:Collection<Document> = db.collection("items");
    // let res = coll.find
    // let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    
    
    for stream in listener.incoming() {
        // connect_to_db();
        let stream = stream.unwrap();

        let _res = handle_connection(stream).await;
    }
}

async fn connect_to_db() -> mongodb::error::Result<()>  {
    println!("We in here");
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await?;
    let db = client.database("shopping_list");
    let coll:Collection<Document> = db.collection("items");
    let mut res = coll.find(doc!{"name": "Jeff"}, None).await?;
    while let Some(doc) = res.try_next().await? {
        println!("{:?}", doc);
    }
    Ok(())
}



async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let _res = connect_to_db().await;
        // println!("{:?}", _res);
        let status_line = "HTTP/1.1 200 OK";
        let contents = "Hello";
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        println!("{:?}", request_line);

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
    }
}

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();

//     println!("Request: {:#?}", http_request);
//     let response  = "Hello";
//     stream.write_all(response.as_bytes());
// }