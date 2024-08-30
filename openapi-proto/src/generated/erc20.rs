/// Define messages for the requests and responses for ERC20
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(rename = "initial_supply")]
    pub initial_supply: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pin_code: ::prost::alloc::string::String,
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
pub struct TotalSupplyRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSupplyResponse {
    #[prost(string, tag = "1")]
    pub total_supply: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOfRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
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
pub struct AllowanceRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub spender: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowanceResponse {
    #[prost(string, tag = "1")]
    pub remaining_amount: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub spender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pin_code: ::prost::alloc::string::String,
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
pub struct TransferRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pin_code: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferResponse {
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
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub pin_code: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferFromResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
pub mod erc20_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::erc20_server::Erc20;
    use super::*;
    use std::sync::Arc;
    /// Define messages for the requests and responses for ERC20
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeployJson {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub symbol: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        #[serde(rename = "initial_supply")]
        pub initial_supply: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TotalSupplyQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BalanceOfQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub account: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AllowanceQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub owner: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub spender: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApproveJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub spender: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferFromJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub sender: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub pin_code: ::prost::alloc::string::String,
    }
    async fn call_deploy(
        service: ::actix_web::web::Data<dyn Erc20 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<DeployResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<DeployJson> as ::actix_web::FromRequest>::from_request(
            &http_request,
            &mut payload,
        )
        .await
        .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
        .into_inner();
        let request = DeployRequest {
            name: json.name,
            symbol: json.symbol,
            initial_supply: json.initial_supply,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.deploy(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_total_supply(
        service: ::actix_web::web::Data<dyn Erc20 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<TotalSupplyResponse>, ::actix_prost::Error> {
        let query =
            <::actix_web::web::Query<TotalSupplyQuery> as ::actix_web::FromRequest>::extract(
                &http_request,
            )
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
            .into_inner();
        let request = TotalSupplyRequest {
            contract: query.contract,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.total_supply(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_balance_of(
        service: ::actix_web::web::Data<dyn Erc20 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<BalanceOfResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<BalanceOfQuery> as ::actix_web::FromRequest>::extract(
            &http_request,
        )
        .await
        .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
        .into_inner();
        let request = BalanceOfRequest {
            contract: query.contract,
            account: query.account,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.balance_of(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_allowance(
        service: ::actix_web::web::Data<dyn Erc20 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<AllowanceResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<AllowanceQuery> as ::actix_web::FromRequest>::extract(
            &http_request,
        )
        .await
        .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
        .into_inner();
        let request = AllowanceRequest {
            contract: query.contract,
            owner: query.owner,
            spender: query.spender,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.allowance(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_approve(
        service: ::actix_web::web::Data<dyn Erc20 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<ApproveResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<ApproveJson> as ::actix_web::FromRequest>::from_request(
            &http_request,
            &mut payload,
        )
        .await
        .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
        .into_inner();
        let request = ApproveRequest {
            contract: json.contract,
            spender: json.spender,
            amount: json.amount,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.approve(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_transfer(
        service: ::actix_web::web::Data<dyn Erc20 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<TransferResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json =
            <::actix_web::web::Json<TransferJson> as ::actix_web::FromRequest>::from_request(
                &http_request,
                &mut payload,
            )
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
            .into_inner();
        let request = TransferRequest {
            contract: json.contract,
            recipient: json.recipient,
            amount: json.amount,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.transfer(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_transfer_from(
        service: ::actix_web::web::Data<dyn Erc20 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<TransferFromResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json =
            <::actix_web::web::Json<TransferFromJson> as ::actix_web::FromRequest>::from_request(
                &http_request,
                &mut payload,
            )
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
            .into_inner();
        let request = TransferFromRequest {
            contract: json.contract,
            sender: json.sender,
            recipient: json.recipient,
            amount: json.amount,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.transfer_from(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_erc20(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn Erc20 + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config.route(
            "/api/erc20/deploy",
            ::actix_web::web::post().to(call_deploy),
        );
        config.route(
            "/api/erc20/totalSupply",
            ::actix_web::web::get().to(call_total_supply),
        );
        config.route(
            "/api/erc20/balanceOf",
            ::actix_web::web::get().to(call_balance_of),
        );
        config.route(
            "/api/erc20/allowance",
            ::actix_web::web::get().to(call_allowance),
        );
        config.route(
            "/api/erc20/approve",
            ::actix_web::web::post().to(call_approve),
        );
        config.route(
            "/api/erc20/transfer",
            ::actix_web::web::post().to(call_transfer),
        );
        config.route(
            "/api/erc20/transferFrom",
            ::actix_web::web::post().to(call_transfer_from),
        );
    }
}
/// Generated client implementations.
pub mod erc20_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Define the service for ERC20
    #[derive(Debug, Clone)]
    pub struct Erc20Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl Erc20Client<tonic::transport::Channel> {
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
    impl<T> Erc20Client<T>
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
        ) -> Erc20Client<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            Erc20Client::new(InterceptedService::new(inner, interceptor))
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
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc20.ERC20/Deploy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn total_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::TotalSupplyRequest>,
        ) -> Result<tonic::Response<super::TotalSupplyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc20.ERC20/TotalSupply");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn balance_of(
            &mut self,
            request: impl tonic::IntoRequest<super::BalanceOfRequest>,
        ) -> Result<tonic::Response<super::BalanceOfResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc20.ERC20/BalanceOf");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn allowance(
            &mut self,
            request: impl tonic::IntoRequest<super::AllowanceRequest>,
        ) -> Result<tonic::Response<super::AllowanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc20.ERC20/Allowance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn approve(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveRequest>,
        ) -> Result<tonic::Response<super::ApproveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc20.ERC20/Approve");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferRequest>,
        ) -> Result<tonic::Response<super::TransferResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc20.ERC20/Transfer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn transfer_from(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferFromRequest>,
        ) -> Result<tonic::Response<super::TransferFromResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc20.ERC20/TransferFrom");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod erc20_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with Erc20Server.
    #[async_trait]
    pub trait Erc20: Send + Sync + 'static {
        async fn deploy(
            &self,
            request: tonic::Request<super::DeployRequest>,
        ) -> Result<tonic::Response<super::DeployResponse>, tonic::Status>;
        async fn total_supply(
            &self,
            request: tonic::Request<super::TotalSupplyRequest>,
        ) -> Result<tonic::Response<super::TotalSupplyResponse>, tonic::Status>;
        async fn balance_of(
            &self,
            request: tonic::Request<super::BalanceOfRequest>,
        ) -> Result<tonic::Response<super::BalanceOfResponse>, tonic::Status>;
        async fn allowance(
            &self,
            request: tonic::Request<super::AllowanceRequest>,
        ) -> Result<tonic::Response<super::AllowanceResponse>, tonic::Status>;
        async fn approve(
            &self,
            request: tonic::Request<super::ApproveRequest>,
        ) -> Result<tonic::Response<super::ApproveResponse>, tonic::Status>;
        async fn transfer(
            &self,
            request: tonic::Request<super::TransferRequest>,
        ) -> Result<tonic::Response<super::TransferResponse>, tonic::Status>;
        async fn transfer_from(
            &self,
            request: tonic::Request<super::TransferFromRequest>,
        ) -> Result<tonic::Response<super::TransferFromResponse>, tonic::Status>;
    }
    /// Define the service for ERC20
    #[derive(Debug)]
    pub struct Erc20Server<T: Erc20> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Erc20> Erc20Server<T> {
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for Erc20Server<T>
    where
        T: Erc20,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/erc20.ERC20/Deploy" => {
                    #[allow(non_camel_case_types)]
                    struct DeploySvc<T: Erc20>(pub Arc<T>);
                    impl<T: Erc20> tonic::server::UnaryService<super::DeployRequest> for DeploySvc<T> {
                        type Response = super::DeployResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc20.ERC20/TotalSupply" => {
                    #[allow(non_camel_case_types)]
                    struct TotalSupplySvc<T: Erc20>(pub Arc<T>);
                    impl<T: Erc20> tonic::server::UnaryService<super::TotalSupplyRequest> for TotalSupplySvc<T> {
                        type Response = super::TotalSupplyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TotalSupplyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).total_supply(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TotalSupplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc20.ERC20/BalanceOf" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceOfSvc<T: Erc20>(pub Arc<T>);
                    impl<T: Erc20> tonic::server::UnaryService<super::BalanceOfRequest> for BalanceOfSvc<T> {
                        type Response = super::BalanceOfResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc20.ERC20/Allowance" => {
                    #[allow(non_camel_case_types)]
                    struct AllowanceSvc<T: Erc20>(pub Arc<T>);
                    impl<T: Erc20> tonic::server::UnaryService<super::AllowanceRequest> for AllowanceSvc<T> {
                        type Response = super::AllowanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AllowanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).allowance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllowanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc20.ERC20/Approve" => {
                    #[allow(non_camel_case_types)]
                    struct ApproveSvc<T: Erc20>(pub Arc<T>);
                    impl<T: Erc20> tonic::server::UnaryService<super::ApproveRequest> for ApproveSvc<T> {
                        type Response = super::ApproveResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc20.ERC20/Transfer" => {
                    #[allow(non_camel_case_types)]
                    struct TransferSvc<T: Erc20>(pub Arc<T>);
                    impl<T: Erc20> tonic::server::UnaryService<super::TransferRequest> for TransferSvc<T> {
                        type Response = super::TransferResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).transfer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc20.ERC20/TransferFrom" => {
                    #[allow(non_camel_case_types)]
                    struct TransferFromSvc<T: Erc20>(pub Arc<T>);
                    impl<T: Erc20> tonic::server::UnaryService<super::TransferFromRequest> for TransferFromSvc<T> {
                        type Response = super::TransferFromResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferFromRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).transfer_from(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Erc20> Clone for Erc20Server<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Erc20> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Erc20> tonic::server::NamedService for Erc20Server<T> {
        const NAME: &'static str = "erc20.ERC20";
    }
}
