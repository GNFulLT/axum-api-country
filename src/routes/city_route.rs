use axum::extract::Path;
use axum::{Extension, Json, Router};
use axum::routing::get;

use crate::models::city::City;
use crate::repository::r#abstract::dyn_city_repository::DynCityRepository;

use super::error::{RouteError, RouteResult};
use super::skip::Skip;

pub fn get_city_router() -> Router {
    Router::new()
        .route("/", get(get_city_pagination))
        .route("/:id",get(get_city_by_id))
}




async fn get_city_pagination(Extension(city_repo) : Extension<DynCityRepository>,body : Json<Skip>) -> RouteResult<Json<Vec<City>>> {
    let res = city_repo.get_cities_pagination(body.skip, body.count).await.map_err(|e| {
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

async fn get_city_by_id(Extension(city_repo) : Extension<DynCityRepository>, Path(id) : Path<i32>) -> RouteResult<Json<City>> {
    let res = city_repo.find_city_by_id(id).await.map_err(|e| {
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