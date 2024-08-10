// @generated automatically by Diesel CLI.

diesel::table! {
    bills (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        customer -> Varchar,
        #[max_length = 255]
        paid_amount -> Varchar,
        #[max_length = 255]
        rewarded_amount -> Nullable<Varchar>,
        #[max_length = 255]
        paid_txhash -> Varchar,
        #[max_length = 255]
        rewarded_txhash -> Nullable<Varchar>,
        paid_at -> Timestamp,
        rewarded_at -> Nullable<Timestamp>,
        status -> Int2,
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

diesel::allow_tables_to_appear_in_same_query!(bills, states,);
