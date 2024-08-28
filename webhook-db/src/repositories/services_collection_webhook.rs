use {
    crate::{
        database::Database,
        models::{Service, ServiceError, ServiceWebhookCollection},
        schema::{service_webhook_collection, services},
    },
    chrono::Local,
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct ServicesCollectionWebhook {
    db: Arc<Database>,
}

impl ServicesCollectionWebhook {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<ServiceWebhookCollection>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_webhook_collection::table
            .select(ServiceWebhookCollection::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_all_with_created_by(
        &self,
        created_by: String,
    ) -> Result<Vec<ServiceWebhookCollection>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_webhook_collection::table
            .filter(service_webhook_collection::created_by.eq(created_by))
            .select(ServiceWebhookCollection::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get(
        &self,
        id: Option<String>,
        client_id: Option<String>,
        created_by: String,
    ) -> Result<ServiceWebhookCollection, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_webhook_collection::table
            .filter(service_webhook_collection::created_by.eq(created_by))
            .filter(service_webhook_collection::id.eq(id.unwrap_or_else(|| "".to_string())))
            .or_filter(
                service_webhook_collection::client_id
                    .eq(client_id.unwrap_or_else(|| "".to_string())),
            )
            .select(ServiceWebhookCollection::as_select())
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

    pub async fn check_exist_service_webhook_collection(
        &self,
        client_id: Option<String>,
    ) -> Result<ServiceWebhookCollection, ServiceError> {
        let mut conn = self.db.get_connection().await;
        let ret = service_webhook_collection::table
            .filter(
                service_webhook_collection::client_id
                    .eq(client_id.unwrap_or_else(|| "".to_string())),
            )
            .select(ServiceWebhookCollection::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn register_service_webhook_collection(
        &self,
        client_id: String,
        endpoint_url: String,
        created_by: String,
    ) -> Result<ServiceWebhookCollection, ServiceError> {
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

        let service_webhook_collection = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceWebhookCollection::default());
        if service_webhook_collection.client_id != "".to_string() {
            return Err(ServiceError {
                msg: "The service has Endpoint. Please unregister first".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_service = ServiceWebhookCollection {
            id: uuid.to_string(),
            client_id: client_id,
            endpoint_url: endpoint_url,
            created_by: created_by,
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        };

        let mut conn = self.db.get_connection().await;
        let ret = diesel::insert_into(service_webhook_collection::table)
            .values(new_service)
            .returning(ServiceWebhookCollection::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn un_register_service_webhook_collection(
        &self,
        client_id: String,
        created_by: String,
    ) -> Result<ServiceWebhookCollection, ServiceError> {
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

        let service_webhook_collection = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceWebhookCollection::default());
        if service_webhook_collection.client_id == "".to_string()
            || service_webhook_collection.endpoint_url == "".to_string()
        {
            return Err(ServiceError {
                msg: "The endpoint for the service does not exist.".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let mut conn = self.db.get_connection().await;
        let ret = diesel::delete(
            service_webhook_collection::table
                .filter(service_webhook_collection::client_id.eq(client_id)),
        )
        .returning(ServiceWebhookCollection::as_returning())
        .get_result(&mut conn)
        .await?;

        Ok(ret)
    }

    pub async fn update_service_webhook_collection(
        &self,
        client_id: String,
        endpoint_url: String,
        created_by: String,
    ) -> Result<ServiceWebhookCollection, ServiceError> {
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

        let service_webhook_collection = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceWebhookCollection::default());

        if service_webhook_collection.client_id == "".to_string()
            || service_webhook_collection.id == "".to_string()
        {
            return Err(ServiceError {
                msg: "The service did not register the endpoint. Please register the endpoint."
                    .into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let mut conn = self.db.get_connection().await;
        let ret = diesel::update(
            service_webhook_collection::table
                .filter(service_webhook_collection::client_id.eq(client_id.clone())),
        )
        .set((
            service_webhook_collection::endpoint_url.eq(endpoint_url),
            service_webhook_collection::updated_at.eq(Local::now().naive_utc()),
        ))
        .returning(ServiceWebhookCollection::as_returning())
        .get_result(&mut conn)
        .await?;

        Ok(ret)
    }
}
