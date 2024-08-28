use {
    crate::{
        tokio_sleep_ms,
        utils::{
            get_signature, load_private_key_from_file,
            send_request_text,
        },
    },
    anyhow::Result,
    futures::{stream::FuturesUnordered, FutureExt, StreamExt},
    openapi_logger::warn,
    openapi_proto::authtelegram_service::*,
    reqwest::{Client as ClientReqwest, Method},
    std::{collections::HashMap, sync::Arc},
    zion_logger::tracing,
    zion_service_db::{
        database::Database,
        models::StatusEvent,
        repositories::{
            nftevents::NftEvents, services_collection::ServicesCollection,
            services_collection_webhood::ServicesCollectionWebhood,
        },
    },
};

pub struct NFTWebhood {
    event_db: Arc<NftEvents>,
    service_collection_db: Arc<ServicesCollection>,
    service_webhood_db: Arc<ServicesCollectionWebhood>,
    private_key_path: String,
}

impl NFTWebhood {
    pub fn get_event_db(&self) -> Arc<NftEvents> {
        Arc::clone(&self.event_db)
    }

    pub fn get_service_collection_db(&self) -> Arc<ServicesCollection> {
        Arc::clone(&self.service_collection_db)
    }

    pub fn get_service_webhood_db(&self) -> Arc<ServicesCollectionWebhood> {
        Arc::clone(&self.service_webhood_db)
    }

    pub fn get_private_key_path(&self) -> String {
        self.private_key_path.clone()
    }
}

impl NFTWebhood {
    pub async fn init(db: Arc<Database>, private_key_path: String) -> Result<Self> {
        let event_db = Arc::new(NftEvents::new(Arc::clone(&db)));
        let service_collection_db = Arc::new(ServicesCollection::new(Arc::clone(&db)));
        let service_webhood_db = Arc::new(ServicesCollectionWebhood::new(Arc::clone(&db)));

        Ok(Self {
            event_db,
            service_collection_db,
            service_webhood_db,
            private_key_path,
        })
    }

    #[tracing::instrument(skip_all, name = "heartbeat_event", level = "warn")]
    pub async fn heartbeat_event(&self) -> Result<()> {
        let file_name = self.private_key_path.clone();
        let client = ClientReqwest::new();
        let private_key = load_private_key_from_file(&file_name).unwrap();
        loop {
            // get all contract
            let m = self.service_webhood_db.get_all().await?;
            for s in m {
                let lst_events = self
                    .event_db
                    .get_events_by_client_id(s.client_id.clone())
                    .await?;
                let mut collection_rej = "".to_string();
                let mut token_rej = None;
                for e in lst_events {
                    if e.status == StatusEvent::Sent.as_str() {
                        continue;
                    }
                    if e.collection == collection_rej && e.token_id.is_some() && Some(e.token_id) == Some(token_rej) {
                        continue;
                    }
                    // send endpoint
                    let url = s.endpoint_url.clone();
                    let body = TestSendToEndpointsRequest {
                        id: e.id.clone(),
                        payload: e.payload.clone(),
                        client_id: s.client_id.clone()
                    };
                    // create signature
                    let data: String = format!("{}{}", e.id.clone(), s.client_id.clone());
                    let signature = get_signature(&data, private_key.clone())?;
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
                                    if res.code == StatusSendToEndpointsResponse::SuccessStatus as i32 {
                                        let res = self
                                            .event_db
                                            .update_status(e.id.clone(), StatusEvent::Sent)
                                            .await;

                                        match res {
                                            Ok(_) => {}
                                            Err(err) => {
                                                warn!("{}", err.msg);
                                            }
                                        }
                                    } else {
                                        let res = self
                                            .event_db
                                            .update_status(e.id.clone(), StatusEvent::SentError)
                                            .await;
                                        match res {
                                            Ok(_) => {}
                                            Err(err) => {
                                                warn!("{}", err.msg);
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    let res = self
                                        .event_db
                                        .update_status(e.id.clone(), StatusEvent::SentError)
                                        .await;
                                    match res {
                                        Ok(_) => {}
                                        Err(err) => {
                                            warn!("{}", err.msg);
                                        }
                                    }
                                    collection_rej = e.collection.clone();
                                    token_rej = e.token_id;
                                }
                            }
                        }
                        Err(_) => {
                            let res = self
                                .event_db
                                .update_status(e.id.clone(), StatusEvent::SentError)
                                .await;
                            match res {
                                Ok(_) => {}
                                Err(err) => {
                                    warn!("{}", err.msg);
                                }
                            }
                            collection_rej = e.collection.clone();
                            token_rej = e.token_id;
                        }
                    }
                }
            }
            tokio_sleep_ms!(10 * 1000)
        }
    }

    pub async fn heartbeat(&self) -> Result<()> {
        let tasks = FuturesUnordered::new();
        tasks.push(async move { self.heartbeat_event().await }.boxed());
        let _: Vec<Result<()>> = tasks.collect().await;
        Ok(())
    }
}
