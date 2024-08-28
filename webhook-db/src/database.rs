use {
    anyhow::{anyhow, Result},
    diesel_async::{
        pooled_connection::{
            bb8::{Pool, PooledConnection},
            AsyncDieselConnectionManager,
        },
        AsyncPgConnection,
    },
    diesel_async_migrations::{embed_migrations, EmbeddedMigrations},
    dotenv::dotenv,
    futures::executor::block_on,
    once_cell::sync::Lazy,
};

pub static MIGRATIONS: Lazy<EmbeddedMigrations> = Lazy::new(|| embed_migrations!("./migrations"));

pub struct Database {
    pool: Pool<AsyncPgConnection>,
    url: String,
    _in_use: bool,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Database:\n\turl: {}\n\tin_use: {}",
            self.url, self._in_use
        )
    }
}

impl Default for Database {
    fn default() -> Self {
        dotenv().ok();
        Database::new(std::env::var("DATABASE_URL").unwrap())
    }
}

impl Database {
    async fn try_init_pool_and_automically_migrate(url: String) -> Result<Pool<AsyncPgConnection>> {
        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url.clone());
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .await
            .map_err(|_| anyhow!("Could not build connection pool"))?;

        let mut _conn = pool.get_owned().await?;
        MIGRATIONS.run_pending_migrations(&mut _conn).await?;

        Ok(pool)
    }

    pub fn new(url: String) -> Self {
        let pool = block_on(Self::try_init_pool_and_automically_migrate(url.clone())).unwrap();

        Database {
            pool,
            url,
            _in_use: true,
        }
    }

    pub async fn get_connection(&self) -> PooledConnection<AsyncPgConnection> {
        self.pool.get().await.unwrap()
    }
}
