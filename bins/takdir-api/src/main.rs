use axum::{routing::get, Router};
use std::net::SocketAddr;
use dotenvy::dotenv;
use takdir_core::db::{create_pg_pool, create_redis_pool, run_db_migrations};
use apalis::prelude::*;
use apalis_redis::RedisStorage;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecutionJob {
    pub job_id: uuid::Uuid,
    pub skill_id: uuid::Uuid,
}

impl Job for ExecutionJob {
    const NAME: &'static str = "takdir::ExecutionJob";
}

async fn execute_job_handler(job: ExecutionJob, _ctx: JobContext) -> Result<(), apalis::prelude::JobError> {
    println!("Worker processing execution job: {:?}", job);
    Ok(())
}

async fn ping_handler() -> &'static str {
    "pong"
}

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: deadpool_postgres::Pool,
    pub redis_pool: deadpool_redis::Pool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@127.0.0.1:5432/takdir".to_string());
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    println!("Initializing PostgreSQL connection pool...");
    let pg_pool = create_pg_pool(&database_url)?;
    
    // Test connection and run migrations
    println!("Running database migrations...");
    let mut pg_conn = pg_pool.get().await?;
    match run_db_migrations(&mut pg_conn).await {
        Ok(report) => println!("Migrations completed successfully: {:?}", report),
        Err(e) => eprintln!("Migration error: {:?}", e),
    }

    println!("Initializing Redis connection pool...");
    let redis_pool = create_redis_pool(&redis_url)?;
    
    // Set up apalis background worker
    println!("Setting up Apalis background queue...");
    let redis_client = redis::Client::open(redis_url)?;
    let connection_manager = redis_client.get_connection_manager().await?;
    let storage = RedisStorage::new(connection_manager);
    
    // Start the Apalis monitor
    let monitor = Monitor::new()
        .register(
            WorkerBuilder::new("takdir-execution-worker")
                .with_storage(storage.clone())
                .build_fn(execute_job_handler)
        );
    
    tokio::spawn(monitor.run());
    println!("Apalis worker started.");

    // Build API router
    let state = AppState {
        pg_pool,
        redis_pool,
    };
    
    let app = Router::new()
        .route("/ping", get(ping_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("API server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
