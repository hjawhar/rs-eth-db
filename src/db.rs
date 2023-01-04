use crate::*;
use diesel::result::Error;
use diesel::{connection::Connection, PgConnection};
use diesel::{prelude::*, *};
use dotenv::dotenv;
use std::env;

use models::Car;
use schema::{self, cars::dsl};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_tx(conn: &mut PgConnection) -> Result<usize, Error> {
    let result = insert_into(dsl::cars)
        .values((
            dsl::name.eq("ser_data.name.unwrap()"),
            dsl::model.eq("ser_data.model.unwrap()"),
        ))
        .execute(conn);
    //    .unwrap();
    result
}
