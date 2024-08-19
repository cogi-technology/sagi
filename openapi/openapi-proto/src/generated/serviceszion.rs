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
/// Services with NFT endpoints
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllNftEndpointForServiceRequest {}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllNftEndpointForServiceResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<NftEndpointForService>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoNftEndpointForServiceRequest {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftEndpointForService {
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
pub struct ResgiterNftEndpointForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint_url: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResgiterNftEndpointForServiceResponse {
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
pub struct UnRegisterNftEndpointForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnRegisterNftEndpointForServiceResponse {
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
pub struct UpdateNftEndpointForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint_url: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNftEndpointForServiceeResponse {
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
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendNotiNftEventsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendNotiNftEventsResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoNftEventsRequest {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub collection: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub token_id: ::core::option::Option<i32>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoNftEventsResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<InfoEventNft>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoEventNft {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tx: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub method: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub collection: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub token_id: i32,
    #[prost(string, tag = "8")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub updated_at: ::prost::alloc::string::String,
}
/// Services with endpoints Token
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllTokenEndpointForServiceRequest {}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllTokenEndpointForServiceResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<TokenEndpointForService>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoTokenEndpointForServiceRequest {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenEndpointForService {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub endpoint_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub to_transfer: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub updated_at: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResgiterTokenEndpointForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint_url: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResgiterTokenEndpointForServiceResponse {
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
pub struct UnRegisterTokenEndpointForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnRegisterTokenEndpointForServiceResponse {
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
pub struct UpdateTokenEndpointForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint_url: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTokenEndpointForServiceeResponse {
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
/// Services with Tokens
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllTokenForServiceRequest {}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllTokenForServiceResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<TokenForService>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeInfoTokenForServiceRequest {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenForService {
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
pub struct RegisterTokenForServiceRequest {
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
pub struct RegisterTokenForServiceResponse {
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
pub struct UnRegisterTokenForServiceRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnRegisterTokenForServiceResponse {
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
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendNotiTokenEventsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendNotiTokenEventsResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoTokenEventsRequest {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub token_id: ::core::option::Option<i32>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoTokenEventsResponse {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<InfoEventToken>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoEventToken {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tx: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub method: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub token: ::prost::alloc::string::String,
    #[prost(float, tag = "7")]
    pub amount: f32,
    #[prost(string, tag = "8")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub updated_at: ::prost::alloc::string::String,
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
    pub struct GetInfoNFTEndpointForServiceJson {
        #[prost(string, optional, tag = "1")]
        pub id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResgiterNFTEndpointForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub endpoint_url: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnRegisterNFTEndpointForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateNFTEndpointForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub endpoint_url: ::prost::alloc::string::String,
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
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInfoNFTEventsJson {
        #[prost(string, optional, tag = "1")]
        pub id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub client_id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub collection: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "4")]
        pub token_id: ::core::option::Option<i32>,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResendNotiNFTEventsJson {
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInfoTokenEndpointForServiceJson {
        #[prost(string, optional, tag = "1")]
        pub id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResgiterTokenEndpointForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub endpoint_url: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnRegisterTokenEndpointForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateTokenEndpointForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub endpoint_url: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInfoTokenForServiceJson {
        #[prost(string, optional, tag = "1")]
        pub id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegisterTokenForServiceJson {
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
    pub struct UnRegisterTokenForServiceJson {
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub address: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetInfoTokenEventsJson {
        #[prost(string, optional, tag = "1")]
        pub id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub client_id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub token: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "4")]
        pub token_id: ::core::option::Option<i32>,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResendNotiTokenEventsJson {
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
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
    async fn call_get_all_nft_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<
        ::actix_web::web::Json<GetAllNftEndpointForServiceResponse>,
        ::actix_prost::Error,
    > {
        let request = GetAllNftEndpointForServiceRequest {
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_all_nft_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_info_nft_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<NftEndpointForService>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            GetInfoNFTEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = GetInfoNftEndpointForServiceRequest {
            id: json.id,
            client_id: json.client_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_info_nft_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_resgiter_nft_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<ResgiterNftEndpointForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            ResgiterNFTEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = ResgiterNftEndpointForServiceRequest {
            client_id: json.client_id,
            endpoint_url: json.endpoint_url,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.resgiter_nft_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_un_register_nft_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<UnRegisterNftEndpointForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            UnRegisterNFTEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = UnRegisterNftEndpointForServiceRequest {
            client_id: json.client_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.un_register_nft_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_update_nft_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<UpdateNftEndpointForServiceeResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            UpdateNFTEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = UpdateNftEndpointForServiceRequest {
            client_id: json.client_id,
            endpoint_url: json.endpoint_url,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.update_nft_endpoint_for_service(request).await?;
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
    async fn call_get_info_nft_events(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<GetInfoNftEventsResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            GetInfoNFTEventsJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = GetInfoNftEventsRequest {
            id: json.id,
            client_id: json.client_id,
            collection: json.collection,
            token_id: json.token_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_info_nft_events(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_resend_noti_nft_events(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<ResendNotiNftEventsResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            ResendNotiNFTEventsJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = ResendNotiNftEventsRequest {
            id: json.id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.resend_noti_nft_events(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_all_token_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<
        ::actix_web::web::Json<GetAllTokenEndpointForServiceResponse>,
        ::actix_prost::Error,
    > {
        let request = GetAllTokenEndpointForServiceRequest {
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_all_token_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_info_token_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<TokenEndpointForService>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            GetInfoTokenEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = GetInfoTokenEndpointForServiceRequest {
            id: json.id,
            client_id: json.client_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_info_token_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_resgiter_token_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<ResgiterTokenEndpointForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            ResgiterTokenEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = ResgiterTokenEndpointForServiceRequest {
            client_id: json.client_id,
            endpoint_url: json.endpoint_url,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.resgiter_token_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_un_register_token_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<UnRegisterTokenEndpointForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            UnRegisterTokenEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = UnRegisterTokenEndpointForServiceRequest {
            client_id: json.client_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.un_register_token_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_update_token_endpoint_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<UpdateTokenEndpointForServiceeResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            UpdateTokenEndpointForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = UpdateTokenEndpointForServiceRequest {
            client_id: json.client_id,
            endpoint_url: json.endpoint_url,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.update_token_endpoint_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_all_token_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<
        ::actix_web::web::Json<GetAllTokenForServiceResponse>,
        ::actix_prost::Error,
    > {
        let request = GetAllTokenForServiceRequest {};
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_all_token_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_info_token_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<TokenForService>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            GetInfoTokenForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = GeInfoTokenForServiceRequest {
            id: json.id,
            client_id: json.client_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_info_token_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_register_token_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<RegisterTokenForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            RegisterTokenForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = RegisterTokenForServiceRequest {
            client_id: json.client_id,
            address: json.address,
            namespace: json.namespace,
            start_block_number: json.start_block_number,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.register_token_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_un_register_token_for_service(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<UnRegisterTokenForServiceResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            UnRegisterTokenForServiceJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = UnRegisterTokenForServiceRequest {
            client_id: json.client_id,
            address: json.address,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.un_register_token_for_service(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_info_token_events(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<GetInfoTokenEventsResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            GetInfoTokenEventsJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = GetInfoTokenEventsRequest {
            id: json.id,
            client_id: json.client_id,
            token: json.token,
            token_id: json.token_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_info_token_events(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_resend_noti_token_events(
        service: ::actix_web::web::Data<dyn ServicesZion + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<ResendNotiTokenEventsResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            ResendNotiTokenEventsJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = ResendNotiTokenEventsRequest {
            id: json.id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.resend_noti_token_events(request).await?;
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
                "/api/serviceszion/getAllNFTEndpointForService",
                ::actix_web::web::get().to(call_get_all_nft_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/getInfoNFTEndpointForService",
                ::actix_web::web::post().to(call_get_info_nft_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/resgiterNFTEndpointForService",
                ::actix_web::web::post().to(call_resgiter_nft_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/unRegisterNFTEndpointForService",
                ::actix_web::web::post().to(call_un_register_nft_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/updateNFTEndpointForService",
                ::actix_web::web::post().to(call_update_nft_endpoint_for_service),
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
                "/api/serviceszion/getInfoNFTEvents",
                ::actix_web::web::post().to(call_get_info_nft_events),
            );
        config
            .route(
                "/api/serviceszion/resendNotiNFTEvents",
                ::actix_web::web::post().to(call_resend_noti_nft_events),
            );
        config
            .route(
                "/api/serviceszion/getAllTokenEndpointForService",
                ::actix_web::web::get().to(call_get_all_token_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/getInfoTokenEndpointForService",
                ::actix_web::web::post().to(call_get_info_token_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/resgiterTokenEndpointForService",
                ::actix_web::web::post().to(call_resgiter_token_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/unRegisterTokenEndpointForService",
                ::actix_web::web::post().to(call_un_register_token_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/updateTokenEndpointForService",
                ::actix_web::web::post().to(call_update_token_endpoint_for_service),
            );
        config
            .route(
                "/api/serviceszion/getAllTokenForService",
                ::actix_web::web::get().to(call_get_all_token_for_service),
            );
        config
            .route(
                "/api/serviceszion/getInfoTokenForService",
                ::actix_web::web::post().to(call_get_info_token_for_service),
            );
        config
            .route(
                "/api/serviceszion/registerTokenForService",
                ::actix_web::web::post().to(call_register_token_for_service),
            );
        config
            .route(
                "/api/serviceszion/unRegisterTokenForService",
                ::actix_web::web::post().to(call_un_register_token_for_service),
            );
        config
            .route(
                "/api/serviceszion/getInfoTokenEvents",
                ::actix_web::web::post().to(call_get_info_token_events),
            );
        config
            .route(
                "/api/serviceszion/resendNotiTokenEvents",
                ::actix_web::web::post().to(call_resend_noti_token_events),
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
        pub async fn get_all_nft_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllNftEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllNftEndpointForServiceResponse>,
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
                "/serviceszion.ServicesZION/GetAllNFTEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_info_nft_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoNftEndpointForServiceRequest>,
        ) -> Result<tonic::Response<super::NftEndpointForService>, tonic::Status> {
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
                "/serviceszion.ServicesZION/GetInfoNFTEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resgiter_nft_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::ResgiterNftEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::ResgiterNftEndpointForServiceResponse>,
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
                "/serviceszion.ServicesZION/ResgiterNFTEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn un_register_nft_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UnRegisterNftEndpointForServiceRequest,
            >,
        ) -> Result<
            tonic::Response<super::UnRegisterNftEndpointForServiceResponse>,
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
                "/serviceszion.ServicesZION/UnRegisterNFTEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_nft_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNftEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UpdateNftEndpointForServiceeResponse>,
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
                "/serviceszion.ServicesZION/UpdateNFTEndpointForService",
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
        /// Resend Noti Events
        pub async fn get_info_nft_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoNftEventsRequest>,
        ) -> Result<tonic::Response<super::GetInfoNftEventsResponse>, tonic::Status> {
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
                "/serviceszion.ServicesZION/GetInfoNFTEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resend_noti_nft_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendNotiNftEventsRequest>,
        ) -> Result<tonic::Response<super::ResendNotiNftEventsResponse>, tonic::Status> {
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
                "/serviceszion.ServicesZION/ResendNotiNFTEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Services with endpoints
        pub async fn get_all_token_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllTokenEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllTokenEndpointForServiceResponse>,
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
                "/serviceszion.ServicesZION/GetAllTokenEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_info_token_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetInfoTokenEndpointForServiceRequest,
            >,
        ) -> Result<tonic::Response<super::TokenEndpointForService>, tonic::Status> {
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
                "/serviceszion.ServicesZION/GetInfoTokenEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resgiter_token_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ResgiterTokenEndpointForServiceRequest,
            >,
        ) -> Result<
            tonic::Response<super::ResgiterTokenEndpointForServiceResponse>,
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
                "/serviceszion.ServicesZION/ResgiterTokenEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn un_register_token_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UnRegisterTokenEndpointForServiceRequest,
            >,
        ) -> Result<
            tonic::Response<super::UnRegisterTokenEndpointForServiceResponse>,
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
                "/serviceszion.ServicesZION/UnRegisterTokenEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_token_endpoint_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTokenEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UpdateTokenEndpointForServiceeResponse>,
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
                "/serviceszion.ServicesZION/UpdateTokenEndpointForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Services with Tokens
        pub async fn get_all_token_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllTokenForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllTokenForServiceResponse>,
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
                "/serviceszion.ServicesZION/GetAllTokenForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_info_token_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GeInfoTokenForServiceRequest>,
        ) -> Result<tonic::Response<super::TokenForService>, tonic::Status> {
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
                "/serviceszion.ServicesZION/GetInfoTokenForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn register_token_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterTokenForServiceRequest>,
        ) -> Result<
            tonic::Response<super::RegisterTokenForServiceResponse>,
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
                "/serviceszion.ServicesZION/RegisterTokenForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn un_register_token_for_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UnRegisterTokenForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UnRegisterTokenForServiceResponse>,
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
                "/serviceszion.ServicesZION/UnRegisterTokenForService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resend Noti Events
        pub async fn get_info_token_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoTokenEventsRequest>,
        ) -> Result<tonic::Response<super::GetInfoTokenEventsResponse>, tonic::Status> {
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
                "/serviceszion.ServicesZION/GetInfoTokenEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resend_noti_token_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendNotiTokenEventsRequest>,
        ) -> Result<
            tonic::Response<super::ResendNotiTokenEventsResponse>,
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
                "/serviceszion.ServicesZION/ResendNotiTokenEvents",
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
        async fn get_all_nft_endpoint_for_service(
            &self,
            request: tonic::Request<super::GetAllNftEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllNftEndpointForServiceResponse>,
            tonic::Status,
        >;
        async fn get_info_nft_endpoint_for_service(
            &self,
            request: tonic::Request<super::GetInfoNftEndpointForServiceRequest>,
        ) -> Result<tonic::Response<super::NftEndpointForService>, tonic::Status>;
        async fn resgiter_nft_endpoint_for_service(
            &self,
            request: tonic::Request<super::ResgiterNftEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::ResgiterNftEndpointForServiceResponse>,
            tonic::Status,
        >;
        async fn un_register_nft_endpoint_for_service(
            &self,
            request: tonic::Request<super::UnRegisterNftEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UnRegisterNftEndpointForServiceResponse>,
            tonic::Status,
        >;
        async fn update_nft_endpoint_for_service(
            &self,
            request: tonic::Request<super::UpdateNftEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UpdateNftEndpointForServiceeResponse>,
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
        /// Resend Noti Events
        async fn get_info_nft_events(
            &self,
            request: tonic::Request<super::GetInfoNftEventsRequest>,
        ) -> Result<tonic::Response<super::GetInfoNftEventsResponse>, tonic::Status>;
        async fn resend_noti_nft_events(
            &self,
            request: tonic::Request<super::ResendNotiNftEventsRequest>,
        ) -> Result<tonic::Response<super::ResendNotiNftEventsResponse>, tonic::Status>;
        ///
        /// Services with endpoints
        async fn get_all_token_endpoint_for_service(
            &self,
            request: tonic::Request<super::GetAllTokenEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllTokenEndpointForServiceResponse>,
            tonic::Status,
        >;
        async fn get_info_token_endpoint_for_service(
            &self,
            request: tonic::Request<super::GetInfoTokenEndpointForServiceRequest>,
        ) -> Result<tonic::Response<super::TokenEndpointForService>, tonic::Status>;
        async fn resgiter_token_endpoint_for_service(
            &self,
            request: tonic::Request<super::ResgiterTokenEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::ResgiterTokenEndpointForServiceResponse>,
            tonic::Status,
        >;
        async fn un_register_token_endpoint_for_service(
            &self,
            request: tonic::Request<super::UnRegisterTokenEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UnRegisterTokenEndpointForServiceResponse>,
            tonic::Status,
        >;
        async fn update_token_endpoint_for_service(
            &self,
            request: tonic::Request<super::UpdateTokenEndpointForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UpdateTokenEndpointForServiceeResponse>,
            tonic::Status,
        >;
        /// Services with Tokens
        async fn get_all_token_for_service(
            &self,
            request: tonic::Request<super::GetAllTokenForServiceRequest>,
        ) -> Result<
            tonic::Response<super::GetAllTokenForServiceResponse>,
            tonic::Status,
        >;
        async fn get_info_token_for_service(
            &self,
            request: tonic::Request<super::GeInfoTokenForServiceRequest>,
        ) -> Result<tonic::Response<super::TokenForService>, tonic::Status>;
        async fn register_token_for_service(
            &self,
            request: tonic::Request<super::RegisterTokenForServiceRequest>,
        ) -> Result<
            tonic::Response<super::RegisterTokenForServiceResponse>,
            tonic::Status,
        >;
        async fn un_register_token_for_service(
            &self,
            request: tonic::Request<super::UnRegisterTokenForServiceRequest>,
        ) -> Result<
            tonic::Response<super::UnRegisterTokenForServiceResponse>,
            tonic::Status,
        >;
        /// Resend Noti Events
        async fn get_info_token_events(
            &self,
            request: tonic::Request<super::GetInfoTokenEventsRequest>,
        ) -> Result<tonic::Response<super::GetInfoTokenEventsResponse>, tonic::Status>;
        async fn resend_noti_token_events(
            &self,
            request: tonic::Request<super::ResendNotiTokenEventsRequest>,
        ) -> Result<
            tonic::Response<super::ResendNotiTokenEventsResponse>,
            tonic::Status,
        >;
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
                "/serviceszion.ServicesZION/GetAllNFTEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllNFTEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::GetAllNftEndpointForServiceRequest,
                    > for GetAllNFTEndpointForServiceSvc<T> {
                        type Response = super::GetAllNftEndpointForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAllNftEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_all_nft_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllNFTEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/GetInfoNFTEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoNFTEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::GetInfoNftEndpointForServiceRequest,
                    > for GetInfoNFTEndpointForServiceSvc<T> {
                        type Response = super::NftEndpointForService;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetInfoNftEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_info_nft_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoNFTEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/ResgiterNFTEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct ResgiterNFTEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::ResgiterNftEndpointForServiceRequest,
                    > for ResgiterNFTEndpointForServiceSvc<T> {
                        type Response = super::ResgiterNftEndpointForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ResgiterNftEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).resgiter_nft_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResgiterNFTEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/UnRegisterNFTEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct UnRegisterNFTEndpointForServiceSvc<T: ServicesZion>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::UnRegisterNftEndpointForServiceRequest,
                    > for UnRegisterNFTEndpointForServiceSvc<T> {
                        type Response = super::UnRegisterNftEndpointForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnRegisterNftEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).un_register_nft_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnRegisterNFTEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/UpdateNFTEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateNFTEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::UpdateNftEndpointForServiceRequest,
                    > for UpdateNFTEndpointForServiceSvc<T> {
                        type Response = super::UpdateNftEndpointForServiceeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateNftEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_nft_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateNFTEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/GetInfoNFTEvents" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoNFTEventsSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::GetInfoNftEventsRequest>
                    for GetInfoNFTEventsSvc<T> {
                        type Response = super::GetInfoNftEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInfoNftEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_info_nft_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoNFTEventsSvc(inner);
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
                "/serviceszion.ServicesZION/ResendNotiNFTEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ResendNotiNFTEventsSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::ResendNotiNftEventsRequest>
                    for ResendNotiNFTEventsSvc<T> {
                        type Response = super::ResendNotiNftEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResendNotiNftEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).resend_noti_nft_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResendNotiNFTEventsSvc(inner);
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
                "/serviceszion.ServicesZION/GetAllTokenEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllTokenEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::GetAllTokenEndpointForServiceRequest,
                    > for GetAllTokenEndpointForServiceSvc<T> {
                        type Response = super::GetAllTokenEndpointForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAllTokenEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_all_token_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllTokenEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/GetInfoTokenEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoTokenEndpointForServiceSvc<T: ServicesZion>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::GetInfoTokenEndpointForServiceRequest,
                    > for GetInfoTokenEndpointForServiceSvc<T> {
                        type Response = super::TokenEndpointForService;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetInfoTokenEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_info_token_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoTokenEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/ResgiterTokenEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct ResgiterTokenEndpointForServiceSvc<T: ServicesZion>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::ResgiterTokenEndpointForServiceRequest,
                    > for ResgiterTokenEndpointForServiceSvc<T> {
                        type Response = super::ResgiterTokenEndpointForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ResgiterTokenEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).resgiter_token_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResgiterTokenEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/UnRegisterTokenEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct UnRegisterTokenEndpointForServiceSvc<T: ServicesZion>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::UnRegisterTokenEndpointForServiceRequest,
                    > for UnRegisterTokenEndpointForServiceSvc<T> {
                        type Response = super::UnRegisterTokenEndpointForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnRegisterTokenEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .un_register_token_endpoint_for_service(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnRegisterTokenEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/UpdateTokenEndpointForService" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTokenEndpointForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::UpdateTokenEndpointForServiceRequest,
                    > for UpdateTokenEndpointForServiceSvc<T> {
                        type Response = super::UpdateTokenEndpointForServiceeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateTokenEndpointForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_token_endpoint_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateTokenEndpointForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/GetAllTokenForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllTokenForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::GetAllTokenForServiceRequest>
                    for GetAllTokenForServiceSvc<T> {
                        type Response = super::GetAllTokenForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAllTokenForServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_all_token_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllTokenForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/GetInfoTokenForService" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoTokenForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::GeInfoTokenForServiceRequest>
                    for GetInfoTokenForServiceSvc<T> {
                        type Response = super::TokenForService;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GeInfoTokenForServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_info_token_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoTokenForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/RegisterTokenForService" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterTokenForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::RegisterTokenForServiceRequest>
                    for RegisterTokenForServiceSvc<T> {
                        type Response = super::RegisterTokenForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RegisterTokenForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).register_token_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterTokenForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/UnRegisterTokenForService" => {
                    #[allow(non_camel_case_types)]
                    struct UnRegisterTokenForServiceSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<
                        super::UnRegisterTokenForServiceRequest,
                    > for UnRegisterTokenForServiceSvc<T> {
                        type Response = super::UnRegisterTokenForServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UnRegisterTokenForServiceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).un_register_token_for_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnRegisterTokenForServiceSvc(inner);
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
                "/serviceszion.ServicesZION/GetInfoTokenEvents" => {
                    #[allow(non_camel_case_types)]
                    struct GetInfoTokenEventsSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::GetInfoTokenEventsRequest>
                    for GetInfoTokenEventsSvc<T> {
                        type Response = super::GetInfoTokenEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInfoTokenEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_info_token_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInfoTokenEventsSvc(inner);
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
                "/serviceszion.ServicesZION/ResendNotiTokenEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ResendNotiTokenEventsSvc<T: ServicesZion>(pub Arc<T>);
                    impl<
                        T: ServicesZion,
                    > tonic::server::UnaryService<super::ResendNotiTokenEventsRequest>
                    for ResendNotiTokenEventsSvc<T> {
                        type Response = super::ResendNotiTokenEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResendNotiTokenEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).resend_noti_token_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResendNotiTokenEventsSvc(inner);
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
