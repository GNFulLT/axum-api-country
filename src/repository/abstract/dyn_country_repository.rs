use std::sync::Arc;

use axum::async_trait;

use crate::models::country::Country;

use super::error::RepositoryResult;

pub type DynCountryRepository = Arc<dyn CountryRepository + Send + Sync>;

#[async_trait]
pub trait CountryRepository {
    async fn get_countries_pagination(&self,skip :u32,count:u32) -> RepositoryResult<Vec<Country>>;
    async fn find_country_by_id(&self,id : i32) ->  RepositoryResult<Option<Country>>;
}