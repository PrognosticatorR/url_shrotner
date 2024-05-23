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
