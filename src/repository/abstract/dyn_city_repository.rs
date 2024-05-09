use std::sync::Arc;

use axum::async_trait;

use crate::models::city::City;

use super::error::RepositoryResult;

pub type DynCityRepository = Arc<dyn CityRepository + Send + Sync>;

#[async_trait]
pub trait CityRepository {
    async fn get_cities_pagination(&self,skip :u32,count:u32) -> RepositoryResult<Vec<City>>;
    async fn find_city_by_id(&self,id : i32) ->  RepositoryResult<Option<City>>;
}