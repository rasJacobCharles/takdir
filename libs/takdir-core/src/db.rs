use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use deadpool_redis::{Config as RedisConfig, Pool as RedisPool, Runtime as RedisRuntime};

pub fn create_pg_pool(database_url: &str) -> Result<Pool, deadpool_postgres::CreatePoolError> {
    let mut cfg = Config::new();
    cfg.url = Some(database_url.to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}

pub fn create_redis_pool(redis_url: &str) -> Result<RedisPool, deadpool_redis::CreatePoolError> {
    let cfg = RedisConfig::from_url(redis_url);
    cfg.create_pool(Some(RedisRuntime::Tokio1))
}

mod embedded {
    refinery::embed_migrations!("migrations");
}

pub async fn run_db_migrations(client: &mut tokio_postgres::Client) -> Result<refinery::Report, refinery::Error> {
    embedded::migrations::runner().run_async(client).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pg_pool_config() {
        let pool = create_pg_pool("postgresql://postgres:postgres@localhost:5432/takdir");
        assert!(pool.is_ok());
    }

    #[test]
    fn test_create_redis_pool_config() {
        let pool = create_redis_pool("redis://127.0.0.1:6379");
        assert!(pool.is_ok());
    }
}
