use {
    crate::{
        config::TelegramAuthConfig,
        helpers::http_auth::validator_token,
        services::{
            authtelegram::AuthTelegramService, erc20::Erc20Service, erc404::Erc404Service, erc721::Erc721Service, serviceszion::ServicesZionService, zionauthorization::ZionAuthorizationService
        },
    },
    actix_files::Files,
    actix_web::{dev::ServiceRequest, web},
    actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication},
    blockscout_service_launcher::launcher::{
        self, LaunchSettings, MetricsSettings, ServerSettings,
    },
    ethers_providers::{Http, Provider},
    openapi_proto::{
        authtelegram_service::{
            auth_telegram_actix::route_auth_telegram, auth_telegram_server::AuthTelegramServer,
        },
        erc20_service::{erc20_actix::route_erc20, erc20_server::Erc20Server},
        erc404_service::{erc404_actix::route_erc404, erc404_server::Erc404Server},
        erc721_service::{erc721_actix::route_erc721, erc721_server::Erc721Server},
        serviceszion_service::{services_zion_actix::route_services_zion,services_zion_server::ServicesZionServer},
        zionauthorization_service::{
            zion_authorization_actix::route_zion_authorization,
            zion_authorization_server::ZionAuthorizationServer,
        },
    },
    std::{net::SocketAddr, sync::Arc}, zion_service_db::database::Database,
};

const SERVICE_NAME: &str = "sagi_openapi_server";

#[derive(Clone)]
pub struct ServerConfig {
    pub auth_secret: String,
    pub doc_path: String,
    // pub tls_identity: Identity,
    pub grpc_addr: SocketAddr,
    pub openapi_addr: SocketAddr,
    pub private_key_path: String,
}

#[derive(Clone)]
struct Router {
    authtelegram: Arc<AuthTelegramService>,
    zionauthorization: Arc<ZionAuthorizationService>,
    erc20: Arc<Erc20Service>,
    erc721: Arc<Erc721Service>,
    erc404: Arc<Erc404Service>,
    services_zion: Arc<ServicesZionService>,
    config: ServerConfig,
    telegram_auth_config: TelegramAuthConfig,
}

impl Router {
    pub fn grpc_router(&self) -> tonic::transport::server::Router {
        // let interceptor = AuthInterceptor::new(self.config.auth_secret.clone());
        // let service = serviceServiceServer::with_interceptor(self.service.clone(), interceptor);

        tonic::transport::Server::builder()
            // .tls_config(ServerTlsConfig::new().identity(tls_identity))
            .add_service(Erc20Server::from_arc(Arc::clone(&self.erc20)))
            .add_service(Erc721Server::from_arc(Arc::clone(&self.erc721)))
            .add_service(Erc404Server::from_arc(Arc::clone(&self.erc404)))
            .add_service(AuthTelegramServer::from_arc(Arc::clone(&self.authtelegram)))
            .add_service(ZionAuthorizationServer::from_arc(Arc::clone(
                &self.zionauthorization,
            )))
            .add_service(ServicesZionServer::from_arc(Arc::clone(
                &self.services_zion,
            )))
        // .map_err(|e| anyhow!("Failed {}", e.to_string()))
        // .unwrap()
        // .add_service(service)
    }
}

impl launcher::HttpRouter for Router {
    #[allow(clippy::clone_on_ref_ptr)]
    fn register_routes(&self, service_config: &mut actix_web::web::ServiceConfig) {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allow_any_header()
            .max_age(3600);
        let _auth_secret = self.config.auth_secret.clone();
        let _telegram_auth_config = self.telegram_auth_config.clone();
        service_config
            .service(
                web::scope("/api-doc").wrap(cors).service(
                    Files::new("", self.config.doc_path.clone())
                        .index_file("sagi-openapi.swagger.yaml"),
                ),
            )
            .service(
                web::scope("/tele")
                    .configure(|config| route_auth_telegram(config, self.authtelegram.clone())),
            )
            .service(
                web::scope("")
                    .wrap(HttpAuthentication::bearer(
                        move |req: ServiceRequest, credentials: BearerAuth| {
                            // let secret = _auth_secret.clone();
                            let value = _telegram_auth_config.clone();
                            async move { validator_token(req, credentials, value).await }
                            // async move { validator(req, credentials, secret.into()).await }
                        },
                    ))
                    .configure(|config| route_erc20(config, self.erc20.clone()))
                    .configure(|config| route_erc721(config, self.erc721.clone()))
                    .configure(|config| route_erc404(config, self.erc404.clone()))
                    .configure(|config| {
                        route_zion_authorization(config, self.zionauthorization.clone())
                    })
                    .configure(|config| route_services_zion(config, self.services_zion.clone())),
            );
    }
}

pub async fn run(
    zion_provider: Arc<Provider<Http>>,
    torii_provider: Arc<Provider<Http>>,
    server_config: ServerConfig,
    telegram_auth_config: TelegramAuthConfig,
    db:  Arc<Database>
) -> Result<(), anyhow::Error> {
    let erc20 = Arc::new(Erc20Service::new(
        Arc::clone(&zion_provider),
        Arc::clone(&torii_provider),
        telegram_auth_config.clone(),
    ));
    let erc721 = Arc::new(Erc721Service::new(
        Arc::clone(&zion_provider),
        Arc::clone(&torii_provider),
        telegram_auth_config.clone(),
    ));
    let erc404 = Arc::new(Erc404Service::new(
        Arc::clone(&zion_provider),
        Arc::clone(&torii_provider),
        telegram_auth_config.clone(),
    ));
    let authtelegram = Arc::new(AuthTelegramService::new(telegram_auth_config.clone()));
    let zionauthorization = Arc::new(ZionAuthorizationService::new(telegram_auth_config.clone()));
    let services_zion = Arc::new(ServicesZionService::new(db.clone(), server_config.private_key_path.clone()));

    let router = Router {
        authtelegram,
        erc20,
        erc721,
        erc404,
        zionauthorization,
        services_zion,
        config: server_config.clone(),
        telegram_auth_config: telegram_auth_config.clone(),
    };

    let grpc_router = router.grpc_router();
    let http_router = router;

    let mut server = ServerSettings::default();
    server.grpc.addr = server_config.grpc_addr;
    server.grpc.enabled = true;

    server.http.addr = server_config.openapi_addr;

    let metrics = MetricsSettings::default();

    let launch_settings = LaunchSettings {
        service_name: SERVICE_NAME.to_string(),
        server,
        metrics,
    };

    launcher::launch(&launch_settings, http_router, grpc_router).await
}
