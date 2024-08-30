use {
    crate::{
        database::Database,
        models::{Service, ServiceError, ServiceWebhookToken},
        schema::{service_webhook_token, services},
    },
    chrono::Local,
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct ServicesTokenWebhook {
    db: Arc<Database>,
}

impl ServicesTokenWebhook {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<ServiceWebhookToken>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_webhook_token::table
            .select(ServiceWebhookToken::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_all_with_created_by(
        &self,
        created_by: String,
    ) -> Result<Vec<ServiceWebhookToken>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_webhook_token::table
            .filter(service_webhook_token::created_by.eq(created_by))
            .select(ServiceWebhookToken::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get(
        &self,
        id: Option<String>,
        client_id: Option<String>,
        created_by: String,
    ) -> Result<ServiceWebhookToken, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_webhook_token::table
            .filter(service_webhook_token::created_by.eq(created_by))
            .filter(service_webhook_token::id.eq(id.unwrap_or_default()))
            .or_filter(service_webhook_token::client_id.eq(client_id.unwrap_or_default()))
            .select(ServiceWebhookToken::as_select())
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
            .filter(services::id.eq(id.unwrap_or_default()))
            .or_filter(services::client_id.eq(client_id.unwrap_or_default()))
            .select(Service::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn check_exist_service_webhook_token(
        &self,
        client_id: Option<String>,
    ) -> Result<ServiceWebhookToken, ServiceError> {
        let mut conn = self.db.get_connection().await;
        let ret = service_webhook_token::table
            .filter(service_webhook_token::client_id.eq(client_id.unwrap_or_default()))
            .select(ServiceWebhookToken::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn register_service_webhook_token(
        &self,
        client_id: String,
        endpoint_url: String,
        created_by: String,
    ) -> Result<ServiceWebhookToken, ServiceError> {
        let service = self
            .get_service(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(Service::default());

        if service.client_id.is_empty() || service.id.is_empty() {
            return Err(ServiceError {
                msg: "Service not available".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let service_webhook_token = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceWebhookToken::default());
        if !service_webhook_token.client_id.is_empty() {
            return Err(ServiceError {
                msg: "The service has Endpoint. Please unregister first".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_service = ServiceWebhookToken {
            id: uuid.to_string(),
            client_id,
            endpoint_url,
            created_by,
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        };

        let mut conn = self.db.get_connection().await;
        let ret = diesel::insert_into(service_webhook_token::table)
            .values(new_service)
            .returning(ServiceWebhookToken::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn un_register_service_webhook_token(
        &self,
        client_id: String,
        created_by: String,
    ) -> Result<ServiceWebhookToken, ServiceError> {
        let service = self
            .get_service(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(Service::default());

        if service.client_id.is_empty() || service.id.is_empty() {
            return Err(ServiceError {
                msg: "Service not available".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let service_webhook_token = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceWebhookToken::default());
        if service_webhook_token.client_id.is_empty()
            || service_webhook_token.endpoint_url.is_empty()
        {
            return Err(ServiceError {
                msg: "The endpoint for the service does not exist.".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let mut conn = self.db.get_connection().await;
        let ret = diesel::delete(
            service_webhook_token::table.filter(service_webhook_token::client_id.eq(client_id)),
        )
        .returning(ServiceWebhookToken::as_returning())
        .get_result(&mut conn)
        .await?;

        Ok(ret)
    }

    pub async fn update_service_webhook_token(
        &self,
        client_id: String,
        endpoint_url: String,
        created_by: String,
    ) -> Result<ServiceWebhookToken, ServiceError> {
        let service = self
            .get_service(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(Service::default());

        if service.client_id.is_empty() || service.id.is_empty() {
            return Err(ServiceError {
                msg: "Service not available".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let service_webhook_token = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceWebhookToken::default());

        if service_webhook_token.client_id.is_empty() || service_webhook_token.id.is_empty() {
            return Err(ServiceError {
                msg: "The service did not register the endpoint. Please register the endpoint."
                    .into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let mut conn = self.db.get_connection().await;
        let ret = diesel::update(
            service_webhook_token::table
                .filter(service_webhook_token::client_id.eq(client_id.clone())),
        )
        .set((
            service_webhook_token::endpoint_url.eq(endpoint_url),
            service_webhook_token::updated_at.eq(Local::now().naive_utc()),
        ))
        .returning(ServiceWebhookToken::as_returning())
        .get_result(&mut conn)
        .await?;

        Ok(ret)
    }
}
