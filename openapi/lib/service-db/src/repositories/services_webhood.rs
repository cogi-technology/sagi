use {
    crate::{
        database::Database,
        models::{Service, ServiceError, ServiceWebhood},
        schema::{services, services_webhood},
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

    pub async fn get_all_with_created_by(
        &self,
        created_by: String,
    ) -> Result<Vec<ServiceWebhood>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = services_webhood::table
            .filter(services_webhood::created_by.eq(created_by))
            .select(ServiceWebhood::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get(
        &self,
        id: Option<String>,
        client_id: Option<String>,
        created_by: String,
    ) -> Result<ServiceWebhood, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = services_webhood::table
            .filter(services_webhood::created_by.eq(created_by))
            .filter(services_webhood::id.eq(id.unwrap_or_else(|| "".to_string())))
            .or_filter(services_webhood::client_id.eq(client_id.unwrap_or_else(|| "".to_string())))
            .select(ServiceWebhood::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_service(
        &self,
        id: Option<String>,
        client_id: Option<String>,
        created_by: String,
    ) -> Result<Service, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = services::table
            .filter(services::created_by.eq(created_by))
            .filter(services::id.eq(id.unwrap_or_else(|| "".to_string())))
            .or_filter(services::client_id.eq(client_id.unwrap_or_else(|| "".to_string())))
            .select(Service::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn check_exist_service_webhood(
        &self,
        client_id: Option<String>,
    ) -> Result<ServiceWebhood, ServiceError> {
        let mut conn = self.db.get_connection().await;
        let ret = services_webhood::table
            .filter(services_webhood::client_id.eq(client_id.unwrap_or_else(|| "".to_string())))
            .select(ServiceWebhood::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn register_service_webhood(
        &self,
        client_id: String,
        endpoint_url: String,
        created_by: String,
    ) -> Result<ServiceWebhood, ServiceError> {
        let service = self
            .get_service(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(Service::default());

        if service.client_id == "".to_string() || service.id == "".to_string() {
            return Err(ServiceError {
                msg: "Service not available".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let service_webhood = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceWebhood::default());
        if service_webhood.client_id != "".to_string() {
            return Err(ServiceError {
                msg: "The service has Endpoint. Please unregister first".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_service = ServiceWebhood {
            id: uuid.to_string(),
            client_id: client_id,
            endpoint_url: endpoint_url,
            created_by: created_by,
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
        client_id: String,
        created_by: String,
    ) -> Result<ServiceWebhood, ServiceError> {
        let service = self
            .get_service(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(Service::default());

        if service.client_id == "".to_string() || service.id == "".to_string() {
            return Err(ServiceError {
                msg: "Service not available".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let service_webhood = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceWebhood::default());
        if service_webhood.client_id == "".to_string()
            || service_webhood.endpoint_url == "".to_string()
        {
            return Err(ServiceError {
                msg: "The endpoint for the service does not exist.".into(),
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

    pub async fn update_service_webhood(
        &self,
        client_id: String,
        endpoint_url: String,
        created_by: String,
    ) -> Result<ServiceWebhood, ServiceError> {
        let service = self
            .get_service(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(Service::default());

        if service.client_id == "".to_string() || service.id == "".to_string() {
            return Err(ServiceError {
                msg: "Service not available".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let service_webhood = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceWebhood::default());

        if service_webhood.client_id == "".to_string() || service_webhood.id == "".to_string() {
            return Err(ServiceError {
                msg: "The service did not register the endpoint. Please register the endpoint.".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        
        let mut conn = self.db.get_connection().await;
        let ret = diesel::update(
            services_webhood::table.filter(services_webhood::client_id.eq(client_id.clone())),
        )
        .set((
            services_webhood::endpoint_url.eq(endpoint_url),
            services_webhood::updated_at.eq(Local::now().naive_utc()),
        ))
        .returning(ServiceWebhood::as_returning())
        .get_result(&mut conn)
        .await?;

        Ok(ret)
    }
}
