use std::sync::Arc;

use crate::repository::{r#abstract::{dyn_city_repository::DynCityRepository, dyn_country_repository::DynCountryRepository, dyn_state_repository::DynStateRepository}, city_repository::PostgresCityRepository, country_repository::PostgresCountryRepository, state_repository::PostgresStateRepository, PostgresPool};

// Current project only contains get functions so I implemented only repos here
#[derive(Clone)]
pub struct ServiceRegister {
    pub city_repo : DynCityRepository,
    pub country_repo : DynCountryRepository,
    pub state_repo : DynStateRepository,

}

impl ServiceRegister {
    
    pub fn new(pool : PostgresPool) -> Self {
        Self {
            city_repo : Arc::new(PostgresCityRepository::new(pool.clone())),
            country_repo : Arc::new(PostgresCountryRepository::new(pool.clone())),
            state_repo : Arc::new(PostgresStateRepository::new(pool.clone()))

        }
    } 
}