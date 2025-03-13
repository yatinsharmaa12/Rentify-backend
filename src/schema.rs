// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        price -> Int4,
        image_url -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_handle -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::joinable!(products -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
);
