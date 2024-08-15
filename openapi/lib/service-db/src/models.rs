use {
    chrono::{Local, NaiveDateTime},
    diesel::{
        deserialize::{self, FromSql, FromSqlRow},
        expression::AsExpression,
        prelude::*,
        serialize::{self, Output, ToSql},
        sql_types::*,
    },
    diesel_enum::DbEnum,
    ethers::prelude::Address as EthereumAddress,
    std::fmt::Debug,
};

#[derive(Debug, thiserror::Error)]
#[error("BillError: {msg}, {status}")]
pub struct BillError {
    pub msg: String,
    pub status: i32,
}

impl BillError {
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

impl From<diesel::result::Error> for BillError {
    fn from(error: diesel::result::Error) -> Self {
        if error == diesel::result::Error::NotFound {
            BillError::not_found(error.to_string())
        } else {
            BillError::something_wrong(error.to_string())
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, DbEnum, FromPrimitive, ToPrimitive,
)]
#[diesel(sql_type = SmallInt)]
#[diesel_enum(error_fn = BillError::not_found)]
#[diesel_enum(error_type = BillError)]
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

#[derive(AsExpression, FromSqlRow, Debug, Copy, Clone, PartialEq)]
#[diesel(sql_type = VarChar)]
pub struct Address {
    value: EthereumAddress,
}

impl ToSql<VarChar, diesel::pg::Pg> for Address {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        let address = &self.value;

        <String as ToSql<VarChar, diesel::pg::Pg>>::to_sql(
            &format!("{address:#x}"),
            &mut out.reborrow(),
        )
    }
}

impl FromSql<VarChar, diesel::pg::Pg> for Address {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        <String as FromSql<VarChar, diesel::pg::Pg>>::from_sql(bytes).map(|s| Address {
            value: s.parse().unwrap(),
        })
    }
}

#[derive(Queryable, Identifiable, AsChangeset, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::bills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct Bill {
    pub id: String,
    pub customer: String,
    pub paid_amount: String,
    pub rewarded_amount: Option<String>,
    pub paid_txhash: String,
    pub rewarded_txhash: Option<String>,
    pub paid_at: NaiveDateTime,
    pub rewarded_at: Option<NaiveDateTime>,
    pub status: Status,
}

impl Default for Bill {
    fn default() -> Self {
        Self {
            id: Default::default(),
            customer: "0x0000000000000000000000000000000000000000".into(),
            paid_amount: Default::default(),
            rewarded_amount: None,
            paid_txhash: "0x0000000000000000000000000000000000000000".into(),
            rewarded_txhash: None,
            paid_at: Local::now().naive_utc(),
            rewarded_at: None,
            status: Status::NotFound,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::bills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewBill<'a> {
    pub id: &'a str,
    pub customer: &'a str,
    pub paid_amount: &'a str,
    pub paid_txhash: &'a str,
    pub paid_at: &'a NaiveDateTime,
    pub status: &'a i16,
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            id: Default::default(),
            client_id: "".into(),
            info: "".into(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq)]
#[diesel(table_name = crate::schema::services_webhood)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ServiceWebhood {
    pub id: String,
    pub client_id: String,
    pub endpoint_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for ServiceWebhood {
    fn default() -> Self {
        Self {
            id: Default::default(),
            client_id: "".into(),
            endpoint_url: "".into(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::services_collections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ServiceCollection {
    pub id: String,
    pub client_id: String,
    pub address: String,
    pub namespace: String,
    pub status: i32,
    pub start_block_number: i32,
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
            status: 1,
            start_block_number: 1,
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
    pub token_id: i32,
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
            token_id: 0,
            status: StatusEvent::Sent.as_str().to_string(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        }
    }
}
