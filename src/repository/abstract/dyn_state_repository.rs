use std::sync::Arc;

use axum::async_trait;

use crate::models::state::State;

use super::error::RepositoryResult;

pub type DynStateRepository = Arc<dyn StateRepository + Send + Sync>;

#[async_trait]
pub trait StateRepository {
    async fn get_states_pagination(&self,skip :u32,count:u32) -> RepositoryResult<Vec<State>>;
    async fn find_state_by_id(&self,id : i32) ->  RepositoryResult<Option<State>>;
}