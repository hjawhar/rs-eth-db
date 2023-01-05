// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (number) {
        number -> Int8,
        timestamp -> Int8,
        hash -> Varchar,
    }
}

diesel::table! {
    transactions (hash) {
        hash -> Varchar,
        value -> Int8,
        position -> Int4,
        sender -> Varchar,
        receiver -> Nullable<Varchar>,
        input -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blocks,
    transactions,
);
