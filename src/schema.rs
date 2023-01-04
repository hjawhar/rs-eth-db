// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (id) {
        id -> Int4,
        number -> Nullable<Int8>,
        timestamp -> Nullable<Int8>,
        hash -> Nullable<Varchar>,
    }
}
