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
    let p2 = provider.clone();
    let db1 = db.clone();
    let db2 = db.clone();

    let last_synced_block = db1.get_last_block().await;
    let mut last_synced_block_res = 1_u64;
    let blocks_count = db1.get_last_block_count().await;
    if let Some(last_block_res) = last_synced_block {
        println!("Last fetched block: {}", last_block_res.number);
        last_synced_block_res = last_block_res.number.to_string().parse::<u64>().unwrap();
    }
    println!("Blocks fetched: {}", blocks_count);
    let current_block = (p1.provider().get_block_number().await).unwrap().as_u64();
    thread_handles.push(tokio::spawn(async move {
        let l1 = p1;
        let mut stream = l1.provider().subscribe_blocks().await.unwrap();
        while let Some(block) = stream.next().await {
            println!("Current block number: {}", block.number.unwrap());
        }
    }));

    thread_handles.push(tokio::spawn(async move {
        let l2 = p2;
        for n in last_synced_block_res..current_block {
            if let Ok(block_res) = l2.get_block_with_txs(n).await {
                if let Some(block_with_txs) = block_res {
                    db2.insert_block(&block_with_txs).await;
                }
            }
        }
    }));

    let _join_rs = join_all(thread_handles).await;
}
