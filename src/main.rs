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

    let last_block = db1.get_last_block().await;
    let blocks_count = db1.get_last_block_count().await;
    if let Some(last_block_res) = last_block {
        println!("Last fetched block: {}", last_block_res.number);
    }
    println!("Blocks fetched: {}", blocks_count);

    thread_handles.push(tokio::spawn(async move {
        let l1 = p1;
        let mut stream = l1.provider().subscribe_blocks().await.unwrap();
        while let Some(block) = stream.next().await {
            if let Some(block_number) = block.number {
                if let Ok(block_res) = provider.get_block_with_txs(block_number).await {
                    if let Some(block_with_txs) = block_res {
                        db1.insert_block(&block_with_txs).await;
                    }
                }
            }
        }
    }));

    let _join_rs = join_all(thread_handles).await;
}
