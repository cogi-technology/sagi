use {
    super::utils::get_payload_from_jwt,
    crate::error::{into_anyhow, Result as TonicResult},
    jsonwebtoken::TokenData,
    openapi_proto::{
        authtelegram_service::*,
        serviceszion_service::{services_zion_server::ServicesZion, *},
    },
    reqwest::{Client, Method},
    std::{collections::HashMap, fmt::Write, sync::Arc},
    tonic::{Request, Response, Status},
    webhook_db::{
        database::Database,
        models::StatusEvent,
        repositories::{
            nftevents::NftEvents, services::Services, services_collection::ServicesCollection,
            services_collection_webhook::ServicesCollectionWebhook, services_token::ServicesToken,
            services_token_webhook::ServicesTokenWebhook, tokenevents::TokenEvents,
        },
    },
    webhook_etherman::utils::{get_signature, load_private_key_from_file, send_request_text},
    zion_aa::types::jwt::JWTPayload,
};

#[derive(Debug, Clone)]
pub struct ServicesZionService {
    services_db: Arc<Services>,
    //
    services_collection_webhook_db: Arc<ServicesCollectionWebhook>,
    services_collection_db: Arc<ServicesCollection>,
    nft_events_db: Arc<NftEvents>,
    //
    services_token_webhook_db: Arc<ServicesTokenWebhook>,
    services_token_db: Arc<ServicesToken>,
    token_events_db: Arc<TokenEvents>,
    private_key_path: String,
}

