use {
    crate::error::{into_anyhow, Result as TonicResult},
    openapi_proto::{
        authtelegram_service::{TestSendToEndpointsRequest, TestSendToEndpointsResponse},
        serviceszion_service::{services_zion_server::ServicesZion, *},
    },
    reqwest::{Client, Method},
    std::{collections::HashMap, sync::Arc},
    tonic::{Request, Response, Status},
    zion_service_db::{
        database::Database,
        models::StatusEvent,
        repositories::{
            events::Events, services::Services, services_collection::ServicesCollection,
            services_webhood::ServicesWebhood,
        },
    },
    zion_service_etherman::utils::{get_signature, load_private_key_from_file, send_request_text},
};

#[derive(Debug, Clone)]
pub struct ServicesZionService {
    services_db: Arc<Services>,
    services_webhood_db: Arc<ServicesWebhood>,
    services_collection_db: Arc<ServicesCollection>,
    events_db: Arc<Events>,
    private_key_path: String,
}

impl ServicesZionService {
    pub fn new(db: Arc<Database>, private_key_path: String) -> Self {
        let services_db = Arc::new(Services::new(Arc::clone(&db)));
        let services_webhood_db = Arc::new(ServicesWebhood::new(Arc::clone(&db)));
        let services_collection_db = Arc::new(ServicesCollection::new(Arc::clone(&db)));
        let events_db = Arc::new(Events::new(Arc::clone(&db)));
        Self {
            services_db,
            services_webhood_db,
            services_collection_db,
            events_db,
            private_key_path,
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

    async fn get_info_events(
        &self,
        req: Request<GetInfoEventsRequest>,
    ) -> TonicResult<Response<GetInfoEventsResponse>> {
        let filter = req.get_ref();
        let response = self
            .events_db
            .get_events_filters(
                filter.id.clone(),
                filter.client_id.clone(),
                filter.collection.clone(),
                filter.token_id.clone(),
            )
            .await;
        match response {
            Ok(events) => {
                let lst_events = events
                    .iter()
                    .map(|s| {
                        let mut response = InfoEvent::default();
                        response.id = s.id.clone();
                        response.client_id = s.client_id.clone();
                        response.status = s.status.clone();
                        response.collection = s.collection.clone();
                        response.token_id = s.token_id.clone();
                        response.created_at = s.created_at.to_string();
                        response.updated_at = s.updated_at.to_string();
                        response
                    })
                    .collect();
                let mut response = GetInfoEventsResponse::default();
                response.data = lst_events;
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn resend_noti_events(
        &self,
        req: Request<ResendNotiEventsRequest>,
    ) -> TonicResult<Response<ResendNotiEventsResponse>> {
        let filter = req.get_ref();
        let response = self.events_db.get_events_by_id(filter.id.clone()).await;
        match response {
            Ok(event) => {
                if event.client_id.is_empty() || event.collection.is_empty() 
                {
                    return Err(Status::new(
                        tonic::Code::InvalidArgument,
                        "client_id or collection or token_id is empty",
                    ));
                }
                if event.status == StatusEvent::Sent.as_str() {
                    return Err(Status::new(
                        tonic::Code::InvalidArgument,
                        "Event has been sent",
                    ));
                }
                // Resend noti
                let services_webhood = self
                    .services_webhood_db
                    .get(Some("".to_string()), Some(event.client_id.clone()))
                    .await
                    .map_err(|e| into_anyhow(e.into()))?;

                if services_webhood.id.is_empty() {
                    return Err(Status::new(
                        tonic::Code::InvalidArgument,
                        "Endpoint for Service not exist",
                    ));
                }
                //
                let body = TestSendToEndpointsRequest {
                    id: event.id.clone(),
                    payload: event.payload.clone(),
                    client_id: event.client_id.clone(),
                };
                // create signature
                let client = Client::new();
                let url = services_webhood.endpoint_url.clone();
                let data: String = format!("{}{}", event.id.clone(), event.client_id.clone());
                let file_name = self.private_key_path.clone();
                let private_key = load_private_key_from_file(&file_name).unwrap();
                let signature =
                    get_signature(&data, private_key.clone()).map_err(|e| into_anyhow(e.into()))?;
                let s: String = signature
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect();
                //
                let mut headers: HashMap<String, String> = HashMap::new();
                headers.insert("signature".to_string(), s.to_string());
                // request
                let res =
                    send_request_text(&client, Method::POST, &url, Some(&body), Some(headers))
                        .await;
                match res {
                    Ok(res) => {
                        let res = res.into_inner();
                        let res = serde_json::from_str::<TestSendToEndpointsResponse>(&res);
                        match res {
                            Ok(res) => {
                                if res.code == "1" {
                                    let _ = self
                                        .events_db
                                        .update_status(event.id.clone(), StatusEvent::Sent)
                                        .await
                                        .map_err(|e| into_anyhow(e.into()))?;
                                } else {
                                    let _ = self
                                        .events_db
                                        .update_status(event.id.clone(), StatusEvent::SentError)
                                        .await
                                        .map_err(|e| into_anyhow(e.into()))?;
                                    return Err(Status::new(
                                        tonic::Code::Unknown,
                                        "Error send to endpoint".to_string(),
                                    ));
                                }
                            }
                            Err(e) => {
                                let _ = self
                                    .events_db
                                    .update_status(event.id.clone(), StatusEvent::SentError)
                                    .await
                                    .map_err(|e| into_anyhow(e.into()))?;
                                return Err(into_anyhow(e.into()));
                            }
                        }
                    }
                    Err(e) => {
                        let _ = self
                            .events_db
                            .update_status(event.id.clone(), StatusEvent::SentError)
                            .await
                            .map_err(|e| into_anyhow(e.into()))?;
                        return Err(into_anyhow(e.into()));
                    }
                }

                let mut response = ResendNotiEventsResponse::default();
                response.id = event.id.clone();
                response.status = "Send success".to_string();
                Ok(Response::new(response))
            }
            Err(e) => Err(into_anyhow(e.into())),
        }
    }
}
