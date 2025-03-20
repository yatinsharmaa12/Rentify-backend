// @generated automatically by Diesel CLI.

diesel::table! {
    cart (id) {
        id -> Int4,
        user_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
        added_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        description -> Text,
        price -> Int4,
        image_url -> Text,
        created_at -> Timestamp,
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

diesel::joinable!(cart -> products (product_id));
diesel::joinable!(cart -> users (user_id));
diesel::joinable!(products -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart,
    products,
    users,
);
