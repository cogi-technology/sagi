use {
    crate::{
        abi::*,
        config::Config,
        db_string,
        etherman_state::EthermanState,
        model::{ParamPayloadNftCallback, PayloadNftCallback, StatusEventCallback},
        tokio_sleep_ms,
    },
    anyhow::{anyhow, Result},
    ethers::{
        types::{Address, Filter, Log, H160, H256, U256},
        utils::keccak256,
    },
    ethers_contract::EthEvent,
    ethers_providers::Middleware,
    futures::{stream::FuturesUnordered, FutureExt, StreamExt},
    openapi_logger::{debug, info, tracing, warn},
    std::{str::FromStr, sync::Arc},
    webhook_db::{
        database::Database,
        models::ServiceCollection,
        repositories::{nftevents::NftEvents, services_collection::ServicesCollection},
    },
    zion_aa::contract_wallet::client::{Client, ClientMethods},
};

pub struct Etherman {
    // erc721_contract: Arc<KogiERC721<Client>>,
    state: Arc<EthermanState>,
    event_db: Arc<NftEvents>,
    service_collection_db: Arc<ServicesCollection>,
    client: Arc<Client>,
    // event_filter: Filter,
}

impl Etherman {
    pub fn get_event_db(&self) -> Arc<NftEvents> {
        Arc::clone(&self.event_db)
    }

    pub fn get_service_collection_db(&self) -> Arc<ServicesCollection> {
        Arc::clone(&self.service_collection_db)
    }

    pub fn get_client(&self) -> Arc<Client> {
        Arc::clone(&self.client)
    }

    // pub fn get_contract(&self) -> Arc<KogiERC721<Client>> {
    //     Arc::clone(&self.erc721_contract)
    // }
}

impl Etherman {
    pub async fn init_with_default() -> Result<Self> {
        Self::init(Arc::new(Default::default()), Default::default()).await
    }

    pub async fn init(db: Arc<Database>, c: Config) -> Result<Self> {
        let state = Arc::new(EthermanState::init(Arc::clone(&db)).await?);
        let event_db = Arc::new(NftEvents::new(Arc::clone(&db)));
        let service_collection_db = Arc::new(ServicesCollection::new(Arc::clone(&db)));
        let client = Client::random_wallet(c.ethereum_rpc.as_str(), c.chain_id).await?;

        Ok(Self {
            // erc721_contract: contract,
            state,
            event_db,
            service_collection_db,
            client: Arc::new(client),
            // event_filter,
        })
    }

