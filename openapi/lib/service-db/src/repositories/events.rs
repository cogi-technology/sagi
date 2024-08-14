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

pub struct Events {
    db: Arc<Database>,
}

impl Events {
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
            .filter(events_erc721::id.eq(id.unwrap_or_else(|| "".to_string())))
            .or_filter(events_erc721::txhash.eq(tx.unwrap_or_else(|| "".to_string())))
            .select(EventErc721::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn add(
        &self,
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
        if event.payload != "".to_string() && event.txhash != "".to_string() {
            return Err(ServiceError {
                msg: "Event exists".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_event = EventErc721 {
            id: uuid.to_string(),
            payload: payload,
            txhash: txhash,
            collection: collection,
            method: method,
            token_id: token_id,
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

    pub async fn update(
        &self,
        id: String,
        status: StatusEvent,
    ) -> Result<EventErc721, ServiceError> {
        let event = self
            .get(Some(id.to_string()), Some("".to_string()))
            .await
            .unwrap_or(EventErc721::default());
        if event.payload != "".to_string() && event.txhash != "".to_string() {
            return Err(ServiceError {
                msg: "Event exists".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let mut conn = self.db.get_connection().await;
        let ret = diesel::update(events_erc721::table.filter(events_erc721::id.eq(id)))
            .set(events_erc721::status.eq(status.as_str().to_string()))
            .returning(EventErc721::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }
}
