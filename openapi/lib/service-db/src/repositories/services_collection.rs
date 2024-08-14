use {
    crate::{
        database::Database,
        models::{ServiceCollection, ServiceError},
        schema::services_collections,
    },
    chrono::Local,
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct ServicesCollection {
    db: Arc<Database>,
}

impl ServicesCollection {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<ServiceCollection>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = services_collections::table
            .select(ServiceCollection::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get(
        &self,
        id: Option<String>,
        client_id: Option<String>,
    ) -> Result<ServiceCollection, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = services_collections::table
            .filter(services_collections::id.eq(id.unwrap_or_else(|| "".to_string())))
            .or_filter(
                services_collections::client_id.eq(client_id.unwrap_or_else(|| "".to_string())),
            )
            .select(ServiceCollection::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn register_service_collection(
        &self,
        client_id: String,
        address: String,
        namespace: String,
        start_block_number: i32
    ) -> Result<ServiceCollection, ServiceError> {
        let service = self
            .get(Some("".to_string()), Some(client_id.clone()))
            .await
            .unwrap_or(ServiceCollection::default());
        if service.client_id != "".to_string() && service.address != "".to_string() {
            return Err(ServiceError {
                msg: "Collection for Service exists".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_service = ServiceCollection {
            id: uuid.to_string(),
            client_id: client_id,
            address: address,
            namespace: namespace.to_string(),
            status: 1,
            start_block_number: start_block_number,
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        };

        let mut conn = self.db.get_connection().await;
        let ret = diesel::insert_into(services_collections::table)
            .values(new_service)
            .returning(ServiceCollection::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn un_register_service_collection(
        &self,
        client_id: String,
    ) -> Result<ServiceCollection, ServiceError> {
        let service = self
            .get(Some("".to_string()), Some(client_id.clone()))
            .await
            .unwrap_or(ServiceCollection::default());
        if service.client_id == "".to_string() || service.address == "".to_string() {
            return Err(ServiceError {
                msg: "Endpoint for Service not exist".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let mut conn = self.db.get_connection().await;
        let ret = diesel::delete(
            services_collections::table.filter(services_collections::client_id.eq(client_id)),
        )
        .returning(ServiceCollection::as_returning())
        .get_result(&mut conn)
        .await?;

        Ok(ret)
    }
}
