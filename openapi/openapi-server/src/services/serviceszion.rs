use {
    crate::error::Result as TonicResult,
    openapi_proto::serviceszion_service::{services_zion_server::ServicesZion, *},
    std::sync::Arc,
    tonic::{Request, Response, Status},
    zion_service_db::{
        database::Database,
        repositories::{
            services::Services, services_collection::ServicesCollection,
            services_webhood::ServicesWebhood,
        },
    },
};

#[derive(Debug, Clone)]
pub struct ServicesZionService {
    services_db: Arc<Services>,
    services_webhood_db: Arc<ServicesWebhood>,
    services_collection_db: Arc<ServicesCollection>,
}

impl ServicesZionService {
    pub fn new(db: Arc<Database>) -> Self {
        let services_db = Arc::new(Services::new(Arc::clone(&db)));
        let services_webhood_db = Arc::new(ServicesWebhood::new(Arc::clone(&db)));
        let services_collection_db = Arc::new(ServicesCollection::new(Arc::clone(&db)));
        Self {
            services_db,
            services_webhood_db,
            services_collection_db,
        }
    }
}

#[tonic::async_trait]
impl ServicesZion for ServicesZionService {
    async fn register_service(
        &self,
        req: Request<RegisterServiceRequest>,
    ) -> TonicResult<Response<InfoService>> {
        if req.get_ref().client_id.is_empty() || req.get_ref().info.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id or info is empty",
            ));
        }
        let response = self
            .services_db
            .register_service(req.get_ref().client_id.clone(), req.get_ref().info.clone())
            .await;
        match response {
            Ok(service) => {
                let mut response = InfoService::default();
                response.id = service.id.clone();
                response.client_id = service.client_id.clone();
                response.info = service.info.clone();
                response.created_at = service.created_at.to_string();
                response.updated_at = service.updated_at.to_string();
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_all_services(
        &self,
        _: Request<GetAllServicesRequest>,
    ) -> TonicResult<Response<GetAllServicesResponse>> {
        let response = self.services_db.get_all().await;
        match response {
            Ok(service) => {
                let lst_services = service
                    .iter()
                    .map(|s| {
                        let mut response = InfoService::default();
                        response.id = s.id.clone();
                        response.client_id = s.client_id.clone();
                        response.info = s.info.clone();
                        response.created_at = s.created_at.to_string();
                        response.updated_at = s.updated_at.to_string();
                        response
                    })
                    .collect();
                let mut response = GetAllServicesResponse::default();
                response.data = lst_services;
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_service(
        &self,
        req: Request<GetInfoServiceRequest>,
    ) -> TonicResult<Response<InfoService>> {
        let response = self
            .services_db
            .get(req.get_ref().id.clone(), req.get_ref().client_id.clone())
            .await;
        match response {
            Ok(service) => {
                let mut response = InfoService::default();
                response.id = service.id.clone();
                response.client_id = service.client_id.clone();
                response.info = service.info.clone();
                response.created_at = service.created_at.to_string();
                response.updated_at = service.updated_at.to_string();
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    // Service Endpoints
    async fn get_all_endpoint_for_service(
        &self,
        _: Request<GetAllEndpointForServiceRequest>,
    ) -> TonicResult<Response<GetAllEndpointForServiceResponse>> {
        let response = self.services_webhood_db.get_all().await;
        match response {
            Ok(service) => {
                let lst_services = service
                    .iter()
                    .map(|s| {
                        let mut response = EndpointForService::default();
                        response.id = s.id.clone();
                        response.client_id = s.client_id.clone();
                        response.endpoint_url = s.endpoint_url.clone();
                        response.created_at = s.created_at.to_string();
                        response.updated_at = s.updated_at.to_string();
                        response
                    })
                    .collect();
                let mut response = GetAllEndpointForServiceResponse::default();
                response.data = lst_services;
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_endpoint_for_service(
        &self,
        req: Request<GetInfoEndpointForServiceRequest>,
    ) -> TonicResult<Response<EndpointForService>> {
        let response = self
            .services_webhood_db
            .get(req.get_ref().id.clone(), req.get_ref().client_id.clone())
            .await;
        match response {
            Ok(service) => {
                let mut response = EndpointForService::default();
                response.id = service.id.clone();
                response.client_id = service.client_id.clone();
                response.endpoint_url = service.endpoint_url.clone();
                response.created_at = service.created_at.to_string();
                response.updated_at = service.updated_at.to_string();
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn resgiter_endpoint_for_service(
        &self,
        req: Request<ResgiterEndpointForServiceRequest>,
    ) -> TonicResult<Response<ResgiterEndpointForServiceResponse>> {
        if req.get_ref().client_id.is_empty() || req.get_ref().endpoint_url.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id or endpoint_url is empty",
            ));
        }
        let response = self
            .services_webhood_db
            .register_service_webhood(
                req.get_ref().client_id.clone(),
                req.get_ref().endpoint_url.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let mut response = ResgiterEndpointForServiceResponse::default();
                response.id = service.id.clone();
                response.client_id = service.client_id.clone();
                response.endpoint_url = service.endpoint_url.clone();
                response.created_at = service.created_at.to_string();
                response.updated_at = service.updated_at.to_string();
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn un_register_endpoint_for_service(
        &self,
        req: Request<UnRegisterEndpointForServiceRequest>,
    ) -> TonicResult<Response<UnRegisterEndpointForServiceResponse>> {
        if req.get_ref().client_id.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id is empty",
            ));
        }
        let response = self
            .services_webhood_db
            .un_register_service_webhood(req.get_ref().client_id.clone())
            .await;
        match response {
            Ok(service) => {
                let mut response = UnRegisterEndpointForServiceResponse::default();
                response.id = service.id.clone();
                response.client_id = service.client_id.clone();
                response.endpoint_url = service.endpoint_url.clone();
                response.created_at = service.created_at.to_string();
                response.updated_at = service.updated_at.to_string();
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    //Service Collections
    async fn get_all_collection_for_service(
        &self,
        _: Request<GetAllCollectionForServiceRequest>,
    ) -> TonicResult<Response<GetAllCollectionForServiceResponse>> {
        let response = self.services_collection_db.get_all().await;
        match response {
            Ok(service) => {
                let lst_services = service
                    .iter()
                    .map(|s| {
                        let mut response = CollectionForService::default();
                        response.id = s.id.clone();
                        response.client_id = s.client_id.clone();
                        response.address = s.address.clone();
                        response.created_at = s.created_at.to_string();
                        response.updated_at = s.updated_at.to_string();
                        response
                    })
                    .collect();
                let mut response = GetAllCollectionForServiceResponse::default();
                response.data = lst_services;
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_collection_for_service(
        &self,
        req: Request<GeInfoCollectionForServiceRequest>,
    ) -> TonicResult<Response<CollectionForService>> {
        let response = self
            .services_collection_db
            .get(req.get_ref().id.clone(), req.get_ref().client_id.clone())
            .await;
        match response {
            Ok(service) => {
                let mut response = CollectionForService::default();
                response.id = service.id.clone();
                response.client_id = service.client_id.clone();
                response.address = service.address.clone();
                response.created_at = service.created_at.to_string();
                response.updated_at = service.updated_at.to_string();
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn register_collection_for_service(
        &self,
        req: Request<RegisterCollectionForServiceRequest>,
    ) -> TonicResult<Response<RegisterCollectionForServiceResponse>> {
        if req.get_ref().client_id.is_empty() || req.get_ref().address.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id or address is empty",
            ));
        }
        let response = self
            .services_collection_db
            .register_service_collection(
                req.get_ref().client_id.clone(),
                req.get_ref().address.clone(),
                req.get_ref().namespace.clone(),
                req.get_ref().start_block_number.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let mut response = RegisterCollectionForServiceResponse::default();
                response.id = service.id.clone();
                response.client_id = service.client_id.clone();
                response.address = service.address.clone();
                response.namespace = service.namespace.clone();
                response.created_at = service.created_at.to_string();
                response.updated_at = service.updated_at.to_string();
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn un_register_collection_for_service(
        &self,
        req: Request<UnRegisterCollectionForServiceRequest>,
    ) -> TonicResult<Response<UnRegisterCollectionForServiceResponse>> {
        if req.get_ref().client_id.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id is empty",
            ));
        }
        let response = self
            .services_collection_db
            .un_register_service_collection(req.get_ref().client_id.clone())
            .await;
        match response {
            Ok(service) => {
                let mut response = UnRegisterCollectionForServiceResponse::default();
                response.id = service.id.clone();
                response.client_id = service.client_id.clone();
                response.address = service.address.clone();
                response.created_at = service.created_at.to_string();
                response.updated_at = service.updated_at.to_string();
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    // Test Service
    async fn test_send_to_endpoints(
        &self,
        req: Request<TestSendToEndpointsRequest>,
    ) -> TonicResult<Response<TestSendToEndpointsResponse>> {
        let mut response = TestSendToEndpointsResponse::default();
        response.code = "1".to_string();
        response.description = "Success".to_string();
        response.id = req.get_ref().id.clone();
        Ok(Response::new(response))
    }
}
