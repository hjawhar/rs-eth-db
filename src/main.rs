#[macro_use]
extern crate diesel;

use std::env; 
use std::sync::Arc;
use std::time::Duration;

use diesel::dsl;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sql_query;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::providers::Ws;
 use tokio::sync::futures;
use tokio::task::JoinHandle;
 pub mod db;
pub mod models;
pub mod schema;
use crate::db::insert_tx;
use crate::models::Count;
use ethers::{
    providers::{ StreamExt },
    types::H256,
};

#[tokio::main]
async fn main() {
    let mut thread_handles: Vec<JoinHandle<()>> = vec![];
    let conn = &mut db::establish_connection();
    let node_ws = env::var("WSS_NODE").expect("WSS Node endpoint is missing");
    let url = String::from(node_ws);
    let ws = Ws::connect(url).await.unwrap();
    let provider = Arc::new(Provider::new(ws).interval(Duration::from_millis(2000)));
    let p1 = provider.clone();
    let p2 = provider.clone();

    let result = insert_tx(conn);
    match result {
        Ok(r) => {
            println!("Successfully inserted tx");
        }
        Err(err) => {
            println!("Inserting inserting tx: {}", err);
        }
    }
    let users = sql_query("SELECT COUNT(*) from cars")
        .load::<Count>(conn)
        .unwrap();

    println!("{:?}", users);
    // Ok(())
    thread_handles.push(tokio::spawn(async move {
        let l1 = p1;
        let mut stream = l1.provider().subscribe_pending_txs().await.unwrap();
        while let Some(tx_hash) = stream.next().await {
            println!("{}",tx_hash);
        }
    })); 

}
