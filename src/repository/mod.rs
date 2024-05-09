use sqlx::{Pool, Postgres};

pub mod city_repository;
pub mod r#abstract;
pub mod country_repository;
pub mod state_repository;
mod base_postgres_repository;
pub type PostgresPool = Pool<Postgres>;
