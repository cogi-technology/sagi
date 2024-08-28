use {
    once_cell::sync::Lazy,
    openapi_logger::debug,
    parking_lot::RwLock,
    std::{collections::BTreeMap, path::Path, time::Duration},
    zion_aa::types::request::AuthorizationData,
};

pub static JWT_CACHE: Lazy<RwLock<BTreeMap<String, (AuthorizationData, u64)>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

pub async fn remove_expired_jwt_cache() {
    loop {
        tokio::time::sleep(Duration::from_secs(300)).await;
        let before_remove_amount = JWT_CACHE.read().len();
        {
            let mut cache = JWT_CACHE.write();
            let now = chrono::Utc::now().timestamp() as u64;
            cache.retain(|_, v| v.1 > now);
        }
        {
            let after_remove_amount = JWT_CACHE.read().len();
            debug!(
                "remove_expired_jwt: has removed: {} - after remove: {after_remove_amount}",
                before_remove_amount - after_remove_amount
            );
        }
    }
}

pub static SESSION_CACHE: Lazy<RwLock<BTreeMap<String, (u64, String)>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

pub async fn remove_expired_session_cache(session_path: &str) {
    loop {
        tokio::time::sleep(Duration::from_secs(120)).await;
        let before_remove_amount = SESSION_CACHE.read().len();
        {
            let mut cache = SESSION_CACHE.write();
            let now = chrono::Utc::now().timestamp() as u64;
            cache.retain(|_, v| {
                if now > v.0 {
                    // remove file Session
                    let session_file: String = format!("{}/session_{}", session_path, v.1);
                    let path = Path::new(&session_file);

                    if path.exists() && path.is_file() {
                        let _ = std::fs::remove_file(path).is_ok();
                    }
                    return false;
                }
                true
            });
        }
        {
            let after_remove_amount = SESSION_CACHE.read().len();
            debug!(
                "remove_expired_session_cache: has removed: {} - after remove: {after_remove_amount}",
                before_remove_amount - after_remove_amount
            );
        }
    }
}
