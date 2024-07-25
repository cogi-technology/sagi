/// Define messages for the requests and responses for ERC721
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployResponse {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOfRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOfResponse {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnerOfRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub token_id: i32,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnerOfResponse {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeTransferFromRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_id: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeTransferFromResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferFromRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_id: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferFromResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApprovedRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub token_id: i32,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApprovedResponse {
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetApprovalForAllRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub approved: bool,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetApprovalForAllResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsApprovedForAllRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsApprovedForAllResponse {
    #[prost(bool, tag = "1")]
    pub result: bool,
}
pub mod erc721_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::erc721_server::Erc721;
    use std::sync::Arc;
    /// Define messages for the requests and responses for ERC721
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeployJson {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub symbol: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub owner: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BalanceOfQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub owner: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OwnerOfQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub token_id: i32,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeTransferFromJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub token_id: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferFromJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub token_id: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApproveJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub token_id: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetApprovedQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub token_id: i32,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetApprovalForAllJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub operator: ::prost::alloc::string::String,
        #[prost(bool, tag = "3")]
        pub approved: bool,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IsApprovedForAllQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub owner: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub operator: ::prost::alloc::string::String,
    }
    async fn call_deploy(
        service: ::actix_web::web::Data<dyn Erc721 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<DeployResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            DeployJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = DeployRequest {
            name: json.name,
            symbol: json.symbol,
            owner: json.owner,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.deploy(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_balance_of(
        service: ::actix_web::web::Data<dyn Erc721 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<BalanceOfResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            BalanceOfQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = BalanceOfRequest {
            contract: query.contract,
            owner: query.owner,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.balance_of(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_owner_of(
        service: ::actix_web::web::Data<dyn Erc721 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<OwnerOfResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            OwnerOfQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = OwnerOfRequest {
            contract: query.contract,
            token_id: query.token_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.owner_of(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_safe_transfer_from(
        service: ::actix_web::web::Data<dyn Erc721 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<SafeTransferFromResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SafeTransferFromJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SafeTransferFromRequest {
            contract: json.contract,
            from: json.from,
            to: json.to,
            token_id: json.token_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.safe_transfer_from(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_transfer_from(
        service: ::actix_web::web::Data<dyn Erc721 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<TransferFromResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            TransferFromJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = TransferFromRequest {
            contract: json.contract,
            from: json.from,
            to: json.to,
            token_id: json.token_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.transfer_from(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_approve(
        service: ::actix_web::web::Data<dyn Erc721 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<ApproveResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            ApproveJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = ApproveRequest {
            contract: json.contract,
            to: json.to,
            token_id: json.token_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.approve(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_approved(
        service: ::actix_web::web::Data<dyn Erc721 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<GetApprovedResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            GetApprovedQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = GetApprovedRequest {
            contract: query.contract,
            token_id: query.token_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.get_approved(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_set_approval_for_all(
        service: ::actix_web::web::Data<dyn Erc721 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<SetApprovalForAllResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SetApprovalForAllJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SetApprovalForAllRequest {
            contract: json.contract,
            operator: json.operator,
            approved: json.approved,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.set_approval_for_all(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_is_approved_for_all(
        service: ::actix_web::web::Data<dyn Erc721 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<IsApprovedForAllResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            IsApprovedForAllQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = IsApprovedForAllRequest {
            contract: query.contract,
            owner: query.owner,
            operator: query.operator,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.is_approved_for_all(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_erc721(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn Erc721 + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config.route("/api/erc721/deploy", ::actix_web::web::post().to(call_deploy));
        config
            .route("/api/erc721/balanceOf", ::actix_web::web::get().to(call_balance_of));
        config.route("/api/erc721/ownerOf", ::actix_web::web::get().to(call_owner_of));
        config
            .route(
                "/api/erc721/safeTransferFrom",
                ::actix_web::web::post().to(call_safe_transfer_from),
            );
        config
            .route(
                "/api/erc721/transferFrom",
                ::actix_web::web::post().to(call_transfer_from),
            );
        config.route("/api/erc721/approve", ::actix_web::web::post().to(call_approve));
        config
            .route(
                "/api/erc721/getApproved",
                ::actix_web::web::get().to(call_get_approved),
            );
        config
            .route(
                "/api/erc721/setApprovalForAll",
                ::actix_web::web::post().to(call_set_approval_for_all),
            );
        config
            .route(
                "/api/erc721/isApprovedForAll",
                ::actix_web::web::get().to(call_is_approved_for_all),
            );
    }
}
/// Generated client implementations.
pub mod erc721_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Define the service for
    #[derive(Debug, Clone)]
    pub struct Erc721Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl Erc721Client<tonic::transport::Channel> {
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
    impl<T> Erc721Client<T>
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
        ) -> Erc721Client<InterceptedService<T, F>>
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
            Erc721Client::new(InterceptedService::new(inner, interceptor))
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
        pub async fn deploy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeployRequest>,
        ) -> Result<tonic::Response<super::DeployResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/erc721.ERC721/Deploy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn balance_of(
            &mut self,
            request: impl tonic::IntoRequest<super::BalanceOfRequest>,
        ) -> Result<tonic::Response<super::BalanceOfResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/erc721.ERC721/BalanceOf");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn owner_of(
            &mut self,
            request: impl tonic::IntoRequest<super::OwnerOfRequest>,
        ) -> Result<tonic::Response<super::OwnerOfResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/erc721.ERC721/OwnerOf");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn safe_transfer_from(
            &mut self,
            request: impl tonic::IntoRequest<super::SafeTransferFromRequest>,
        ) -> Result<tonic::Response<super::SafeTransferFromResponse>, tonic::Status> {
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
                "/erc721.ERC721/SafeTransferFrom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn transfer_from(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferFromRequest>,
        ) -> Result<tonic::Response<super::TransferFromResponse>, tonic::Status> {
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
                "/erc721.ERC721/TransferFrom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn approve(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveRequest>,
        ) -> Result<tonic::Response<super::ApproveResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/erc721.ERC721/Approve");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_approved(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApprovedRequest>,
        ) -> Result<tonic::Response<super::GetApprovedResponse>, tonic::Status> {
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
                "/erc721.ERC721/GetApproved",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_approval_for_all(
            &mut self,
            request: impl tonic::IntoRequest<super::SetApprovalForAllRequest>,
        ) -> Result<tonic::Response<super::SetApprovalForAllResponse>, tonic::Status> {
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
                "/erc721.ERC721/SetApprovalForAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn is_approved_for_all(
            &mut self,
            request: impl tonic::IntoRequest<super::IsApprovedForAllRequest>,
        ) -> Result<tonic::Response<super::IsApprovedForAllResponse>, tonic::Status> {
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
                "/erc721.ERC721/IsApprovedForAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod erc721_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with Erc721Server.
    #[async_trait]
    pub trait Erc721: Send + Sync + 'static {
        async fn deploy(
            &self,
            request: tonic::Request<super::DeployRequest>,
        ) -> Result<tonic::Response<super::DeployResponse>, tonic::Status>;
        async fn balance_of(
            &self,
            request: tonic::Request<super::BalanceOfRequest>,
        ) -> Result<tonic::Response<super::BalanceOfResponse>, tonic::Status>;
        async fn owner_of(
            &self,
            request: tonic::Request<super::OwnerOfRequest>,
        ) -> Result<tonic::Response<super::OwnerOfResponse>, tonic::Status>;
        async fn safe_transfer_from(
            &self,
            request: tonic::Request<super::SafeTransferFromRequest>,
        ) -> Result<tonic::Response<super::SafeTransferFromResponse>, tonic::Status>;
        async fn transfer_from(
            &self,
            request: tonic::Request<super::TransferFromRequest>,
        ) -> Result<tonic::Response<super::TransferFromResponse>, tonic::Status>;
        async fn approve(
            &self,
            request: tonic::Request<super::ApproveRequest>,
        ) -> Result<tonic::Response<super::ApproveResponse>, tonic::Status>;
        async fn get_approved(
            &self,
            request: tonic::Request<super::GetApprovedRequest>,
        ) -> Result<tonic::Response<super::GetApprovedResponse>, tonic::Status>;
        async fn set_approval_for_all(
            &self,
            request: tonic::Request<super::SetApprovalForAllRequest>,
        ) -> Result<tonic::Response<super::SetApprovalForAllResponse>, tonic::Status>;
        async fn is_approved_for_all(
            &self,
            request: tonic::Request<super::IsApprovedForAllRequest>,
        ) -> Result<tonic::Response<super::IsApprovedForAllResponse>, tonic::Status>;
    }
    /// Define the service for
    #[derive(Debug)]
    pub struct Erc721Server<T: Erc721> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Erc721> Erc721Server<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for Erc721Server<T>
    where
        T: Erc721,
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
                "/erc721.ERC721/Deploy" => {
                    #[allow(non_camel_case_types)]
                    struct DeploySvc<T: Erc721>(pub Arc<T>);
                    impl<T: Erc721> tonic::server::UnaryService<super::DeployRequest>
                    for DeploySvc<T> {
                        type Response = super::DeployResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeployRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).deploy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeploySvc(inner);
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
                "/erc721.ERC721/BalanceOf" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceOfSvc<T: Erc721>(pub Arc<T>);
                    impl<T: Erc721> tonic::server::UnaryService<super::BalanceOfRequest>
                    for BalanceOfSvc<T> {
                        type Response = super::BalanceOfResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BalanceOfRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).balance_of(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BalanceOfSvc(inner);
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
                "/erc721.ERC721/OwnerOf" => {
                    #[allow(non_camel_case_types)]
                    struct OwnerOfSvc<T: Erc721>(pub Arc<T>);
                    impl<T: Erc721> tonic::server::UnaryService<super::OwnerOfRequest>
                    for OwnerOfSvc<T> {
                        type Response = super::OwnerOfResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OwnerOfRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).owner_of(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OwnerOfSvc(inner);
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
                "/erc721.ERC721/SafeTransferFrom" => {
                    #[allow(non_camel_case_types)]
                    struct SafeTransferFromSvc<T: Erc721>(pub Arc<T>);
                    impl<
                        T: Erc721,
                    > tonic::server::UnaryService<super::SafeTransferFromRequest>
                    for SafeTransferFromSvc<T> {
                        type Response = super::SafeTransferFromResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SafeTransferFromRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).safe_transfer_from(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SafeTransferFromSvc(inner);
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
                "/erc721.ERC721/TransferFrom" => {
                    #[allow(non_camel_case_types)]
                    struct TransferFromSvc<T: Erc721>(pub Arc<T>);
                    impl<
                        T: Erc721,
                    > tonic::server::UnaryService<super::TransferFromRequest>
                    for TransferFromSvc<T> {
                        type Response = super::TransferFromResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferFromRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transfer_from(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferFromSvc(inner);
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
                "/erc721.ERC721/Approve" => {
                    #[allow(non_camel_case_types)]
                    struct ApproveSvc<T: Erc721>(pub Arc<T>);
                    impl<T: Erc721> tonic::server::UnaryService<super::ApproveRequest>
                    for ApproveSvc<T> {
                        type Response = super::ApproveResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApproveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).approve(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ApproveSvc(inner);
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
                "/erc721.ERC721/GetApproved" => {
                    #[allow(non_camel_case_types)]
                    struct GetApprovedSvc<T: Erc721>(pub Arc<T>);
                    impl<
                        T: Erc721,
                    > tonic::server::UnaryService<super::GetApprovedRequest>
                    for GetApprovedSvc<T> {
                        type Response = super::GetApprovedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetApprovedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_approved(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetApprovedSvc(inner);
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
                "/erc721.ERC721/SetApprovalForAll" => {
                    #[allow(non_camel_case_types)]
                    struct SetApprovalForAllSvc<T: Erc721>(pub Arc<T>);
                    impl<
                        T: Erc721,
                    > tonic::server::UnaryService<super::SetApprovalForAllRequest>
                    for SetApprovalForAllSvc<T> {
                        type Response = super::SetApprovalForAllResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetApprovalForAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_approval_for_all(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetApprovalForAllSvc(inner);
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
                "/erc721.ERC721/IsApprovedForAll" => {
                    #[allow(non_camel_case_types)]
                    struct IsApprovedForAllSvc<T: Erc721>(pub Arc<T>);
                    impl<
                        T: Erc721,
                    > tonic::server::UnaryService<super::IsApprovedForAllRequest>
                    for IsApprovedForAllSvc<T> {
                        type Response = super::IsApprovedForAllResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsApprovedForAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).is_approved_for_all(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsApprovedForAllSvc(inner);
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
    impl<T: Erc721> Clone for Erc721Server<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Erc721> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Erc721> tonic::server::NamedService for Erc721Server<T> {
        const NAME: &'static str = "erc721.ERC721";
    }
}
