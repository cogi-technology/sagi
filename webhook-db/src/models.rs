use {
    chrono::{Local, NaiveDateTime},
    diesel::{deserialize::FromSqlRow, expression::AsExpression, prelude::*, sql_types::*},
    std::fmt::Debug,
};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, FromPrimitive, ToPrimitive,
)]
#[diesel(sql_type = SmallInt)]
pub enum Status {
    NotFound = 0,
    Paid = 1,
    RewardingWaitForTxConfirmed,
    Rewarded,
    SomethingWrong = 500,
    AlreadyPaid,
    AlreadyRewardingWaitForTxConfirmed,
    AlreadyRewarded,
}

pub fn to_error_status(status: Status) -> Status {
    if status == Status::Paid {
        Status::AlreadyPaid
    } else if status == Status::RewardingWaitForTxConfirmed {
        Status::AlreadyRewardingWaitForTxConfirmed
    } else if status == Status::Rewarded {
        Status::AlreadyRewarded
    } else if status == Status::NotFound {
        Status::NotFound
    } else {
        Status::SomethingWrong
    }
}
#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = crate::schema::states)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct State {
    pub key: String,
    pub value: String,
}

// Services
#[derive(Debug, thiserror::Error)]
#[error("ServiceError: {msg}, {status}")]
pub struct ServiceError {
    pub msg: String,
    pub status: i32,
}

impl ServiceError {
    pub fn not_found(msg: String) -> Self {
        Self {
            msg,
            status: Status::NotFound as i32,
        }
    }
    pub fn something_wrong(msg: String) -> Self {
        Self {
            msg,
            status: Status::SomethingWrong as i32,
        }
    }
}

impl From<diesel::result::Error> for ServiceError {
    fn from(error: diesel::result::Error) -> Self {
        if error == diesel::result::Error::NotFound {
            ServiceError::not_found(error.to_string())
        } else {
            ServiceError::something_wrong(error.to_string())
        }
    }
}
#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = crate::schema::services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Service {
    pub id: String,
    pub client_id: String,
    pub info: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            id: Default::default(),
            client_id: "".into(),
            info: "".into(),
            created_by: "".into(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = crate::schema::service_webhook_collection)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ServiceWebhookCollection {
    pub id: String,
    pub client_id: String,
    pub endpoint_url: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for ServiceWebhookCollection {
    fn default() -> Self {
        Self {
            id: Default::default(),
            client_id: "".into(),
            endpoint_url: "".into(),
            created_by: "".into(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::service_collection)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ServiceCollection {
    pub id: String,
    pub client_id: String,
    pub address: String,
    pub namespace: String,
    pub start_block_number: i32,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for ServiceCollection {
    fn default() -> Self {
        Self {
            id: Default::default(),
            client_id: "".into(),
            address: "".into(),
            namespace: "".into(),
            start_block_number: 1,
            created_by: "".into(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}

pub enum StatusEvent {
    Sent,
    Init,
    SentError,
}

impl StatusEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusEvent::Sent => "Sent",
            StatusEvent::Init => "Init",
            StatusEvent::SentError => "SentError",
        }
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = crate::schema::events_erc721)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EventErc721 {
    pub id: String,
    pub payload: String,
    pub txhash: String,
    pub status: String,
    pub method: String,
    pub collection: String,
    pub client_id: String,
    pub token_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for EventErc721 {
    fn default() -> Self {
        Self {
            id: Default::default(),
            payload: "".into(),
            txhash: "".into(),
            method: "".into(),
            collection: "".into(),
            client_id: "".into(),
            token_id: None,
            status: StatusEvent::Sent.as_str().to_string(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}

// Token
#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = crate::schema::service_webhook_token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ServiceWebhookToken {
    pub id: String,
    pub client_id: String,
    pub endpoint_url: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for ServiceWebhookToken {
    fn default() -> Self {
        Self {
            id: Default::default(),
            client_id: "".into(),
            endpoint_url: "".into(),
            created_by: "".into(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::service_token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ServiceToken {
    pub id: String,
    pub client_id: String,
    pub address: String,
    pub to_transfer: String,
    pub namespace: String,
    pub start_block_number: i32,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for ServiceToken {
    fn default() -> Self {
        Self {
            id: Default::default(),
            client_id: "".into(),
            address: "".into(),
            to_transfer: "".into(),
            namespace: "".into(),
            start_block_number: 1,
            created_by: "".into(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = crate::schema::events_erc20)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EventErc20 {
    pub id: String,
    pub payload: String,
    pub txhash: String,
    pub status: String,
    pub method: String,
    pub token_address: String,
    pub client_id: String,
    pub amount: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for EventErc20 {
    fn default() -> Self {
        Self {
            id: Default::default(),
            payload: "".into(),
            txhash: "".into(),
            method: "".into(),
            token_address: "".into(),
            client_id: "".into(),
            amount: 0.0,
            status: StatusEvent::Sent.as_str().to_string(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}
