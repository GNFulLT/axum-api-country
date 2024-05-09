use axum::extract::Path;
use axum::{Extension, Json, Router};
use axum::routing::get;

use crate::models::country::Country;
use crate::repository::r#abstract::dyn_country_repository::DynCountryRepository;

use super::error::{RouteError, RouteResult};
use super::skip::Skip;

pub fn get_country_router() -> Router {
    Router::new()
        .route("/", get(get_country_pagination))
        .route("/:id",get(get_country_by_id))
}




async fn get_country_pagination(Extension(country_repo) : Extension<DynCountryRepository>,body : Json<Skip>) -> RouteResult<Json<Vec<Country>>> {
    let res = country_repo.get_countries_pagination(body.skip, body.count).await.map_err(|e| {
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

async fn get_country_by_id(Extension(country_repo) : Extension<DynCountryRepository>, Path(id) : Path<i32>) -> RouteResult<Json<Country>> {
    let res = country_repo.find_country_by_id(id).await.map_err(|e| {
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