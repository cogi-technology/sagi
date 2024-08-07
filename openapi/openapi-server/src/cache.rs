use std::{collections::BTreeMap, time::Duration};

use once_cell::sync::Lazy;
use openapi_logger::debug;
use parking_lot::RwLock;
use zion_aa::types::request::AuthorizationData;

pub static JWT_CACHE: Lazy<RwLock<BTreeMap<String, (AuthorizationData, u64)>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

pub async fn remove_expired_jwt_cache() {
    loop {
        let before_remove_amount = JWT_CACHE.read().len();
        {
            let mut cache = JWT_CACHE.write();
            let now = chrono::Utc::now().timestamp() as u64;
            cache.retain(|_, v| v.1 > now);
        }
        let after_remove_amount = JWT_CACHE.read().len();
        debug!(
            "remove_expired_jwt: has removed: {} - after remove: {after_remove_amount}",
            before_remove_amount - after_remove_amount
        );
        tokio::time::sleep(Duration::from_secs(300)).await;
    }
}
