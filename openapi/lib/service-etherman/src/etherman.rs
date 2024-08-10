use {
    super::abi::*,
    crate::{
        config::Config,
        db_string,
        tokio_sleep_ms,
    },
    anyhow::{
        anyhow,
        Result,
    },
    ethers::{
        middleware::SignerMiddleware,
        providers::Provider,
        signers::{
            LocalWallet,
            Signer,
        },
        types::{
            Address,
            Filter,
            Log,
            H256,
            U256,
        },
        utils::keccak256,
    },
    ethers_contract::EthEvent,
    ethers_providers::{
        Http,
        Middleware,
        ProviderExt,
    },
    futures::{
        stream::FuturesUnordered,
        FutureExt,
        StreamExt,
    },
    std::sync::{
        atomic::{
            AtomicU64,
            Ordering,
        },
        Arc,
    },
    zion_service_db::{
        database::Database,
        models::Status,
        repositories::{
            bill::Bills,
            state::States,
        },
    },
    zion_logger::{
        debug,
        info,
        tracing,
        warn,
    },
};

pub struct EthermanState {
    db: Arc<States>,
    start_block_number: AtomicU64,
    last_block_number: AtomicU64,
}

impl EthermanState {
    pub async fn init(db: Arc<Database>, c: Config) -> Result<Self> {
        let db = Arc::new(States::new(db));
        let b = db
            .get("ethermanstate.start_block_number".into())
            .await
            .unwrap_or(c.start_block_number.to_string())
            .parse::<u64>()?;
        println!("annnnnnn start_block_number:{}", b);
        Ok(Self {
            db,
            start_block_number: b.into(),
            last_block_number: b.into(),
        })
    }

    pub async fn set_start_block_number(&self, v: u64) -> Result<()> {
        self.db
            .push("ethermanstate.start_block_number".into(), v.to_string())
            .await?;
        self.start_block_number.store(v, Ordering::Release);
        Ok(())
    }

    pub fn get_start_block_number(&self) -> u64 {
        self.start_block_number.load(Ordering::Relaxed)
    }

    pub fn set_last_block_number(&self, v: u64) -> Result<()> {
        self.last_block_number.store(v, Ordering::Release);
        Ok(())
    }

    pub fn get_last_block_number(&self) -> u64 {
        self.last_block_number.load(Ordering::Relaxed)
    }
}

type Client = SignerMiddleware<Arc<Provider<Http>>, LocalWallet>;

pub struct Etherman {
    rewarder_contract: Arc<RewardPool<Client>>,
    state: Arc<EthermanState>,
    bills: Arc<Bills>,
    client: Arc<Client>,
    event_filter: Filter,
}

impl Etherman {
    pub fn get_bills(&self) -> Arc<Bills> {
        Arc::clone(&self.bills)
    }

    pub fn get_client(&self) -> Arc<Client> {
        Arc::clone(&self.client)
    }

    pub fn get_contract(&self) -> Arc<RewardPool<Client>> {
        Arc::clone(&self.rewarder_contract)
    }
}

impl Etherman {
    pub async fn init_with_default() -> Result<Self> {
        Self::init(
            Arc::new(Default::default()),
            "test".into(),
            Default::default(),
        )
        .await
    }

    pub async fn init(db: Arc<Database>, key_password: String, c: Config) -> Result<Self> {
        let provider = Arc::new(Provider::<Http>::connect(c.ethereum_rpc.as_str()).await);

        let state = Arc::new(EthermanState::init(Arc::clone(&db), c.clone()).await?);
        let bills = Arc::new(Bills::new(Arc::clone(&db)));
        let operator = LocalWallet::decrypt_keystore(c.operator_keystore, key_password)
            .map_err(|x| anyhow!("decrypt_keystore failed err:{}", x))?;

        let client = Arc::new(SignerMiddleware::new(
            Arc::clone(&provider),
            operator.with_chain_id(c.chain_id),
        ));
        let contract = Arc::new(RewardPool::new(c.contract_address, Arc::clone(&client)));

        let events = vec![
            PaidOnBillFilter::abi_signature().into_owned(),
            RewardedOnBillFilter::abi_signature().into_owned(),
        ];
        let event_filter = Filter::new().address(contract.address()).events(events);

        Ok(Self {
            rewarder_contract: contract,
            state,
            bills,
            client,
            event_filter,
        })
    }

