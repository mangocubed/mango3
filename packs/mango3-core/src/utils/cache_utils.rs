use std::fmt::Display;

use cached::async_sync::OnceCell;
use cached::{AsyncRedisCache, IOCachedAsync};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::config::CACHE_CONFIG;

#[macro_export]
macro_rules! async_redis_cache {
    ($prefix:expr) => {
        $crate::utils::async_redis_cache($prefix)
    };
}

pub(crate) async fn async_redis_cache<K, V>(prefix: &str) -> AsyncRedisCache<K, V>
where
    K: Display + Send + Sync,
    V: DeserializeOwned + Display + Send + Serialize + Sync,
{
    AsyncRedisCache::new(format!("{prefix}:"), CACHE_CONFIG.ttl)
        .set_connection_string(&CACHE_CONFIG.redis_url)
        .set_refresh(true)
        .build()
        .await
        .expect("Could not get redis cache")
}

pub(crate) trait AsyncRedisCacheTrait<K> {
    async fn cache_remove(&self, prefix: &str, key: &K);
}

impl<K, V> AsyncRedisCacheTrait<K> for OnceCell<AsyncRedisCache<K, V>>
where
    K: Display + Send + Sync,
    V: DeserializeOwned + Display + Send + Serialize + Sync,
{
    async fn cache_remove(&self, prefix: &str, key: &K) {
        let _ = self
            .get_or_init(|| async { async_redis_cache(prefix).await })
            .await
            .cache_remove(key)
            .await;
    }
}
