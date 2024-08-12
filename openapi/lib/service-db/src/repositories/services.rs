use {
    crate::{
        database::Database,
        models::{Service, ServiceError},
        schema::services,
    },
    chrono::Local,
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct Services {
    db: Arc<Database>,
}

impl Services {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<Service>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = services::table
            .select(Service::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get(&self, id: Option<String>, service_id: Option<String>) -> Result<Service, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = services::table
            .filter(services::id.eq(id.unwrap_or_else(|| "".to_string())))
            .or_filter(services::client_id.eq(service_id.unwrap_or_else(|| "".to_string())))
            .select(Service::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn register_service(
        &self,
        client_id: String,
        info: String,
    ) -> Result<Service, ServiceError> {
        let service = self
            .get(Some("".to_string()), Some(client_id.clone()))
            .await
            .unwrap_or(Service::default());
        if service.client_id != "".to_string() {
            return Err(ServiceError {
                msg: "Service exists".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_service = Service {
            id: uuid.to_string(),
            client_id: client_id,
            info: info,
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        };

        let mut conn = self.db.get_connection().await;
        let ret = diesel::insert_into(services::table)
            .values(new_service)
            .returning(Service::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }
}
