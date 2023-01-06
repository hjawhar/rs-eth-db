// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (number) {
        number -> Int8,
        timestamp -> Int8,
        hash -> Varchar,
    }
}

diesel::table! {
    transactions (hash, block_number) {
        hash -> Varchar,
        value -> Numeric,
        position -> Int4,
        sender -> Varchar,
        receiver -> Varchar,
        input -> Varchar,
        block_number -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blocks,
    transactions,
);