    #[tracing::instrument(skip_all, name = "get_logs", level = "info")]
    pub async fn get_logs(
        &self,
        event_filter: Filter,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<Log>> {
        let filter = event_filter
            .clone()
            .from_block(from_block)
            .to_block(to_block);
        let found_logs = self.client.get_logs(&filter).await?;
        info!(
            "from_block:{} to_block:{} found:{}",
            from_block,
            to_block,
            found_logs.len()
        );
        Ok(found_logs)
    }

    fn is_allow_event(&self, log: &Log, event_name: &str, address: Address) -> bool {
        let topic0 = H256::from(keccak256(event_name.as_bytes()));
        topic0 == log.topics[0] && address == log.address
    }

    /*
    event Transfer(address indexed arg0, address indexed arg1, uint256 indexed arg2)
    https://devnet-explorer.zionx.network/tx/0x6ad8158ff44cc7ac76b299dc1987747c6ae58bf44be17ea375e7d4125cd12a26
    */
    #[tracing::instrument(skip_all, name = "Transfer", level = "warn")]
    async fn on_transfer(
        &self,
        service_collection: ServiceCollection,
        erc721_contract: Arc<KogiERC721<Client>>,
        log: Log,
    ) -> Result<bool> {
        if !self.is_allow_event(
            &log,
            OnTransferFilter::abi_signature().into_owned().as_mut(),
            erc721_contract.address(),
        ) {
            return Ok(false);
        }
        let txhash = db_string!(log
            .transaction_hash
            .ok_or(anyhow!("onTransfer.txhash is None"))?);
        let (_, to, token_id): (Address, Address, U256) = erc721_contract
            .decode_event("onTransfer", log.topics, log.data)
            .unwrap();
        // get cid
        let cid: String = erc721_contract
            .method::<_, String>("tokenURI", token_id)?
            .call()
            .await?;
        // param
        let param = ParamPayloadNftCallback {
            token_id,
            txhash: txhash.clone(),
            address: erc721_contract.address(),
            owner: to,
            cid: cid.clone(),
        };

        // payload
        let payload_call_back = PayloadNftCallback {
            status: StatusEventCallback::Transfer.as_str(),
            namespace: service_collection.namespace.clone(),
            param,
        };

        //
        self.event_db
            .add(
                service_collection.client_id,
                serde_json::to_string(&payload_call_back).expect("Failed to serialize"),
                txhash,
                erc721_contract.address().to_string(),
                "onTransfer".to_string(),
                i32::try_from(token_id.low_u64()).unwrap_or_default(),
            )
            .await?;
        Ok(true)
    }

    /*
    event OnAwardItem(address indexed arg0, address indexed arg1, uint256 indexed arg2)
    https://sepolia.etherscan.io/tx/
    */
    #[tracing::instrument(skip_all, name = "OnAwardItemFilter", level = "warn")]
    async fn on_award_item_filter(
        &self,
        service_collection: ServiceCollection,
        erc721_contract: Arc<KogiERC721<Client>>,
        log: Log,
    ) -> Result<bool> {
        if !self.is_allow_event(
            &log,
            OnAwardItemFilter::abi_signature().into_owned().as_mut(),
            erc721_contract.address(),
        ) {
            return Ok(false);
        }
        let txhash = db_string!(log
            .transaction_hash
            .ok_or(anyhow!("OnAwardItem.txhash is None"))?);
        let (recipient, cid, token_id): (Address, String, U256) = erc721_contract
            .decode_event("onAwardItem", log.topics, log.data)
            .unwrap();

        // param
        let param = ParamPayloadNftCallback {
            token_id,
            txhash: txhash.clone(),
            address: erc721_contract.address(),
            owner: recipient,
            cid,
        };

        // payload
        let payload_call_back = PayloadNftCallback {
            status: StatusEventCallback::TxSuccess.as_str(),
            namespace: service_collection.namespace.clone(),
            param,
        };

        //
        self.event_db
            .add(
                service_collection.client_id,
                serde_json::to_string(&payload_call_back).expect("Failed to serialize"),
                txhash,
                erc721_contract.address().to_string(),
                "onAwardItem".to_string(),
                i32::try_from(token_id.low_u64()).unwrap_or_default(),
            )
            .await?;
        Ok(true)
    }

    /*
    event OnAwardItem(address indexed arg0, address indexed arg1, uint256 indexed arg2)
    https://sepolia.etherscan.io/tx/
    */
    #[tracing::instrument(skip_all, name = "OnBurnFilter", level = "warn")]
    async fn on_burn_filter(
        &self,
        service_collection: ServiceCollection,
        erc721_contract: Arc<KogiERC721<Client>>,
        log: Log,
    ) -> Result<bool> {
        if !self.is_allow_event(
            &log,
            OnBurnFilter::abi_signature().into_owned().as_mut(),
            erc721_contract.address(),
        ) {
            return Ok(false);
        }
        let txhash = db_string!(log
            .transaction_hash
            .ok_or(anyhow!("OnBurnFilter.txhash is None"))?);

        let token_id: U256 = erc721_contract
            .decode_event("onBurn", log.topics, log.data)
            .unwrap();

        // get owner
        let owner: Address = erc721_contract
            .method::<_, Address>("ownerOf", token_id)?
            .call()
            .await?;

        let cid: String = erc721_contract
            .method::<_, String>("tokenURI", token_id)?
            .call()
            .await?;

        // param
        let param = ParamPayloadNftCallback {
            token_id,
            txhash: txhash.clone(),
            address: erc721_contract.address(),
            owner,
            cid,
        };

        // payload
        let payload_call_back = PayloadNftCallback {
            status: StatusEventCallback::Burned.as_str(),
            namespace: service_collection.namespace.clone(),
            param,
        };
        //
        self.event_db
            .add(
                service_collection.client_id,
                serde_json::to_string(&payload_call_back).expect("Failed to serialize"),
                txhash,
                erc721_contract.address().to_string(),
                "onBurn".to_string(),
                i32::try_from(token_id.low_u64()).unwrap_or_default(),
            )
            .await?;
        Ok(true)
    }

    #[tracing::instrument(skip_all, name = "event_perform", level = "info")]
    pub async fn event_perform(
        &self,
        service_collection: ServiceCollection,
        erc721_contract: Arc<KogiERC721<Client>>,
        event_filter: Filter,
        from_block: u64,
        to_block: u64,
    ) -> Result<(i32, i32)> {
        let mut found = 0;
        let mut apply = 0;
        let mut processing_block_number = None;

        if from_block <= to_block {
            for log in self
                .get_logs(event_filter.clone(), from_block, to_block)
                .await?
            {
                found += 1;
                debug!(
                    "processing_block:{} tx:{:?}",
                    log.block_number.unwrap().as_u64(),
                    log.transaction_hash.unwrap()
                );
                processing_block_number = Some(log.block_number.unwrap().as_u64());

                if self
                    .on_transfer(
                        service_collection.clone(),
                        Arc::clone(&erc721_contract),
                        log.clone(),
                    )
                    .await?
                {
                    apply += 1;
                    continue;
                }
                if self
                    .on_burn_filter(
                        service_collection.clone(),
                        Arc::clone(&erc721_contract),
                        log.clone(),
                    )
                    .await?
                {
                    apply += 1;
                    continue;
                }
                if self
                    .on_award_item_filter(
                        service_collection.clone(),
                        Arc::clone(&erc721_contract),
                        log.clone(),
                    )
                    .await?
                {
                    apply += 1;
                    continue;
                }
            }
            if processing_block_number.is_some() {
                // self.state
                //     .set_start_block_number(processing_block_number.unwrap() + 1)
                //     .await?;
                self.service_collection_db
                    .update_start_block_number(
                        service_collection.id,
                        (processing_block_number.unwrap() + 1) as i32,
                    )
                    .await?;
            }
            info!(
                "from_block:{} to_block:{} found:{} apply:{}",
                from_block, to_block, found, apply
            );
            return Ok((found, apply));
        }
        Err(anyhow!("from_block > to_block"))
    }

    #[tracing::instrument(skip_all, name = "heartbeat_last_block", level = "warn")]
    pub async fn heartbeat_last_block(&self) -> Result<()> {
        loop {
            if let Err(e) = {
                let b = self.client.get_block_number().await?;
                info!("last_block:{}", b.as_u64());
                self.state.set_last_block_number(b.as_u64())
            } {
                warn!("err:{}", e);
            };
            tokio_sleep_ms!(10 * 1000)
        }
    }

    #[tracing::instrument(skip_all, name = "heartbeat_event", level = "warn")]
    pub async fn heartbeat_event(&self) -> Result<()> {
        loop {
            // get all contract
            let m = self.service_collection_db.get_all().await?;
            for s in m {
                let from_block = s.start_block_number;
                let to_block = self.state.get_last_block_number();
                // Contract
                let address = s.address.to_string();
                let addres_erc721: H160 = H160::from_str(&address).expect("Invalid H160 address");
                let contract = Arc::new(KogiERC721::new(addres_erc721, self.get_client()));
                let events = vec![
                    OnBurnFilter::abi_signature().into_owned(),
                    OnAwardItemFilter::abi_signature().into_owned(),
                    OnTransferFilter::abi_signature().into_owned(),
                ];
                let event_filter = Filter::new().address(contract.address()).events(events);
                // event_perform
                if let Err(e) = self
                    .event_perform(s, contract, event_filter, from_block as u64, to_block)
                    .await
                {
                    warn!(
                        "err:{}, from_block:{}, to_block:{}",
                        e, from_block, to_block
                    );
                }
            }
            tokio_sleep_ms!(10 * 1000)
        }
    }

    pub async fn heartbeat(&self) -> Result<()> {
        let tasks = FuturesUnordered::new();
        tasks.push(async move { self.heartbeat_last_block().await }.boxed());
        tasks.push(async move { self.heartbeat_event().await }.boxed());
        let _: Vec<Result<()>> = tasks.collect().await;
        Ok(())
    }
}
