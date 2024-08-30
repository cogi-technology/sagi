use {
    crate::{
        database::Database,
        models::{EventErc721, ServiceError, StatusEvent},
        schema::events_erc721,
    },
    chrono::Local,
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct NftEvents {
    db: Arc<Database>,
}

impl NftEvents {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<EventErc721>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = events_erc721::table
            .select(EventErc721::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get(
        &self,
        id: Option<String>,
        tx: Option<String>,
    ) -> Result<EventErc721, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = events_erc721::table
            .filter(events_erc721::id.eq(id.unwrap_or_default()))
            .or_filter(events_erc721::txhash.eq(tx.unwrap_or_default()))
            .select(EventErc721::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_events_by_id(&self, id: String) -> Result<EventErc721, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = events_erc721::table
            .filter(events_erc721::id.eq(id.clone()))
            .select(EventErc721::as_select())
            .order(events_erc721::id)
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_events_by_client_id(
        &self,
        client_id: String,
    ) -> Result<Vec<EventErc721>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = events_erc721::table
            .filter(events_erc721::client_id.eq(client_id.clone()))
            .select(EventErc721::as_select())
            .order((events_erc721::client_id, events_erc721::created_at.asc()))
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_events_filters(
        &self,
        id: Option<String>,
        client_id: Option<String>,
        collection: Option<String>,
        token_id: Option<i32>,
    ) -> Result<Vec<EventErc721>, ServiceError> {
        let mut conn = self.db.get_connection().await;
        let mut query = events_erc721::table.into_boxed();
        if let Some(id) = id {
            query = query.filter(events_erc721::id.eq(id));
        }
        if let Some(client_id) = client_id {
            query = query.filter(events_erc721::client_id.eq(client_id));
        }
        if let Some(collection) = collection {
            query = query.filter(events_erc721::collection.eq(collection));
        }
        if let Some(token_id) = token_id {
            query = query.filter(events_erc721::token_id.eq(token_id));
        }
        let ret = query
            .select(EventErc721::as_select())
            .order((events_erc721::client_id, events_erc721::created_at.asc()))
            .load(&mut conn)
            .await?;
        Ok(ret)
    }

    pub async fn add(
        &self,
        client_id: String,
        payload: String,
        txhash: String,
        collection: String,
        method: String,
        token_id: i32,
    ) -> Result<EventErc721, ServiceError> {
        let event = self
            .get(Some("".to_string()), Some(txhash.clone()))
            .await
            .unwrap_or(EventErc721::default());
        if !event.payload.is_empty() && !event.txhash.is_empty() {
            return Err(ServiceError {
                msg: "Event exists".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_event = EventErc721 {
            id: uuid.to_string(),
            payload,
            txhash,
            collection,
            method,
            token_id: Some(token_id),
            client_id,
            status: StatusEvent::Init.as_str().to_string(),
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        };

        let mut conn = self.db.get_connection().await;
        let ret = diesel::insert_into(events_erc721::table)
            .values(new_event)
            .returning(EventErc721::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn update_status(
        &self,
        id: String,
        status: StatusEvent,
    ) -> Result<EventErc721, ServiceError> {
        let event = self
            .get(Some(id.to_string()), Some("".to_string()))
            .await
            .unwrap_or(EventErc721::default());
        if event.id.is_empty() {
            return Err(ServiceError {
                msg: "Event not exists".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let mut conn = self.db.get_connection().await;
        let ret = diesel::update(events_erc721::table.filter(events_erc721::id.eq(id)))
            .set((
                events_erc721::status.eq(status.as_str().to_string()),
                events_erc721::updated_at.eq(Local::now().naive_utc()),
            ))
            .returning(EventErc721::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }
}
