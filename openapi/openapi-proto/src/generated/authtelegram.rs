/// Define messages for the requests and responses for AuthTelegram
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCodeTelegramRequest {
    #[prost(string, tag = "1")]
    pub phone_number: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendCodeTelegramResponse {
    #[prost(string, tag = "1")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub session_uuid: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInTelegramRequest {
    #[prost(string, tag = "1")]
    pub phone_number: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub session_uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub code2_fa: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInTelegramResponse {
    #[prost(string, tag = "1")]
    pub jwt: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub session_uuid: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOutTelegramRequest {
    #[prost(string, tag = "1")]
    pub session_uuid: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOutTelegramResponse {
    #[prost(string, tag = "1")]
    pub session_uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInTelegramAsBotRequest {
    #[prost(string, tag = "1")]
    pub token_auth: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInTelegramAsBotResponse {
    #[prost(string, tag = "1")]
    pub jwt: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub session_uuid: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOutTelegramAsbotRequest {
    #[prost(string, tag = "2")]
    pub session_uuid: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOutTelegramAsBotResponse {
    #[prost(string, tag = "2")]
    pub session_uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
pub mod auth_telegram_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::auth_telegram_server::AuthTelegram;
    use std::sync::Arc;
    /// Define messages for the requests and responses for AuthTelegram
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SendCodeTelegramJson {
        #[prost(string, tag = "1")]
        pub phone_number: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SignInTelegramJson {
        #[prost(string, tag = "1")]
        pub phone_number: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub session_uuid: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub code: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub code2_fa: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LogOutTelegramJson {
        #[prost(string, tag = "1")]
        pub session_uuid: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SignInTelegramAsBotJson {
        #[prost(string, tag = "1")]
        pub token_auth: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LogOutTelegramAsBotJson {
        #[prost(string, tag = "2")]
        pub session_uuid: ::prost::alloc::string::String,
    }
    async fn call_send_code_telegram(
        service: ::actix_web::web::Data<dyn AuthTelegram + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<SendCodeTelegramResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SendCodeTelegramJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SendCodeTelegramRequest {
            phone_number: json.phone_number,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.send_code_telegram(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_sign_in_telegram(
        service: ::actix_web::web::Data<dyn AuthTelegram + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<SignInTelegramResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SignInTelegramJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SignInTelegramRequest {
            phone_number: json.phone_number,
            session_uuid: json.session_uuid,
            code: json.code,
            code2_fa: json.code2_fa,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.sign_in_telegram(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_log_out_telegram(
        service: ::actix_web::web::Data<dyn AuthTelegram + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<LogOutTelegramResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            LogOutTelegramJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = LogOutTelegramRequest {
            session_uuid: json.session_uuid,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.log_out_telegram(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_sign_in_telegram_as_bot(
        service: ::actix_web::web::Data<dyn AuthTelegram + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<SignInTelegramAsBotResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SignInTelegramAsBotJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SignInTelegramAsBotRequest {
            token_auth: json.token_auth,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.sign_in_telegram_as_bot(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_log_out_telegram_as_bot(
        service: ::actix_web::web::Data<dyn AuthTelegram + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<LogOutTelegramAsBotResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            LogOutTelegramAsBotJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = LogOutTelegramAsbotRequest {
            session_uuid: json.session_uuid,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.log_out_telegram_as_bot(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_auth_telegram(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn AuthTelegram + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config
            .route(
                "/tele/api/authtelegram/sendCodeTelegram",
                ::actix_web::web::post().to(call_send_code_telegram),
            );
        config
            .route(
                "/tele/api/authtelegram/signInTelegram",
                ::actix_web::web::post().to(call_sign_in_telegram),
            );
        config
            .route(
                "/tele/api/authtelegram/logOutTelegram",
                ::actix_web::web::post().to(call_log_out_telegram),
            );
        config
            .route(
                "/tele/api/authtelegram/signInTelegramAsBot",
                ::actix_web::web::post().to(call_sign_in_telegram_as_bot),
            );
        config
            .route(
                "/tele/api/authtelegram/logOutTelegramAsBot",
                ::actix_web::web::post().to(call_log_out_telegram_as_bot),
            );
    }
}
/// Generated client implementations.
pub mod auth_telegram_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Define the service for
    #[derive(Debug, Clone)]
    pub struct AuthTelegramClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthTelegramClient<tonic::transport::Channel> {
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
    impl<T> AuthTelegramClient<T>
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
        ) -> AuthTelegramClient<InterceptedService<T, F>>
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
            AuthTelegramClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn send_code_telegram(
            &mut self,
            request: impl tonic::IntoRequest<super::SendCodeTelegramRequest>,
        ) -> Result<tonic::Response<super::SendCodeTelegramResponse>, tonic::Status> {
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
                "/authtelegram.AuthTelegram/SendCodeTelegram",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn sign_in_telegram(
            &mut self,
            request: impl tonic::IntoRequest<super::SignInTelegramRequest>,
        ) -> Result<tonic::Response<super::SignInTelegramResponse>, tonic::Status> {
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
                "/authtelegram.AuthTelegram/SignInTelegram",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn log_out_telegram(
            &mut self,
            request: impl tonic::IntoRequest<super::LogOutTelegramRequest>,
        ) -> Result<tonic::Response<super::LogOutTelegramResponse>, tonic::Status> {
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
                "/authtelegram.AuthTelegram/LogOutTelegram",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn sign_in_telegram_as_bot(
            &mut self,
            request: impl tonic::IntoRequest<super::SignInTelegramAsBotRequest>,
        ) -> Result<tonic::Response<super::SignInTelegramAsBotResponse>, tonic::Status> {
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
                "/authtelegram.AuthTelegram/SignInTelegramAsBot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn log_out_telegram_as_bot(
            &mut self,
            request: impl tonic::IntoRequest<super::LogOutTelegramAsbotRequest>,
        ) -> Result<tonic::Response<super::LogOutTelegramAsBotResponse>, tonic::Status> {
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
                "/authtelegram.AuthTelegram/LogOutTelegramAsBot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod auth_telegram_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AuthTelegramServer.
    #[async_trait]
    pub trait AuthTelegram: Send + Sync + 'static {
        async fn send_code_telegram(
            &self,
            request: tonic::Request<super::SendCodeTelegramRequest>,
        ) -> Result<tonic::Response<super::SendCodeTelegramResponse>, tonic::Status>;
        async fn sign_in_telegram(
            &self,
            request: tonic::Request<super::SignInTelegramRequest>,
        ) -> Result<tonic::Response<super::SignInTelegramResponse>, tonic::Status>;
        async fn log_out_telegram(
            &self,
            request: tonic::Request<super::LogOutTelegramRequest>,
        ) -> Result<tonic::Response<super::LogOutTelegramResponse>, tonic::Status>;
        async fn sign_in_telegram_as_bot(
            &self,
            request: tonic::Request<super::SignInTelegramAsBotRequest>,
        ) -> Result<tonic::Response<super::SignInTelegramAsBotResponse>, tonic::Status>;
        async fn log_out_telegram_as_bot(
            &self,
            request: tonic::Request<super::LogOutTelegramAsbotRequest>,
        ) -> Result<tonic::Response<super::LogOutTelegramAsBotResponse>, tonic::Status>;
    }
    /// Define the service for
    #[derive(Debug)]
    pub struct AuthTelegramServer<T: AuthTelegram> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AuthTelegram> AuthTelegramServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthTelegramServer<T>
    where
        T: AuthTelegram,
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
                "/authtelegram.AuthTelegram/SendCodeTelegram" => {
                    #[allow(non_camel_case_types)]
                    struct SendCodeTelegramSvc<T: AuthTelegram>(pub Arc<T>);
                    impl<
                        T: AuthTelegram,
                    > tonic::server::UnaryService<super::SendCodeTelegramRequest>
                    for SendCodeTelegramSvc<T> {
                        type Response = super::SendCodeTelegramResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendCodeTelegramRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_code_telegram(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendCodeTelegramSvc(inner);
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
                "/authtelegram.AuthTelegram/SignInTelegram" => {
                    #[allow(non_camel_case_types)]
                    struct SignInTelegramSvc<T: AuthTelegram>(pub Arc<T>);
                    impl<
                        T: AuthTelegram,
                    > tonic::server::UnaryService<super::SignInTelegramRequest>
                    for SignInTelegramSvc<T> {
                        type Response = super::SignInTelegramResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignInTelegramRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).sign_in_telegram(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignInTelegramSvc(inner);
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
                "/authtelegram.AuthTelegram/LogOutTelegram" => {
                    #[allow(non_camel_case_types)]
                    struct LogOutTelegramSvc<T: AuthTelegram>(pub Arc<T>);
                    impl<
                        T: AuthTelegram,
                    > tonic::server::UnaryService<super::LogOutTelegramRequest>
                    for LogOutTelegramSvc<T> {
                        type Response = super::LogOutTelegramResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogOutTelegramRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).log_out_telegram(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LogOutTelegramSvc(inner);
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
                "/authtelegram.AuthTelegram/SignInTelegramAsBot" => {
                    #[allow(non_camel_case_types)]
                    struct SignInTelegramAsBotSvc<T: AuthTelegram>(pub Arc<T>);
                    impl<
                        T: AuthTelegram,
                    > tonic::server::UnaryService<super::SignInTelegramAsBotRequest>
                    for SignInTelegramAsBotSvc<T> {
                        type Response = super::SignInTelegramAsBotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignInTelegramAsBotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).sign_in_telegram_as_bot(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignInTelegramAsBotSvc(inner);
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
                "/authtelegram.AuthTelegram/LogOutTelegramAsBot" => {
                    #[allow(non_camel_case_types)]
                    struct LogOutTelegramAsBotSvc<T: AuthTelegram>(pub Arc<T>);
                    impl<
                        T: AuthTelegram,
                    > tonic::server::UnaryService<super::LogOutTelegramAsbotRequest>
                    for LogOutTelegramAsBotSvc<T> {
                        type Response = super::LogOutTelegramAsBotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogOutTelegramAsbotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).log_out_telegram_as_bot(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LogOutTelegramAsBotSvc(inner);
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
    impl<T: AuthTelegram> Clone for AuthTelegramServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AuthTelegram> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AuthTelegram> tonic::server::NamedService for AuthTelegramServer<T> {
        const NAME: &'static str = "authtelegram.AuthTelegram";
    }
}
