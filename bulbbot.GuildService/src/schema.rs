// @generated automatically by Diesel CLI.

diesel::table! {
    guilds (id) {
        id -> Int8,
        premium -> Bool,
        developer -> Bool,
    }
}

diesel::table! {
    logging (guilds_id) {
        guilds_id -> Int8,
        mod_action -> Nullable<Int8>,
        auto_mod -> Nullable<Int8>,
        message -> Nullable<Int8>,
        role -> Nullable<Int8>,
        member -> Nullable<Int8>,
        channel -> Nullable<Int8>,
        thread -> Nullable<Int8>,
        join_leave -> Nullable<Int8>,
        invite -> Nullable<Int8>,
        banpool -> Nullable<Int8>,
        other -> Nullable<Int8>,
    }
}

diesel::joinable!(logging -> guilds (guilds_id));

diesel::allow_tables_to_appear_in_same_query!(
    guilds,
    logging,
);
