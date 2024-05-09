use axum::async_trait;



use crate::models::state::State;

use super::{r#abstract::{dyn_state_repository::StateRepository, error::RepositoryResult}, base_postgres_repository::{get_t_option_where, get_t_pagination}, PostgresPool};
pub struct PostgresStateRepository {
    pool : PostgresPool,
}

impl PostgresStateRepository {
    pub fn new(pool : PostgresPool) -> Self {
        Self {
            pool,
        }
    }
}

static SELECT_OP : &str = "s.id,
s.name,
s.country_id,
s.country_code,
s.fips_code,
s.iso2,
s.type,
s.latitude,
s.longitude,
s.created_at,
s.updated_at,
s.flag,
s.wikidataid";

#[async_trait]
impl StateRepository for PostgresStateRepository {
    async fn get_states_pagination(&self,skip :u32,count:u32) -> RepositoryResult<Vec<State>> 
    {
        let res = get_t_pagination::<State>(&self.pool, SELECT_OP,
        "states s", skip, count).await;

        if res.is_err() {
            return Err(res.err().unwrap());
        }
        
        let data = res.ok().unwrap();
        return Ok(data);
    }

    async fn find_state_by_id(&self,id : i32) ->  RepositoryResult<Option<State>>
    {
        let res = get_t_option_where::<State>(&self.pool,SELECT_OP,
        "states s",format!("s.id={id}").as_str()).await;

        if res.is_err() {
            return Err(res.err().unwrap());
        }
            
        let data = res.ok().unwrap();
        return Ok(data);
    }
}