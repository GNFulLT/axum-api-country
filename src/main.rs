mod config;
mod db;
mod models;
mod repository;
mod app;
mod service_register;
mod routes;

use app::{App, AppState};
use config::config::config;
use service_register::ServiceRegister;
use sqlx::postgres::PgPoolOptions;

use crate::db::init_db;

#[tokio::main]
async fn main() {
    // Initialize first
    config();

    let pool = PgPoolOptions::new()
        .max_connections(config().POOL_SIZE)
        .connect(config().POSTGRE_URL.as_str())
        .await
        .expect("Couldn't connect to database");

    let init_pool = pool.clone();
    init_db(init_pool).await;

    let app_state = AppState::from_db(pool.clone());

    App::from_port(config().PORT as u16, app_state,ServiceRegister::new(pool.clone())).run().await.expect("Server shutdown with error");
}
