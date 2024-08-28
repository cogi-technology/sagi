use {
    crate::{
        database::Database,
        models::State,
        schema::states,
    },
    diesel::prelude::*,
    diesel_async::RunQueryDsl,
    std::sync::Arc,
    zion_logger::warn,
};

pub struct States {
    db: Arc<Database>,
}

impl States {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get(&self, key: String) -> anyhow::Result<String> {
        let mut conn = self.db.get_connection().await;

        let ret = states::table
            .filter(states::key.eq(key))
            .select(State::as_select())
            .first(&mut conn)
            .await?;

        Ok(ret.value)
    }

    pub async fn push(&self, key: String, value: String) -> anyhow::Result<State> {
        let mut conn = self.db.get_connection().await;

        let ret = match self.get(key.clone()).await {
            Ok(_v) => {
                diesel::update(states::table.filter(states::key.eq(key.clone())))
                    .set(states::value.eq(value))
                    .returning(State::as_returning())
                    .get_result(&mut conn)
                    .await?
            }
            Err(e) => {
                warn!("States.push err:{}", e);
                self.add(key, value).await?
            }
        };
        Ok(ret)
    }

    pub async fn add(&self, key: String, value: String) -> anyhow::Result<State> {
        let s = State {
            key: key.clone(),
            value,
        };
        let mut conn = self.db.get_connection().await;

        let ret = diesel::insert_into(states::table)
            .values(s)
            .returning(State::as_returning())
            .get_result(&mut conn)
            .await?;

        Ok(ret)
    }
}
