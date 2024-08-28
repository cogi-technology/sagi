use {
    crate::{
        database::Database,
        models::{Service, ServiceError, ServiceToken},
        schema::{service_token, services},
    },
    chrono::Local,
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct ServicesToken {
    db: Arc<Database>,
}

impl ServicesToken {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<ServiceToken>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_token::table
            .select(ServiceToken::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get_all_with_created_by(
        &self,
        created_by: String,
    ) -> Result<Vec<ServiceToken>, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_token::table
            .filter(service_token::created_by.eq(created_by))
            .select(ServiceToken::as_select())
            .load(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn get(
        &self,
        id: Option<String>,
        client_id: Option<String>,
        created_by: String,
    ) -> Result<ServiceToken, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_token::table
            .filter(service_token::created_by.eq(created_by))
            .filter(service_token::id.eq(id.unwrap_or_else(|| "".to_string())))
            .or_filter(service_token::client_id.eq(client_id.unwrap_or_else(|| "".to_string())))
            .select(ServiceToken::as_select())
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

    pub async fn check_exist_service_token(
        &self,
        id: Option<String>,
        client_id: Option<String>,
    ) -> Result<ServiceToken, ServiceError> {
        let mut conn = self.db.get_connection().await;

        let ret = service_token::table
            .filter(service_token::client_id.eq(client_id.unwrap_or_else(|| "".to_string())))
            .or_filter(service_token::id.eq(id.unwrap_or_else(|| "".to_string())))
            .select(ServiceToken::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn register_service_token(
        &self,
        client_id: String,
        address: String,
        to_transfer: String,
        namespace: String,
        start_block_number: i32,
        created_by: String,
    ) -> Result<ServiceToken, ServiceError> {
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

        let service_token = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceToken::default());
        if service_token.client_id != "".to_string() && service_token.address == address.to_string()
        {
            return Err(ServiceError {
                msg: "Collection for service existence".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let uuid = Uuid::new_v4();
        let new_service = ServiceToken {
            id: uuid.to_string(),
            client_id: client_id,
            address: address,
            to_transfer: to_transfer,
            namespace: namespace.to_string(),
            start_block_number: start_block_number,
            created_by: created_by,
            created_at: Local::now().naive_utc(),
            updated_at: Local::now().naive_utc(),
        };

        let mut conn = self.db.get_connection().await;
        let ret = diesel::insert_into(service_token::table)
            .values(new_service)
            .returning(ServiceToken::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }

    pub async fn un_register_service_token(
        &self,
        client_id: String,
        created_by: String,
    ) -> Result<ServiceToken, ServiceError> {
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

        let service_token = self
            .get(
                Some("".to_string()),
                Some(client_id.clone()),
                created_by.clone(),
            )
            .await
            .unwrap_or(ServiceToken::default());
        if service_token.client_id == "".to_string() || service_token.address == "".to_string() {
            return Err(ServiceError {
                msg: "Endpoint for Service not available".into(),
                status: tonic::Code::Unknown as i32,
            });
        }
        let mut conn = self.db.get_connection().await;
        let ret =
            diesel::delete(service_token::table.filter(service_token::client_id.eq(client_id)))
                .returning(ServiceToken::as_returning())
                .get_result(&mut conn)
                .await?;

        Ok(ret)
    }

    pub async fn update_start_block_number(
        &self,
        id: String,
        start_block_number: i32,
    ) -> Result<ServiceToken, ServiceError> {
        let service = self
            .check_exist_service_token(Some(id.to_string()), Some("".to_string()))
            .await
            .unwrap_or(ServiceToken::default());
        if service.client_id == "".to_string() || service.address == "".to_string() {
            return Err(ServiceError {
                msg: "Endpoint for Service not available".into(),
                status: tonic::Code::Unknown as i32,
            });
        }

        let mut conn = self.db.get_connection().await;
        let ret = diesel::update(service_token::table.filter(service_token::id.eq(id.clone())))
            .set((
                service_token::start_block_number.eq(start_block_number),
                service_token::updated_at.eq(Local::now().naive_utc()),
            ))
            .returning(ServiceToken::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }
}
