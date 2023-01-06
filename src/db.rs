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

    // pub async fn get_transactions(&self) -> Vec<models::DbTransaction> {
    //     let conn = &mut *(self.connection.lock().await);
    //     use crate::schema::transactions::dsl::*;
    //     let txs = transactions.load::<models::DbTransaction>(conn).unwrap();
    //     txs
    // }

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
        let block_hash = format!("{:?}", block.hash.unwrap());
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
        let tx_block_number = tx.block_number.unwrap().to_string().parse::<i64>().unwrap();
        let tx_hash = format!("{:#x}", tx.hash);
        let tx_sender = format!("{:#x}", tx.from);
        let tx_receiver: String;
        if let Some(_receiver) = tx.to {
            tx_receiver = format!("{:#x}", _receiver);
        } else {
            tx_receiver = format!(
                "{:#x}",
                H160::from_str("0x0000000000000000000000000000000000000000").unwrap()
            );
        }

        use crate::schema::transactions::dsl::*;
        use diesel::insert_into;
        let tx_value = BigDecimal::from_str(&tx.value.to_string()).unwrap();
        let transaction_value = (
            hash.eq(tx_hash),
            value.eq(tx_value),
            position.eq(tx
                .transaction_index
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()),
            sender.eq(tx_sender),
            receiver.eq(tx_receiver),
            input.eq(tx.input.to_string()),
            block_number.eq(tx_block_number),
        );
        let result = insert_into(transactions)
            .values(transaction_value)
            .execute(conn);
        match result {
            Ok(_) => {}
            Err(err) => {
                println!(
                    "Error adding transaction {} {} {}",
                    err,
                    tx.hash,
                    tx.block_number.unwrap()
                );
            }
        }
    }
}
