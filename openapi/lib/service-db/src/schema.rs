// @generated automatically by Diesel CLI.

diesel::table! {
    events_erc20 (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 500]
        payload -> Varchar,
        #[max_length = 255]
        txhash -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        #[max_length = 255]
        method -> Varchar,
        #[max_length = 255]
        collection -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
        amount -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    events_erc721 (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 500]
        payload -> Varchar,
        #[max_length = 255]
        txhash -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        #[max_length = 255]
        method -> Varchar,
        #[max_length = 255]
        collection -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
        token_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    service_collection (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 255]
        namespace -> Varchar,
        status -> Int4,
        start_block_number -> Int4,
        #[max_length = 255]
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    service_token (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 255]
        namespace -> Varchar,
        status -> Int4,
        start_block_number -> Int4,
        #[max_length = 255]
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    service_webhood_collection (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
        #[max_length = 255]
        endpoint_url -> Varchar,
        #[max_length = 255]
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    service_webhood_token (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
        #[max_length = 255]
        endpoint_url -> Varchar,
        #[max_length = 255]
        to_transfer -> Varchar,
        #[max_length = 255]
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    services (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
        #[max_length = 255]
        info -> Varchar,
        #[max_length = 255]
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    states (key) {
        #[max_length = 255]
        key -> Varchar,
        #[max_length = 255]
        value -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    events_erc20,
    events_erc721,
    service_collection,
    service_token,
    service_webhood_collection,
    service_webhood_token,
    services,
    states,
);
