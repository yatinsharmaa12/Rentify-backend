// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        user_handle -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}
