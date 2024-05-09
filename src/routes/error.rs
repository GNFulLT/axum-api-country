use axum::{http::StatusCode, response::IntoResponse, Json};


pub type RouteResult<T> = Result<T,RouteError>;

#[derive(Debug)]
pub enum RouteError {
    RouteErrorInternalError,
    CouldntBeFound    
}

impl IntoResponse for RouteError {
    fn into_response(self) -> axum::response::Response {

        match self {
            RouteError::CouldntBeFound => (StatusCode::NOT_FOUND,Json("There is no city with this id")).into_response(),
            _ =>  (StatusCode::INTERNAL_SERVER_ERROR,Json("Unexpected internal server error")).into_response(),
        }

       
    }
}
 
