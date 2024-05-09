use axum::async_trait;

use crate::models::city::City;

use super::{r#abstract::{dyn_city_repository::CityRepository, error::RepositoryResult}, base_postgres_repository::{get_t_option_where, get_t_pagination}, PostgresPool};
pub struct PostgresCityRepository {
    pool : PostgresPool,
}

impl PostgresCityRepository {
    pub fn new(pool : PostgresPool) -> Self {
        Self {
            pool,
        }
    }
}

static SELECT_OP : &str = "c.id,
c.name,
c.state_id,
c.state_code,
c.country_id,
c.country_code,
c.latitude,
c.longitude,
c.created_at,
c.updated_at,
c.flag,
c.wikidataid";

#[async_trait]
impl CityRepository for PostgresCityRepository {
    async fn get_cities_pagination(&self,skip :u32,count:u32) -> RepositoryResult<Vec<City>> {
                let res = get_t_pagination::<City>(&self.pool,SELECT_OP, "cities c", skip, count).await;

                if res.is_err() {
                    return Err(res.err().unwrap());
                }
                
                let data = res.ok().unwrap();
                return Ok(data);
       }

    async fn find_city_by_id(&self,id:i32) -> RepositoryResult<Option<City>> {
        let res = get_t_option_where(&self.pool,SELECT_OP, "cities c", format!("c.id={id}").as_str()).await;
    
        if res.is_err() {
            return Err(res.err().unwrap());
        }
            
        let data = res.ok().unwrap();
        return Ok(data);

    }
}

