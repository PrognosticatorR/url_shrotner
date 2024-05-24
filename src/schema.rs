// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int8,
        #[max_length = 2048]
        origin_url -> Varchar,
        #[max_length = 256]
        short_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_url_mappings (id) {
        id -> Int8,
        user_id -> Int4,
        url_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 128]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        created_at -> Timestamp,
        deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(user_url_mappings -> urls (url_id));
diesel::joinable!(user_url_mappings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    urls,
    user_url_mappings,
    users,
);
