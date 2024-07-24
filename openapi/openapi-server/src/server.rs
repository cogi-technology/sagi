use {
    crate::{
        // interceptor::AuthInterceptor,
        services::erc20::Erc20Service,
    },
    actix_files::Files,
    actix_web::{dev::ServiceRequest, web},
    actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication},
    blockscout_service_launcher::launcher::{
        self, LaunchSettings, MetricsSettings, ServerSettings,
    },
    openapi_ethers::client::Client as EthereumClient,
    openapi_proto::erc20_service::{erc20_actix::route_erc20, erc20_server::Erc20Server},
    std::{net::SocketAddr, sync::Arc},
    tonic::transport::{Identity, ServerTlsConfig},
};

const SERVICE_NAME: &str = "sagi_openapi_server";

#[derive(Clone)]
pub struct ServerConfig {
    pub auth_secret: String,
    pub doc_path: String,
    // pub tls_identity: Identity,
    pub grpc_addr: SocketAddr,
    pub openapi_addr: SocketAddr,
}

#[derive(Clone)]
struct Router {
    erc20: Erc20Service,
    config: ServerConfig,
}

impl Router {
    pub fn grpc_router(&self) -> tonic::transport::server::Router {
        // let interceptor = AuthInterceptor::new(self.config.auth_secret.clone());
        // let service = ESimServiceServer::with_interceptor(self.esim.clone(), interceptor);

        tonic::transport::Server::builder()
            // .tls_config(ServerTlsConfig::new().identity(tls_identity))
            .add_service(Erc20Server::from_arc(Arc::new(self.erc20.clone())))
        // .map_err(|e| anyhow!("Failed {}", e.to_string()))
        // .unwrap()
        // .add_service(service)
    }
}

impl launcher::HttpRouter for Router {
    fn register_routes(&self, service_config: &mut actix_web::web::ServiceConfig) {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header();
        let _auth_secret = self.config.auth_secret.clone();

        service_config
            .service(
                web::scope("/api-doc").wrap(cors).service(
                    Files::new("", self.config.doc_path.clone())
                        .index_file("sagi-openapi.swagger.yaml"),
                ),
            )
            .service(
                web::scope("")
                    // .wrap(HttpAuthentication::bearer(
                    //     move |req: ServiceRequest, credentials: BearerAuth| {
                    //         let secret = _auth_secret.clone();
                    //         async move {
                    //             super::http_auth::validator(req, credentials, secret).await
                    //         }
                    //     },
                    // ))
                    .configure(|config| route_erc20(config, Arc::new(self.erc20.clone()))),
            );
    }
}

pub async fn run(
    rpc_client: Arc<EthereumClient>,
    server_config: ServerConfig,
) -> Result<(), anyhow::Error> {
    let erc20 = Erc20Service::new(Arc::clone(&rpc_client));

    let router = Router {
        erc20,
        config: server_config.clone(),
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
