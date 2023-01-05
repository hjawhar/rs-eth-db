extern crate diesel;

use bigdecimal::ToPrimitive;
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
    let enable_multhreading = false;
    if !enable_multhreading {
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
    }

    if enable_multhreading {
        let block_numbers = db1.get_blocks_numbers().await;
        let num = num_cpus::get();
        let blocks_per_core = current_block.to_i64().unwrap() / (num.to_i64().unwrap());
        println!("Blocks per core: {}", blocks_per_core);

        for n in 1..=num {
            let cb_clone = Arc::new(Database::new());
            let p_thread = provider.clone();
            let db_thread = cb_clone.clone();
            let start_block = (n.to_i64().unwrap() - 1) * blocks_per_core;
            let end_block = (n.to_i64().unwrap() * blocks_per_core) - 1;
            println!("Start block {} - End block {}", start_block, end_block);
            let block_numbers_mapped = block_numbers
                .iter()
                .map(|x| x.number.to_u64().unwrap())
                .collect::<Vec<u64>>();
            thread_handles.push(tokio::spawn(async move {
                for n in start_block..=end_block {
                    let block_to_fetch = n.to_u64().unwrap();
                    let position = &block_numbers_mapped
                        .clone()
                        .iter()
                        .position(|&r| r == block_to_fetch);
                    if let None = position {
                        if let Ok(block_res) = p_thread.get_block_with_txs(block_to_fetch).await {
                            if let Some(block_with_txs) = block_res {
                                db_thread.insert_block(&block_with_txs).await;
                            }
                        }
                    } else {
                        println!("Block already exists {}", block_to_fetch)
                    }
                }
            }));
        }
    }

    let _join_rs = join_all(thread_handles).await;
}
