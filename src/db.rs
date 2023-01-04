use crate::*;
use diesel::result::Error;
use diesel::sql_types::{BigInt, Int8};
use diesel::{connection::Connection, PgConnection};
use diesel::{prelude::*, *};
use dotenv::dotenv;
use ethers::types::{Block, H256, U64};
use futures::lock::Mutex;
use models::DBBlock;
use schema::{self, blocks::dsl};
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

    pub async fn simple_query(&self) {
        let conn = &mut *(self.connection.lock().await);
        let users = sql_query("SELECT COUNT(*) from blocks")
            .load::<Count>(conn)
            .unwrap();

        println!("{:?}", users);
    }

    pub fn insert_block(&self, block: &Block<H256>) {
        println!("{:#?}", block.number.unwrap());
        //
        // Implement Logic
        //
        //-> Result<usize, Error>
        // let conn = &mut *(self.connection);
        // let result = insert_into(dsl::blocks)
        //     .values((
        //         dsl::number.eq(BigInt::from(0)), //block.number.unwrap_or(U64::from(0))
        //         dsl::hash.eq(block.hash.unwrap_or("")),
        //         dsl::timestamp.eq(block.timestamp),
        //     ))
        //     .execute(conn);
        // //    .unwrap();
        // result
    }
}
