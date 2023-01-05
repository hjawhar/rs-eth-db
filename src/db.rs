use crate::*;
use bigdecimal::BigDecimal;
use diesel::{connection::Connection, PgConnection};
use dotenv::dotenv;
use ethers::types::{Block, Transaction, H160};
use futures::lock::Mutex;
use std::{env, str::FromStr};
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

    pub async fn get_blocks_numbers(&self) -> Vec<models::BlockNumber> {
        let conn = &mut *(self.connection.lock().await);
        let blocks = sql_query("SELECT number from blocks")
            .load::<models::BlockNumber>(conn)
            .unwrap();

        blocks
    }

    pub async fn insert_block(&self, block: &Block<Transaction>) {
        use crate::schema::blocks::dsl::*;
        use diesel::insert_into;
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
            Ok(_) => {}
            Err(err) => {
                println!("Error adding block {} {}", err, block.number.unwrap());
            }
        }

        for tx in block.transactions.iter() {
            self.insert_tx(tx, conn).await;
        }
        println!(
            "Successfully added block: {} - total transactions: {} ",
            block.number.unwrap(),
            block.transactions.len()
        );
    }

    async fn insert_tx(&self, tx: &Transaction, conn: &mut PgConnection) {
        use crate::schema::transactions::dsl::*;
        use diesel::insert_into;
        let tx_value = BigDecimal::from_str(&tx.value.to_string())
            .unwrap()
            .as_bigint_and_exponent();
        let transaction_value = (
            hash.eq(tx.hash.to_string()),
            value.eq(tx_value.1),
            position.eq(tx
                .transaction_index
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()),
            sender.eq(tx.from.to_string()),
            receiver.eq(tx
                .to
                .unwrap_or(H160::from_str("0x0000000000000000000000000000000000000000").unwrap())
                .to_string()),
            input.eq(tx.input.to_string()),
        );

        let result = insert_into(transactions)
            .values(transaction_value)
            .execute(conn);
        match result {
            Ok(_) => {
                // println!("Successfully added transaction {}", tx.hash);
            }
            Err(err) => {
                println!("Error adding transaction {} {}", err, tx.hash);
            }
        }
    }
}
