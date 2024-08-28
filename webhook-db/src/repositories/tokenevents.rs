use {
    crate::{
        database::Database,
        models::{EventErc20, ServiceError, StatusEvent},
        schema::events_erc20,
    },
    chrono::Local,
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct TokenEvents {
    db: Arc<Database>,
}

impl TokenEvents {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<EventErc20>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = events_erc20::table
            .select(EventErc20::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get(
        &self,
        id: Option<String>,
        tx: Option<String>,
    ) -> Result<EventErc20, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = events_erc20::table
            .filter(events_erc20::id.eq(id.unwrap_or_else(|| "".to_string())))
            .or_filter(events_erc20::txhash.eq(tx.unwrap_or_else(|| "".to_string())))
            .select(EventErc20::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_events_by_id(&self, id: String) -> Result<EventErc20, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = events_erc20::table
            .filter(events_erc20::id.eq(id.clone()))
            .select(EventErc20::as_select())
            .order(events_erc20::id)
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_events_by_client_id(
        &self,
        client_id: String,
    ) -> Result<Vec<EventErc20>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = events_erc20::table
            .filter(events_erc20::client_id.eq(client_id.clone()))
            .select(EventErc20::as_select())
            .order((events_erc20::client_id, events_erc20::created_at.asc()))
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_events_filters(
        &self,
        id: Option<String>,
        client_id: Option<String>,
        token_address: Option<String>,
    ) -> Result<Vec<EventErc20>, ServiceError> {
        let mut conn = self.db.get_connection().await;
        let mut query = events_erc20::table.into_boxed();
        if let Some(id) = id {
            query = query.filter(events_erc20::id.eq(id));
        }
        if let Some(client_id) = client_id {
            query = query.filter(events_erc20::client_id.eq(client_id));
        }
        if let Some(token_address) = token_address {
            query = query.filter(events_erc20::token_address.eq(token_address));
        }
        let ret = query
            .select(EventErc20::as_select())
            .order((events_erc20::client_id, events_erc20::created_at.asc()))
            .load(&mut conn)
            .await?;
        Ok(ret)
    }

    pub async fn add(
        &self,
        client_id: String,
        payload: String,
        txhash: String,
        token_address: String,
        method: String,
        amount: f64,
    ) -> Result<EventErc20, ServiceError> {
        let event = self
            .get(Some("".to_string()), Some(txhash.clone()))
            .await
            .unwrap_or(EventErc20::default());
        if event.payload != "".to_string() && event.txhash != "".to_string() {
            return Err(ServiceError {
                msg: "Event exists".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_event = EventErc20 {
            id: uuid.to_string(),
            payload: payload,
            txhash: txhash,
            token_address: token_address,
            method: method,
            amount: amount,
            client_id: client_id,
            status: StatusEvent::Init.as_str().to_string(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        };

        let mut conn = self.db.get_connection().await;
        let ret = diesel::insert_into(events_erc20::table)
            .values(new_event)
            .returning(EventErc20::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn update_status(
        &self,
        id: String,
        status: StatusEvent,
    ) -> Result<EventErc20, ServiceError> {
        let event = self
            .get(Some(id.to_string()), Some("".to_string()))
            .await
            .unwrap_or(EventErc20::default());
        if event.id == "".to_string() {
            return Err(ServiceError {
                msg: "Event not exists".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let mut conn = self.db.get_connection().await;
        let ret = diesel::update(events_erc20::table.filter(events_erc20::id.eq(id)))
            .set((
                events_erc20::status.eq(status.as_str().to_string()),
                events_erc20::updated_at.eq(Local::now().naive_utc()),
            ))
            .returning(EventErc20::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }
}
