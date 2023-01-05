// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (number) {
        number -> Int8,
        timestamp -> Int8,
        hash -> Varchar,
    }
}