impl ServicesZionService {
    pub fn new(db: Arc<Database>, private_key_path: String) -> Self {
        let services_db = Arc::new(Services::new(Arc::clone(&db)));
        //
        let services_collection_webhook_db =
            Arc::new(ServicesCollectionWebhook::new(Arc::clone(&db)));
        let services_collection_db = Arc::new(ServicesCollection::new(Arc::clone(&db)));
        let nft_events_db = Arc::new(NftEvents::new(Arc::clone(&db)));
        //
        let services_token_webhook_db = Arc::new(ServicesTokenWebhook::new(Arc::clone(&db)));
        let services_token_db = Arc::new(ServicesToken::new(Arc::clone(&db)));
        let token_events_db = Arc::new(TokenEvents::new(Arc::clone(&db)));
        Self {
            services_db,
            services_collection_webhook_db,
            services_collection_db,
            nft_events_db,
            services_token_webhook_db,
            services_token_db,
            token_events_db,
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
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() || req.get_ref().info.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id or info is empty",
            ));
        }
        let response = self
            .services_db
            .register_service(
                req.get_ref().client_id.clone(),
                req.get_ref().info.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = InfoService {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    info: service.info.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_all_services(
        &self,
        req: Request<GetAllServicesRequest>,
    ) -> TonicResult<Response<GetAllServicesResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self.services_db.get_all(payload.claims.sub.clone()).await;
        match response {
            Ok(service) => {
                let lst_services = service
                    .iter()
                    .map(|s| InfoService {
                        id: s.id.clone(),
                        client_id: s.client_id.clone(),
                        info: s.info.clone(),
                        created_at: s.created_at.to_string(),
                        updated_at: s.updated_at.to_string(),
                    })
                    .collect();
                let response = GetAllServicesResponse { data: lst_services };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_service(
        &self,
        req: Request<GetInfoServiceRequest>,
    ) -> TonicResult<Response<InfoService>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self
            .services_db
            .get(
                req.get_ref().id.clone(),
                req.get_ref().client_id.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = InfoService {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    info: service.info.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    // Service Endpoints NFT
    async fn get_all_nft_endpoint_for_service(
        &self,
        req: Request<GetAllNftEndpointForServiceRequest>,
    ) -> TonicResult<Response<GetAllNftEndpointForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self
            .services_collection_webhook_db
            .get_all_with_created_by(payload.claims.sub.clone())
            .await;
        match response {
            Ok(service) => {
                let lst_services = service
                    .iter()
                    .map(|s| NftEndpointForService {
                        id: s.id.clone(),
                        client_id: s.client_id.clone(),
                        endpoint_url: s.endpoint_url.clone(),
                        created_at: s.created_at.to_string(),
                        updated_at: s.updated_at.to_string(),
                    })
                    .collect();
                let response = GetAllNftEndpointForServiceResponse { data: lst_services };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_nft_endpoint_for_service(
        &self,
        req: Request<GetInfoNftEndpointForServiceRequest>,
    ) -> TonicResult<Response<NftEndpointForService>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self
            .services_collection_webhook_db
            .get(
                req.get_ref().id.clone(),
                req.get_ref().client_id.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = NftEndpointForService {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    endpoint_url: service.endpoint_url.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn resgiter_nft_endpoint_for_service(
        &self,
        req: Request<ResgiterNftEndpointForServiceRequest>,
    ) -> TonicResult<Response<ResgiterNftEndpointForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() || req.get_ref().endpoint_url.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id or endpoint_url is empty",
            ));
        }
        let response = self
            .services_collection_webhook_db
            .register_service_webhook_collection(
                req.get_ref().client_id.clone(),
                req.get_ref().endpoint_url.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = ResgiterNftEndpointForServiceResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    endpoint_url: service.endpoint_url.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn un_register_nft_endpoint_for_service(
        &self,
        req: Request<UnRegisterNftEndpointForServiceRequest>,
    ) -> TonicResult<Response<UnRegisterNftEndpointForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id is empty",
            ));
        }
        let response = self
            .services_collection_webhook_db
            .un_register_service_webhook_collection(
                req.get_ref().client_id.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = UnRegisterNftEndpointForServiceResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    endpoint_url: service.endpoint_url.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn update_nft_endpoint_for_service(
        &self,
        req: Request<UpdateNftEndpointForServiceRequest>,
    ) -> TonicResult<Response<UpdateNftEndpointForServiceeResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() || req.get_ref().endpoint_url.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id or endpoint_url is empty",
            ));
        }
        let response = self
            .services_collection_webhook_db
            .update_service_webhook_collection(
                req.get_ref().client_id.clone(),
                req.get_ref().endpoint_url.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = UpdateNftEndpointForServiceeResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    endpoint_url: service.endpoint_url.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    //Service Collections
    async fn get_all_collection_for_service(
        &self,
        req: Request<GetAllCollectionForServiceRequest>,
    ) -> TonicResult<Response<GetAllCollectionForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self
            .services_collection_db
            .get_all_with_created_by(payload.claims.sub.clone())
            .await;
        match response {
            Ok(service) => {
                let lst_services = service
                    .iter()
                    .map(|s| CollectionForService {
                        id: s.id.clone(),
                        client_id: s.client_id.clone(),
                        address: s.address.clone(),
                        created_at: s.created_at.to_string(),
                        updated_at: s.updated_at.to_string(),
                    })
                    .collect();
                let response = GetAllCollectionForServiceResponse { data: lst_services };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }
    async fn get_info_collection_for_service(
        &self,
        req: Request<GeInfoCollectionForServiceRequest>,
    ) -> TonicResult<Response<CollectionForService>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self
            .services_collection_db
            .get(
                req.get_ref().id.clone(),
                req.get_ref().client_id.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = CollectionForService {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    address: service.address.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn register_collection_for_service(
        &self,
        req: Request<RegisterCollectionForServiceRequest>,
    ) -> TonicResult<Response<RegisterCollectionForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
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
                req.get_ref().start_block_number,
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = RegisterCollectionForServiceResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    address: service.address.clone(),
                    namespace: service.namespace.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                    ..Default::default()
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn un_register_collection_for_service(
        &self,
        req: Request<UnRegisterCollectionForServiceRequest>,
    ) -> TonicResult<Response<UnRegisterCollectionForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id is empty",
            ));
        }
        let response = self
            .services_collection_db
            .un_register_service_collection(
                req.get_ref().client_id.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = UnRegisterCollectionForServiceResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    address: service.address.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                    ..Default::default()
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_nft_events(
        &self,
        req: Request<GetInfoNftEventsRequest>,
    ) -> TonicResult<Response<GetInfoNftEventsResponse>> {
        let filter = req.get_ref();
        let response = self
            .nft_events_db
            .get_events_filters(
                filter.id.clone(),
                filter.client_id.clone(),
                filter.collection.clone(),
                filter.token_id,
            )
            .await;
        match response {
            Ok(events) => {
                let lst_events = events
                    .iter()
                    .map(|s| InfoEventNft {
                        id: s.id.clone(),
                        client_id: s.client_id.clone(),
                        status: s.status.clone(),
                        collection: s.collection.clone(),
                        token_id: s.token_id.unwrap_or(0),
                        created_at: s.created_at.to_string(),
                        updated_at: s.updated_at.to_string(),
                        ..Default::default()
                    })
                    .collect();
                let response = GetInfoNftEventsResponse { data: lst_events };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn resend_noti_nft_events(
        &self,
        req: Request<ResendNotiNftEventsRequest>,
    ) -> TonicResult<Response<ResendNotiNftEventsResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let filter = req.get_ref();
        let response = self.nft_events_db.get_events_by_id(filter.id.clone()).await;
        match response {
            Ok(event) => {
                if event.client_id.is_empty() || event.collection.is_empty() {
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
                let services_webhook = self
                    .services_collection_webhook_db
                    .get(
                        Some("".to_string()),
                        Some(event.client_id.clone()),
                        payload.claims.sub.clone(),
                    )
                    .await
                    .map_err(|e| into_anyhow(e.into()))?;

                if services_webhook.id.is_empty() {
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
                let url = services_webhook.endpoint_url.clone();
                let data: String = format!("{}{}", event.id.clone(), event.client_id.clone());
                let file_name = self.private_key_path.clone();
                let private_key = load_private_key_from_file(&file_name).unwrap();
                let signature =
                    get_signature(&data, private_key.clone()).map_err(|e| into_anyhow(e.into()))?;
                // let s: String = signature
                //     .iter()
                //     .map(|byte| format!("{:02x}", byte))
                //     .collect();
                let s: String = signature.iter().fold(String::new(), |mut acc, byte| {
                    write!(acc, "{:02x}", byte).unwrap();
                    acc
                });
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
                                if res.code == StatusSendToEndpointsResponse::SuccessStatus as i32 {
                                    let _ = self
                                        .nft_events_db
                                        .update_status(event.id.clone(), StatusEvent::Sent)
                                        .await
                                        .map_err(|e| into_anyhow(e.into()))?;
                                } else {
                                    let _ = self
                                        .nft_events_db
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
                                    .nft_events_db
                                    .update_status(event.id.clone(), StatusEvent::SentError)
                                    .await
                                    .map_err(|e| into_anyhow(e.into()))?;
                                return Err(into_anyhow(e.into()));
                            }
                        }
                    }
                    Err(e) => {
                        let _ = self
                            .nft_events_db
                            .update_status(event.id.clone(), StatusEvent::SentError)
                            .await
                            .map_err(|e| into_anyhow(e.into()))?;
                        return Err(into_anyhow(e.into()));
                    }
                }

                let response = ResendNotiNftEventsResponse {
                    id: event.id.clone(),
                    status: "Send success".to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(into_anyhow(e.into())),
        }
    }

    // Service Endpoints Token
    async fn get_all_token_endpoint_for_service(
        &self,
        req: Request<GetAllTokenEndpointForServiceRequest>,
    ) -> TonicResult<Response<GetAllTokenEndpointForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self
            .services_token_webhook_db
            .get_all_with_created_by(payload.claims.sub.clone())
            .await;
        match response {
            Ok(service) => {
                let lst_services = service
                    .iter()
                    .map(|s| TokenEndpointForService {
                        id: s.id.clone(),
                        client_id: s.client_id.clone(),
                        endpoint_url: s.endpoint_url.clone(),
                        created_at: s.created_at.to_string(),
                        updated_at: s.updated_at.to_string(),
                    })
                    .collect();
                let response = GetAllTokenEndpointForServiceResponse { data: lst_services };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_token_endpoint_for_service(
        &self,
        req: Request<GetInfoTokenEndpointForServiceRequest>,
    ) -> TonicResult<Response<TokenEndpointForService>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self
            .services_collection_webhook_db
            .get(
                req.get_ref().id.clone(),
                req.get_ref().client_id.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = TokenEndpointForService {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    endpoint_url: service.endpoint_url.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn resgiter_token_endpoint_for_service(
        &self,
        req: Request<ResgiterTokenEndpointForServiceRequest>,
    ) -> TonicResult<Response<ResgiterTokenEndpointForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() || req.get_ref().endpoint_url.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id or endpoint_url is empty",
            ));
        }
        let response = self
            .services_token_webhook_db
            .register_service_webhook_token(
                req.get_ref().client_id.clone(),
                req.get_ref().endpoint_url.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = ResgiterTokenEndpointForServiceResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    endpoint_url: service.endpoint_url.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn un_register_token_endpoint_for_service(
        &self,
        req: Request<UnRegisterTokenEndpointForServiceRequest>,
    ) -> TonicResult<Response<UnRegisterTokenEndpointForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id is empty",
            ));
        }
        let response = self
            .services_token_webhook_db
            .un_register_service_webhook_token(
                req.get_ref().client_id.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = UnRegisterTokenEndpointForServiceResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    endpoint_url: service.endpoint_url.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn update_token_endpoint_for_service(
        &self,
        req: Request<UpdateTokenEndpointForServiceRequest>,
    ) -> TonicResult<Response<UpdateTokenEndpointForServiceeResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() || req.get_ref().endpoint_url.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id or endpoint_url is empty",
            ));
        }
        let response = self
            .services_token_webhook_db
            .update_service_webhook_token(
                req.get_ref().client_id.clone(),
                req.get_ref().endpoint_url.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = UpdateTokenEndpointForServiceeResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    endpoint_url: service.endpoint_url.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    //Service Tokens
    async fn get_all_token_for_service(
        &self,
        req: Request<GetAllTokenForServiceRequest>,
    ) -> TonicResult<Response<GetAllTokenForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self
            .services_token_db
            .get_all_with_created_by(payload.claims.sub.clone())
            .await;
        match response {
            Ok(service) => {
                let lst_services = service
                    .iter()
                    .map(|s| TokenForService {
                        id: s.id.clone(),
                        client_id: s.client_id.clone(),
                        address: s.address.clone(),
                        created_at: s.created_at.to_string(),
                        updated_at: s.updated_at.to_string(),
                        ..Default::default()
                    })
                    .collect();
                let response = GetAllTokenForServiceResponse { data: lst_services };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_token_for_service(
        &self,
        req: Request<GeInfoTokenForServiceRequest>,
    ) -> TonicResult<Response<TokenForService>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let response = self
            .services_token_db
            .get(
                req.get_ref().id.clone(),
                req.get_ref().client_id.clone(),
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = TokenForService {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    address: service.address.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                    ..Default::default()
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn register_token_for_service(
        &self,
        req: Request<RegisterTokenForServiceRequest>,
    ) -> TonicResult<Response<RegisterTokenForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() || req.get_ref().address.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id or address is empty",
            ));
        }
        let response = self
            .services_token_db
            .register_service_token(
                req.get_ref().client_id.clone(),
                req.get_ref().address.clone(),
                req.get_ref().to_transfer.clone(),
                req.get_ref().namespace.clone(),
                req.get_ref().start_block_number,
                payload.claims.sub.clone(),
            )
            .await;
        match response {
            Ok(service) => {
                let response = RegisterTokenForServiceResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    address: service.address.clone(),
                    to_transfer: service.to_transfer.clone(),
                    namespace: service.namespace.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                    ..Default::default()
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn un_register_token_for_service(
        &self,
        req: Request<UnRegisterTokenForServiceRequest>,
    ) -> TonicResult<Response<UnRegisterTokenForServiceResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        if req.get_ref().client_id.is_empty() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "client_id is empty",
            ));
        }
        let response = self
            .services_token_db
            .un_register_service_token(req.get_ref().client_id.clone(), payload.claims.sub.clone())
            .await;
        match response {
            Ok(service) => {
                let response = UnRegisterTokenForServiceResponse {
                    id: service.id.clone(),
                    client_id: service.client_id.clone(),
                    address: service.address.clone(),
                    to_transfer: service.to_transfer.clone(),
                    created_at: service.created_at.to_string(),
                    updated_at: service.updated_at.to_string(),
                    ..Default::default()
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn get_info_token_events(
        &self,
        req: Request<GetInfoTokenEventsRequest>,
    ) -> TonicResult<Response<GetInfoTokenEventsResponse>> {
        let filter = req.get_ref();
        let response = self
            .token_events_db
            .get_events_filters(
                filter.id.clone(),
                filter.client_id.clone(),
                filter.token_address.clone(),
            )
            .await;
        match response {
            Ok(events) => {
                let lst_events = events
                    .iter()
                    .map(|s| InfoEventToken {
                        id: s.id.clone(),
                        client_id: s.client_id.clone(),
                        status: s.status.clone(),
                        token_address: s.token_address.clone(),
                        amount: s.amount,
                        created_at: s.created_at.to_string(),
                        updated_at: s.updated_at.to_string(),
                        ..Default::default()
                    })
                    .collect();
                let response = GetInfoTokenEventsResponse { data: lst_events };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::new(tonic::Code::Unknown, e.msg)),
        }
    }

    async fn resend_noti_token_events(
        &self,
        req: Request<ResendNotiTokenEventsRequest>,
    ) -> TonicResult<Response<ResendNotiTokenEventsResponse>> {
        // Get Sub User
        let metadata = req.metadata();
        let payload: TokenData<JWTPayload> = match get_payload_from_jwt(metadata).await {
            Ok(p) => p,
            Err(e) => return Err(Status::new(tonic::Code::Unknown, e.to_string())),
        };
        //
        let filter = req.get_ref();
        let response = self
            .token_events_db
            .get_events_by_id(filter.id.clone())
            .await;
        match response {
            Ok(event) => {
                if event.client_id.is_empty() || event.token_address.is_empty() {
                    return Err(Status::new(
                        tonic::Code::InvalidArgument,
                        "client_id or token_address or token_id is empty",
                    ));
                }
                if event.status == StatusEvent::Sent.as_str() {
                    return Err(Status::new(
                        tonic::Code::InvalidArgument,
                        "Event has been sent",
                    ));
                }
                // Resend noti
                let services_token_webhook = self
                    .services_token_webhook_db
                    .get(
                        Some("".to_string()),
                        Some(event.client_id.clone()),
                        payload.claims.sub.clone(),
                    )
                    .await
                    .map_err(|e| into_anyhow(e.into()))?;

                if services_token_webhook.id.is_empty() {
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
                let url = services_token_webhook.endpoint_url.clone();
                let data: String = format!("{}{}", event.id.clone(), event.client_id.clone());
                let file_name = self.private_key_path.clone();
                let private_key = load_private_key_from_file(&file_name).unwrap();
                let signature =
                    get_signature(&data, private_key.clone()).map_err(|e| into_anyhow(e.into()))?;
                // let s: String = signature
                //     .iter()
                //     .map(|byte| format!("{:02x}", byte))
                //     .collect();
                let s: String = signature.iter().fold(String::new(), |mut acc, byte| {
                    write!(acc, "{:02x}", byte).unwrap();
                    acc
                });

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
                                if res.code == StatusSendToEndpointsResponse::SuccessStatus as i32 {
                                    let _ = self
                                        .token_events_db
                                        .update_status(event.id.clone(), StatusEvent::Sent)
                                        .await
                                        .map_err(|e| into_anyhow(e.into()))?;
                                } else {
                                    let _ = self
                                        .token_events_db
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
                                    .token_events_db
                                    .update_status(event.id.clone(), StatusEvent::SentError)
                                    .await
                                    .map_err(|e| into_anyhow(e.into()))?;
                                return Err(into_anyhow(e.into()));
                            }
                        }
                    }
                    Err(e) => {
                        let _ = self
                            .token_events_db
                            .update_status(event.id.clone(), StatusEvent::SentError)
                            .await
                            .map_err(|e| into_anyhow(e.into()))?;
                        return Err(into_anyhow(e.into()));
                    }
                }

                let response = ResendNotiTokenEventsResponse {
                    id: event.id.clone(),
                    status: "Send success".to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(into_anyhow(e.into())),
        }
    }
}
