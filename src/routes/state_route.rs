use axum::extract::Path;
use axum::{Extension, Json, Router};
use axum::routing::get;

use crate::models::state::State;
use crate::repository::r#abstract::dyn_state_repository::DynStateRepository;

use super::error::{RouteError, RouteResult};
use super::skip::Skip;

pub fn get_state_router() -> Router {
    Router::new()
        .route("/", get(get_state_pagination))
        .route("/:id",get(get_state_by_id))
}

async fn get_state_pagination(Extension(state_repo) : Extension<DynStateRepository>,body : Json<Skip>) -> RouteResult<Json<Vec<State>>> {
    let res = state_repo.get_states_pagination(body.skip, body.count).await.map_err(|e| {
        match e {
            crate::repository::r#abstract::error::RepositoryError::QueryExecutionError(msg) => println!("query execution err {}",msg),
        }
        RouteError::RouteErrorInternalError
    }
    );
    if res.is_err() {
        return Err(res.err().unwrap());
    }

    return Ok(Json(res.ok().unwrap()));
}   

async fn get_state_by_id(Extension(state_repo) : Extension<DynStateRepository>, Path(id) : Path<i32>) -> RouteResult<Json<State>> {
    let res = state_repo.find_state_by_id(id).await.map_err(|e| {
        match e {
            crate::repository::r#abstract::error::RepositoryError::QueryExecutionError(msg) => println!("query execution err {}",msg),
        }
        RouteError::RouteErrorInternalError
    });

    if res.is_err() {
        return Err(res.err().unwrap());
    }

    let data = res.ok().unwrap();

    match data {
        Some(city) => Ok(Json(city)),
        None => Err(RouteError::CouldntBeFound),
    }
}