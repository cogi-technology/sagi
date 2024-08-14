/// Services
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub info: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllServicesRequest {}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllServicesResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<InfoService>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoServiceRequest {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoService {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub info: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub updated_at: ::prost::alloc::string::String,
}
/// Services with endpoints
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllEndpointForServiceRequest {}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllEndpointForServiceResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<EndpointForService>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoEndpointForServiceRequest {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointForService {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub endpoint_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub updated_at: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResgiterEndpointForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint_url: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResgiterEndpointForServiceResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub endpoint_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub updated_at: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnRegisterEndpointForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnRegisterEndpointForServiceResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub endpoint_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub updated_at: ::prost::alloc::string::String,
}
/// Services with Collections
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllCollectionForServiceRequest {}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllCollectionForServiceResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<CollectionForService>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeInfoCollectionForServiceRequest {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionForService {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub updated_at: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterCollectionForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub start_block_number: i32,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterCollectionForServiceResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub start_block_number: i32,
    #[prost(string, tag = "6")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub updated_at: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnRegisterCollectionForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnRegisterCollectionForServiceResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub start_block_number: i32,
    #[prost(string, tag = "5")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub updated_at: ::prost::alloc::string::String,
}
/// Test Endpoints
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSendToEndpointsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub payload: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSendToEndpointsResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub additional_info: ::prost::alloc::string::String,
}
pub mod services_zion_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::services_zion_server::ServicesZion;
    use std::sync::Arc;
    /// Services
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegisterServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub info: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInfoServiceJson {
        #[prost(string, optional, tag = "1")]
        pub id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInfoEndpointForServiceJson {
        #[prost(string, optional, tag = "1")]
        pub id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResgiterEndpointForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub endpoint_url: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnRegisterEndpointForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInfoCollectionForServiceJson {
        #[prost(string, optional, tag = "1")]
        pub id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegisterCollectionForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub address: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub namespace: ::prost::alloc::string::String,
        #[prost(int32, tag = "4")]
        pub start_block_number: i32,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnRegisterCollectionForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub address: ::prost::alloc::string::String,
    }
    /// Test Endpoints
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TestSendToEndpointsJson {
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub payload: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub owner: ::prost::alloc::string::String,
    }
    async fn call_register_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<InfoService>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            RegisterServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = RegisterServiceRequest {
            client_id: json.client_id,
            info: json.info,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.register_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_all_services(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<GetAllServicesResponse>, ::actix_prost::Error> {
        let request = GetAllServicesRequest {};
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_all_services(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_info_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<InfoService>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            GetInfoServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = GetInfoServiceRequest {
            id: json.id,
            client_id: json.client_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_info_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_all_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<
        ::actix_web::web::Json<GetAllEndpointForServiceResponse>,
        ::actix_prost::Error,
    > {
        let request = GetAllEndpointForServiceRequest {};
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_all_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_info_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<EndpointForService>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            GetInfoEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = GetInfoEndpointForServiceRequest {
            id: json.id,
            client_id: json.client_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_info_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_resgiter_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<ResgiterEndpointForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            ResgiterEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = ResgiterEndpointForServiceRequest {
            client_id: json.client_id,
            endpoint_url: json.endpoint_url,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.resgiter_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_un_register_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<UnRegisterEndpointForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            UnRegisterEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = UnRegisterEndpointForServiceRequest {
            client_id: json.client_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.un_register_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_all_collection_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<
        ::actix_web::web::Json<GetAllCollectionForServiceResponse>,
        ::actix_prost::Error,
    > {
        let request = GetAllCollectionForServiceRequest {
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_all_collection_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_info_collection_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<CollectionForService>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            GetInfoCollectionForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = GeInfoCollectionForServiceRequest {
            id: json.id,
            client_id: json.client_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_info_collection_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_register_collection_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<RegisterCollectionForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            RegisterCollectionForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = RegisterCollectionForServiceRequest {
            client_id: json.client_id,
            address: json.address,
            namespace: json.namespace,
            start_block_number: json.start_block_number,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.register_collection_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_un_register_collection_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<UnRegisterCollectionForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            UnRegisterCollectionForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = UnRegisterCollectionForServiceRequest {
            client_id: json.client_id,
            address: json.address,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.un_register_collection_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_test_send_to_endpoints(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<TestSendToEndpointsResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            TestSendToEndpointsJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = TestSendToEndpointsRequest {
            id: json.id,
            client_id: json.client_id,
            payload: json.payload,
            owner: json.owner,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.test_send_to_endpoints(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_services_zion(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn ServicesZion + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config
            .route(
                "/api/serviceszion/registerService",
                ::actix_web::web::post().to(call_register_service),
            );
        config
            .route(
                "/api/serviceszion/getAllServices",
                ::actix_web::web::get().to(call_get_all_services),
            );
        config
            .route(
                "/api/serviceszion/getInfoService",
                ::actix_web::web::post().to(call_get_info_service),
            );
        config
            .route(
                "/api/serviceszion/getAllEndpointForService",
                ::actix_web::web::get().to(call_get_all_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/getInfoEndpointForService",
                ::actix_web::web::post().to(call_get_info_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/resgiterEndpointForService",
                ::actix_web::web::post().to(call_resgiter_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/unRegisterEndpointForService",
                ::actix_web::web::post().to(call_un_register_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/getAllCollectionForService",
                ::actix_web::web::get().to(call_get_all_collection_for_service),
            );
        config
            .route(
                "/api/serviceszion/getInfoCollectionForService",
                ::actix_web::web::post().to(call_get_info_collection_for_service),
            );
        config
            .route(
                "/api/serviceszion/registerCollectionForService",
                ::actix_web::web::post().to(call_register_collection_for_service),
            );
        config
            .route(
                "/api/serviceszion/unRegisterCollectionForService",
                ::actix_web::web::post().to(call_un_register_collection_for_service),
            );
        config
            .route(
                "/api/serviceszion/testSendToEndpoints",
                ::actix_web::web::post().to(call_test_send_to_endpoints),
            );
    }
}
/// Generated client implementations.
pub mod services_zion_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Define the service for
    #[derive(Debug, Clone)]
    pub struct ServicesZionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServicesZionClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ServicesZionClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ServicesZionClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ServicesZionClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Services
        pub async fn register_service(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterServiceRequest>,
        ) -> Result<tonic::Response<super::InfoService>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/RegisterService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all_services(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllServicesRequest>,
        ) -> Result<tonic::Response<super::GetAllServicesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/GetAllServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_info_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoServiceRequest>,
        ) -> Result<tonic::Response<super::InfoService>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/GetInfoService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Services with endpoints
        pub async fn get_all_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllEndpointForServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/GetAllEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_info_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoEndpointForServiceRequest>,
        ) -> Result<tonic::Response<super::EndpointForService>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/GetInfoEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resgiter_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::ResgiterEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::ResgiterEndpointForServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/ResgiterEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn un_register_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UnRegisterEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UnRegisterEndpointForServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/UnRegisterEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Services with Collections
        pub async fn get_all_collection_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllCollectionForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllCollectionForServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/GetAllCollectionForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_info_collection_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GeInfoCollectionForServiceRequest>,
        ) -> Result<tonic::Response<super::CollectionForService>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/GetInfoCollectionForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn register_collection_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterCollectionForServiceRequest>,
        ) -> Result<
            tonic::Response<super::RegisterCollectionForServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/RegisterCollectionForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn un_register_collection_for_service(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UnRegisterCollectionForServiceRequest,
            >,
        ) -> Result<
            tonic::Response<super::UnRegisterCollectionForServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/UnRegisterCollectionForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn test_send_to_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::TestSendToEndpointsRequest>,
        ) -> Result<tonic::Response<super::TestSendToEndpointsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/serviceszion.ServicesZION/TestSendToEndpoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod services_zion_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ServicesZionServer.
    #[async_trait]
    pub trait ServicesZion: Send + Sync + 'static {
        /// Services
        async fn register_service(
            &self,
            request: tonic::Request<super::RegisterServiceRequest>,
        ) -> Result<tonic::Response<super::InfoService>, tonic::Status>;
        async fn get_all_services(
            &self,
            request: tonic::Request<super::GetAllServicesRequest>,
        ) -> Result<tonic::Response<super::GetAllServicesResponse>, tonic::Status>;
        async fn get_info_service(
            &self,
            request: tonic::Request<super::GetInfoServiceRequest>,
        ) -> Result<tonic::Response<super::InfoService>, tonic::Status>;
        /// Services with endpoints
        async fn get_all_endpoint_for_service(
            &self,
            request: tonic::Request<super::GetAllEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllEndpointForServiceResponse>,
            tonic::Status,
        >;
        async fn get_info_endpoint_for_service(
            &self,
            request: tonic::Request<super::GetInfoEndpointForServiceRequest>,
        ) -> Result<tonic::Response<super::EndpointForService>, tonic::Status>;
        async fn resgiter_endpoint_for_service(
            &self,
            request: tonic::Request<super::ResgiterEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::ResgiterEndpointForServiceResponse>,
            tonic::Status,
        >;
        async fn un_register_endpoint_for_service(
            &self,
            request: tonic::Request<super::UnRegisterEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UnRegisterEndpointForServiceResponse>,
            tonic::Status,
        >;
        /// Services with Collections
        async fn get_all_collection_for_service(
            &self,
            request: tonic::Request<super::GetAllCollectionForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllCollectionForServiceResponse>,
            tonic::Status,
        >;
        async fn get_info_collection_for_service(
            &self,
            request: tonic::Request<super::GeInfoCollectionForServiceRequest>,
        ) -> Result<tonic::Response<super::CollectionForService>, tonic::Status>;
        async fn register_collection_for_service(
            &self,
            request: tonic::Request<super::RegisterCollectionForServiceRequest>,
        ) -> Result<
            tonic::Response<super::RegisterCollectionForServiceResponse>,
            tonic::Status,
        >;
        async fn un_register_collection_for_service(
            &self,
            request: tonic::Request<super::UnRegisterCollectionForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UnRegisterCollectionForServiceResponse>,
            tonic::Status,
        >;
        async fn test_send_to_endpoints(
            &self,
            request: tonic::Request<super::TestSendToEndpointsRequest>,
        ) -> Result<tonic::Response<super::TestSendToEndpointsResponse>, tonic::Status>;
    }
    /// Define the service for
    #[derive(Debug)]
    pub struct ServicesZionServer<T: ServicesZion> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ServicesZion> ServicesZionServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ServicesZionServer<T>
    where
        T: ServicesZion,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/serviceszion.ServicesZION/RegisterService" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::RegisterServiceRequest>
                    for RegisterServiceSvc<T> {
                        type Response = super::InfoService;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).register_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/GetAllServices" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllServicesSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::GetAllServicesRequest>
                    for GetAllServicesSvc<T> {
                        type Response = super::GetAllServicesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAllServicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_all_services(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllServicesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/GetInfoService" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::GetInfoServiceRequest>
                    for GetInfoServiceSvc<T> {
                        type Response = super::InfoService;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInfoServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_info_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/GetAllEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::GetAllEndpointForServiceRequest>
                    for GetAllEndpointForServiceSvc<T> {
                        type Response = super::GetAllEndpointForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAllEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_all_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllEndpointForServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/GetInfoEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::GetInfoEndpointForServiceRequest,
                    > for GetInfoEndpointForServiceSvc<T> {
                        type Response = super::EndpointForService;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetInfoEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_info_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoEndpointForServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/ResgiterEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct ResgiterEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::ResgiterEndpointForServiceRequest,
                    > for ResgiterEndpointForServiceSvc<T> {
                        type Response = super::ResgiterEndpointForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ResgiterEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).resgiter_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResgiterEndpointForServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/UnRegisterEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct UnRegisterEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::UnRegisterEndpointForServiceRequest,
                    > for UnRegisterEndpointForServiceSvc<T> {
                        type Response = super::UnRegisterEndpointForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnRegisterEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).un_register_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnRegisterEndpointForServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/GetAllCollectionForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllCollectionForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::GetAllCollectionForServiceRequest,
                    > for GetAllCollectionForServiceSvc<T> {
                        type Response = super::GetAllCollectionForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAllCollectionForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_all_collection_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllCollectionForServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/GetInfoCollectionForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoCollectionForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::GeInfoCollectionForServiceRequest,
                    > for GetInfoCollectionForServiceSvc<T> {
                        type Response = super::CollectionForService;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GeInfoCollectionForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_info_collection_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoCollectionForServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/RegisterCollectionForService" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterCollectionForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::RegisterCollectionForServiceRequest,
                    > for RegisterCollectionForServiceSvc<T> {
                        type Response = super::RegisterCollectionForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RegisterCollectionForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).register_collection_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterCollectionForServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/UnRegisterCollectionForService" => {
                    #[allow(non_camel_case_types)]
                    struct UnRegisterCollectionForServiceSvc<T: ServicesZion>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::UnRegisterCollectionForServiceRequest,
                    > for UnRegisterCollectionForServiceSvc<T> {
                        type Response = super::UnRegisterCollectionForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnRegisterCollectionForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).un_register_collection_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnRegisterCollectionForServiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/serviceszion.ServicesZION/TestSendToEndpoints" => {
                    #[allow(non_camel_case_types)]
                    struct TestSendToEndpointsSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::TestSendToEndpointsRequest>
                    for TestSendToEndpointsSvc<T> {
                        type Response = super::TestSendToEndpointsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestSendToEndpointsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).test_send_to_endpoints(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TestSendToEndpointsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ServicesZion> Clone for ServicesZionServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ServicesZion> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServicesZion> tonic::server::NamedService for ServicesZionServer<T> {
        const NAME: &'static str = "serviceszion.ServicesZION";
    }
}
