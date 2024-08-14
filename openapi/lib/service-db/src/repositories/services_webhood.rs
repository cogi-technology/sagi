use {
    crate::{
        database::Database,
        models::{ServiceError, ServiceWebhood},
        schema::services_webhood,
    },
    chrono::Local,
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct ServicesWebhood {
    db: Arc<Database>,
}

impl ServicesWebhood {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<ServiceWebhood>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = services_webhood::table
            .select(ServiceWebhood::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get(
        &self,
        id: Option<String>,
        client_id: Option<String>,
    ) -> Result<ServiceWebhood, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = services_webhood::table
            .filter(services_webhood::id.eq(id.unwrap_or_else(|| "".to_string())))
            .or_filter(services_webhood::client_id.eq(client_id.unwrap_or_else(|| "".to_string())))
            .select(ServiceWebhood::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn register_service_webhood(
        &self,
        client_id: String,
        endpoint_url: String,
    ) -> Result<ServiceWebhood, ServiceError> {
        let service = self
            .get(Some("".to_string()), Some(client_id.clone()))
            .await
            .unwrap_or(ServiceWebhood::default());
        if service.client_id != "".to_string() {
            return Err(ServiceError {
                msg: "Service has Endpoint. Please unRegister before.".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_service = ServiceWebhood {
            id: uuid.to_string(),
            client_id: client_id,
            endpoint_url: endpoint_url,
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        };

        let mut conn = self.db.get_connection().await;
        let ret = diesel::insert_into(services_webhood::table)
            .values(new_service)
            .returning(ServiceWebhood::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn un_register_service_webhood(
        &self,
        client_id: String
    ) -> Result<ServiceWebhood, ServiceError> {
        let service = self
            .get(Some("".to_string()), Some(client_id.clone()))
            .await
            .unwrap_or(ServiceWebhood::default());
        if service.client_id == "".to_string() || service.endpoint_url == "".to_string() {
            return Err(ServiceError {
                msg: "Endpoint for Service not exist".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let mut conn = self.db.get_connection().await;
        let ret = diesel::delete(
            services_webhood::table.filter(services_webhood::client_id.eq(client_id)),
        )
        .returning(ServiceWebhood::as_returning())
        .get_result(&mut conn)
        .await?;

        Ok(ret)
    }
}