    #[tracing::instrument(skip_all, name = "get_logs", level = "info")]
    pub async fn get_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<Log>> {
        // let _step: usize = 1000;
        // let mut logs: Vec<Log> = Vec::new();

        // for sub_range in split_range(from_block, to_block, _step) {
        //     let filter = self
        //         .event_filter
        //         .clone()
        //         .from_block(sub_range.0)
        //         .to_block(sub_range.1);
        //     let found_logs = self.client.get_logs(&filter).await?;
        //     info!(
        //         "from_block:{} to_block:{} found:{}",
        //         sub_range.0,
        //         sub_range.1,
        //         found_logs.len()
        //     );
        //     logs.extend(found_logs.into_iter());
        // }
        // Ok(logs)
        let filter = self
            .event_filter
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
    event PaidOnBill(address indexed bill_customer,uint256 indexed bill_id,uint256 bill_quality)
    https://sepolia.etherscan.io/tx/0x353877fb193c161e656f9928a12aac5281812b12774c22a3af7eaddad0306442
    */
    #[tracing::instrument(skip_all, name = "on_paid_event", level = "warn")]
    async fn on_paid_event(&self, log: Log) -> Result<bool> {
        if !self.is_allow_event(
            &log,
            PaidOnBillFilter::abi_signature().into_owned().as_mut(),
            self.rewarder_contract.address(),
        ) {
            return Ok(false);
        }
        let txhash = db_string!(log
            .transaction_hash
            .ok_or(anyhow!("on_paid_event.txhash is None"))?);
        let (bill_customer, bill_id, bill_quality): (Address, String, U256) = self
            .rewarder_contract
            .decode_event("PaidOnBill", log.topics, log.data)
            .unwrap();

        debug!(
            "bill_customer:{bill_customer:?} bill_id:{bill_id} bill_quality:{:?}",
            bill_quality
        );

        if let Err(e) = self
            .bills
            .add(
                bill_id.clone(),
                db_string!(bill_customer),
                bill_quality.to_string(),
                txhash.clone(),
            )
            .await
        {
            let failed_msg = e.msg;
            warn!("{failed_msg} txhash:{:?}", txhash);
            return Ok(false);
        };

        let is_rewarded = self
            .rewarder_contract
            .method::<String, bool>("isRewardedBill", bill_id.clone())?
            .call()
            .await?;
        if !is_rewarded {
            let function_call = self
                .rewarder_contract
                .method::<(Address, String, U256), H256>(
                    "rewardOnBill",
                    (bill_customer, bill_id.clone(), bill_quality),
                )?
                .legacy();
            let rewarded_txhash = function_call.send().await?.tx_hash();
            debug!("rewarded_txhash: {:?}", rewarded_txhash);
        } else {
            debug!("bill rewarded: {}", bill_id);
        }

        Ok(true)
    }

    /*
    event RewardedOnBill(address indexed bill_customer, string indexed bill_id, uint256 rewarded_amount)
    https://sepolia.etherscan.io/tx/0xe0a7acb3e9d6eae0d8694b29fe757abf326814d4f1607eb1e77ca43d81a77a58
    */
    #[tracing::instrument(skip_all, name = "on_rewarded_event", level = "warn")]
    async fn on_rewarded_event(&self, log: Log) -> Result<bool> {
        if !self.is_allow_event(
            &log,
            RewardedOnBillFilter::abi_signature().into_owned().as_mut(),
            self.rewarder_contract.address(),
        ) {
            return Ok(false);
        }
        let txhash = db_string!(log
            .transaction_hash
            .ok_or(anyhow!("on_rewarded_event.txhash is None"))?);
        let (bill_customer, bill_id, rewarded_amount) = self
            .rewarder_contract
            .decode_event::<(Address, String, U256)>("RewardedOnBill", log.topics, log.data)
            .unwrap();
        debug!(
            "bill_customer:{bill_customer:?} amount:{} rewarded_amount:{:?} txhash:{:?}",
            &bill_id, rewarded_amount, txhash
        );

        if let Err(e) = self
            .bills
            .update_rewarded(
                bill_id,
                Some(txhash.clone()),
                Some(rewarded_amount.to_string()),
            )
            .await
        {
            if e.status == Status::AlreadyRewarded as i32 {
                warn!("Status::AlreadyRewarded txhash:{:?}", txhash);
                return Ok(false);
            }
        };

        Ok(true)
    }

    #[tracing::instrument(skip_all, name = "event_perform", level = "info")]
    pub async fn event_perform(&self, from_block: u64, to_block: u64) -> Result<(i32, i32)> {
        let mut found = 0;
        let mut apply = 0;
        let mut processing_block_number = None;

        if from_block <= to_block {
            for log in self.get_logs(from_block, to_block).await? {
                found += 1;
                debug!(
                    "processing_block:{} tx:{:?}",
                    log.block_number.unwrap().as_u64(),
                    log.transaction_hash.unwrap()
                );
                processing_block_number = Some(log.block_number.unwrap().as_u64());

                if self.on_paid_event(log.clone()).await? {
                    apply += 1;
                    continue;
                }
                if self.on_rewarded_event(log.clone()).await? {
                    apply += 1;
                    continue;
                }
            }
            if processing_block_number.is_some() {
                self.state
                    .set_start_block_number(processing_block_number.unwrap() + 1)
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
            let from_block = self.state.get_start_block_number();
            let to_block = self.state.get_last_block_number();
            if let Err(e) = self.event_perform(from_block, to_block).await {
                warn!(
                    "err:{}, from_block:{}, to_block:{}",
                    e, from_block, to_block
                );
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
