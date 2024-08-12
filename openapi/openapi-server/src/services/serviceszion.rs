use {
    crate::{
        cache::JWT_CACHE,
        config::TelegramAuthConfig,
        entity::telegram::{GetProofRequest, GetRequestType, GetSaltRequest},
        error::{into_anyhow, Result as TonicResult},
        helpers::{into::proto_proofpoint_from, utils::send_request_text},
    },
    anyhow::{anyhow, Result},
    ethers::signers::{LocalWallet, Signer},
    ethers_core::{k256::ecdsa::SigningKey, rand::rngs::OsRng},
    jsonwebtoken::TokenData,
    openapi_logger::debug,
    openapi_proto::serviceszion_service::{services_zion_server::ServicesZion, *},
    reqwest::{Client as ClientReqwest, Method},
    std::sync::Arc,
    tonic::{metadata::MetadataMap, Request, Response, Status},
    zion_aa::{
        address_to_string,
        types::{
            jwt::{JWTPayload, ProofPoints as SdkProofPoints},
            request::AuthorizationData,
        },
    },
    zion_service_db::{database::Database, repositories::services::Services},
};

#[derive(Debug, Clone)]
pub struct ServicesZionService {
    services_db: Arc<Services>, // pub cfg: TelegramAuthConfig,
}

impl ServicesZionService {
    pub fn new(db: Arc<Database>) -> Self {
        let services_db = Arc::new(Services::new(Arc::clone(&db)));
        Self { services_db }
    }
}

#[tonic::async_trait]
impl ServicesZion for ServicesZionService {
    async fn register_service(
        &self,
        req: Request<RegisterServiceRequest>,
    ) -> TonicResult<Response<InfoService>> {
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
            },
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
                let lst_services = service.iter().map(|s| {
                    let mut response = InfoService::default();
                    response.id = s.id.clone();
                    response.client_id = s.client_id.clone();
                    response.info = s.info.clone();
                    response.created_at = s.created_at.to_string();
                    response.updated_at = s.updated_at.to_string();
                    response
                }).collect();
                let mut response = GetAllServicesResponse::default();
                response.lst_services = lst_services;
                Ok(Response::new(response))
            },
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_service(
        &self,
        req: Request<GetInfoServiceRequest>,
    ) -> TonicResult<Response<InfoService>> {
        let response = self.services_db.get(req.get_ref().id.clone(), req.get_ref().client_id.clone()).await;
        match response {
            Ok(service) => {
                let mut response = InfoService::default();
                response.id = service.id.clone();
                response.client_id = service.client_id.clone();
                response.info = service.info.clone();
                response.created_at = service.created_at.to_string();
                response.updated_at = service.updated_at.to_string();
                Ok(Response::new(response))
            },
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    // Service Endpoints
    async fn get_all_endpoint_for_service(
        &self,
        _: Request<GetAllEndpointForServiceRequest>,
    ) -> TonicResult<Response<GetAllEndpointForServiceResponse>> {
        Err(Status::new(tonic::Code::Aborted, "Error".to_string()))
    }

    async fn get_info_endpoint_for_service(
        &self,
        req: Request<GetInfoEndpointForServiceRequest>,
    ) -> TonicResult<Response<EndpointForService>> {
        Err(Status::new(tonic::Code::Aborted, "Error".to_string()))
    }

    async fn resgiter_endpoint_for_service(
        &self,
        req: Request<ResgiterEndpointForServiceRequest>,
    ) -> TonicResult<Response<ResgiterEndpointForServiceResponse>> {
        Err(Status::new(tonic::Code::Aborted, "Error".to_string()))
    }

    async fn un_register_endpoint_for_service(
        &self,
        req: Request<UnRegisterEndpointForServiceRequest>,
    ) -> TonicResult<Response<UnRegisterEndpointForServiceResponse>> {
        Err(Status::new(tonic::Code::Aborted, "Error".to_string()))
    }

    //Service Collections
    async fn get_all_collection_for_service(
        &self,
        req: Request<GetAllCollectionForServiceRequest>,
    ) -> TonicResult<Response<GetAllCollectionForServiceResponse>> {
        Err(Status::new(tonic::Code::Aborted, "Error".to_string()))
    }

    async fn ge_info_collection_for_service(
        &self,
        req: Request<GeInfoCollectionForServiceRequest>,
    ) -> TonicResult<Response<CollectionForService>> {
        Err(Status::new(tonic::Code::Aborted, "Error".to_string()))
    }

    async fn register_collection_for_service(
        &self,
        req: Request<RegisterCollectionForServiceRequest>,
    ) -> TonicResult<Response<RegisterCollectionForServiceResponse>> {
        Err(Status::new(tonic::Code::Aborted, "Error".to_string()))
    }

    async fn un_register_collection_for_service(
        &self,
        req: Request<UnRegisterCollectionForServiceRequest>,
    ) -> TonicResult<Response<UnRegisterCollectionForServiceResponse>> {
        Err(Status::new(tonic::Code::Aborted, "Error".to_string()))
    }

    // Test Service
    async fn test_send_to_endpoints(
        &self,
        req: Request<TestSendToEndpointsRequest>,
    ) -> TonicResult<Response<TestSendToEndpointsResponse>> {
        Err(Status::new(tonic::Code::Aborted, "Error".to_string()))
    }
}
