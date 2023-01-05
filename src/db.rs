use crate::*;
use diesel::{connection::Connection, PgConnection};
use dotenv::dotenv;
use ethers::types::{Block, Transaction};
use futures::lock::Mutex;
use std::env;
pub struct Database {
    pub connection: Arc<Mutex<PgConnection>>,
}

impl Database {
    pub fn new() -> Database {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Database {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub async fn get_last_block_count(&self) -> i64 {
        let conn = &mut *(self.connection.lock().await);
        use crate::schema::blocks::dsl::*;
        let count = blocks.count().get_result(conn).unwrap();
        count
    }

    pub async fn get_blocks_count_raw(&self) -> Vec<Count> {
        let conn = &mut *(self.connection.lock().await);
        let count = sql_query("SELECT COUNT(*) from blocks")
            .load::<Count>(conn)
            .unwrap();
        count
    }

    pub async fn get_last_block(&self) -> Option<models::Block> {
        let conn = &mut *(self.connection.lock().await);
        use crate::schema::blocks::dsl::*;
        let block = blocks
            .order_by(number.desc())
            .first(conn)
            .optional()
            .unwrap();
        block
    }

    pub async fn get_blocks(&self) -> Vec<models::Block> {
        let conn = &mut *(self.connection.lock().await);
        let blocks = sql_query("SELECT * from blocks")
            .load::<models::Block>(conn)
            .unwrap();

        blocks
    }

    pub async fn insert_block(&self, block: &Block<Transaction>) {
        use crate::schema::blocks::dsl::*;
        use diesel::insert_into;
        println!(
            "Block number: {} - Total transactions: {}",
            block.number.unwrap(),
            block.transactions.len()
        );
        let conn = &mut *(self.connection.lock().await);
        let block_number = block.number.unwrap().to_string().parse::<i64>().unwrap();
        let block_hash = block.hash.unwrap().to_string();
        let block_timestamp = block.timestamp.to_string().parse::<i64>().unwrap();
        let block_value = (
            number.eq(block_number),
            hash.eq(block_hash),
            timestamp.eq(block_timestamp),
        );
        let result = insert_into(blocks).values(block_value).execute(conn);
        match result {
            Ok(_) => {
                println!("Successfully added block {}", block.number.unwrap());
            }
            Err(err) => {
                println!("Error adding block {} {}", err, block.number.unwrap());
            }
        }

        for tx in block.transactions.iter() {
            println!("{}", tx.hash);
        }
    }
}
