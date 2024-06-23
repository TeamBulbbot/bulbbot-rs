// @generated automatically by Diesel CLI.

diesel::table! {
    messages (message_id) {
        message_id -> Int8,
        guild_id -> Int8,
        channel_id -> Int8,
        author_id -> Int8,
        #[max_length = 4000]
        content -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}
