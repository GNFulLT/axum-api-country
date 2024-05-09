use axum::async_trait;


use crate::models::country::Country;

use super::{r#abstract::{dyn_country_repository::CountryRepository, error::RepositoryResult}, base_postgres_repository::{get_t_option_where, get_t_pagination}, PostgresPool};
pub struct PostgresCountryRepository {
    pool : PostgresPool,
}

impl PostgresCountryRepository {
    pub fn new(pool : PostgresPool) -> Self {
        Self {
            pool,
        }
    }
}

static SELECT_OP : &str = "c.id,
c.name,
c.iso3,
c.numeric_code,
c.iso2,
c.phonecode,
c.capital,
c.currency,
c.currency_name,
c.currency_symbol,
c.tld,
c.native,
c.region,
c.region_id,
c.subregion,
c.subregion_id,
c.nationality,
c.timezones,
c.translations,
c.latitude,
c.longitude,
c.emoji,
c.emojiu,
c.created_at,
c.updated_at,
c.flag,
c.wikidataid";

#[async_trait]
impl CountryRepository for PostgresCountryRepository {
    async fn get_countries_pagination(&self,skip :u32,count:u32) -> RepositoryResult<Vec<Country>> 
    {
        let res = get_t_pagination::<Country>(&self.pool, SELECT_OP,
        "countries c", skip, count).await;

        if res.is_err() {
            return Err(res.err().unwrap());
        }
        
        let data = res.ok().unwrap();
        return Ok(data);
    }

    async fn find_country_by_id(&self,id : i32) ->  RepositoryResult<Option<Country>>
    {
        let res = get_t_option_where::<Country>(&self.pool,SELECT_OP,
        "countries c",format!("c.id={id}").as_str()).await;

        if res.is_err() {
            return Err(res.err().unwrap());
        }
            
        let data = res.ok().unwrap();
        return Ok(data);
    }
}