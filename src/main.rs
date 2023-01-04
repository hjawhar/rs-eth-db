#[macro_use]
extern crate diesel;

use futures::future::join_all;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use diesel::prelude::*;
use diesel::sql_query;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::providers::Ws;
use tokio::task::JoinHandle;
pub mod db;
pub mod models;
pub mod schema;
use crate::models::Count;
use db::Database;
use dotenv::dotenv;
use ethers::providers::StreamExt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = Arc::new(Database::new());
    let mut thread_handles: Vec<JoinHandle<()>> = vec![];
    let node_ws = env::var("WSS_NODE").expect("WSS Node endpoint is missing");
    let url = String::from(node_ws);
    let ws = Ws::connect(url).await.unwrap();
    let provider = Arc::new(Provider::new(ws).interval(Duration::from_millis(2000)));
    let p1 = provider.clone();
    let db1 = db.clone();

    // THIS IS FOR TESTING
    db1.simple_query().await;

    thread_handles.push(tokio::spawn(async move {
        let l1 = p1;
        let mut stream = l1.provider().subscribe_blocks().await.unwrap();
        while let Some(block) = stream.next().await {
            db1.insert_block(&block);
        }
    }));

    let join_rs = join_all(thread_handles).await;
}
