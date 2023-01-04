// @generated automatically by Diesel CLI.

diesel::table! {
    cars (id) {
        id -> Int4,
        name -> Varchar,
        model -> Varchar,
    }
}
