use {
    crate::config::Config,
    anyhow::Result,
    std::sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    webhook_db::{database::Database, repositories::state::States},
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
