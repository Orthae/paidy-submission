use axum::extract::rejection::{JsonRejection, PathRejection};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use tracing::warn;
use crate::application::item_service::ApplicationError;

pub enum ServerError {
    InternalServerError,
    UnprocessableEntity(String),
    BadRequest(String),
    NotFound,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            ServerError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { message: "Internal server".to_string() })
            },
            ServerError::UnprocessableEntity(e) => {
                (StatusCode::UNPROCESSABLE_ENTITY, ErrorResponse { message: e.to_string() })
            }
            ServerError::BadRequest(e) => {
                (StatusCode::BAD_REQUEST, ErrorResponse { message: e.to_string() })
            }
            ServerError::NotFound => {
                (StatusCode::NOT_FOUND, ErrorResponse { message: "Resource not found".to_string() })
            }
        };

        (status, Json(body)).into_response()
    }
}

impl From<JsonRejection> for ServerError {
    fn from(error: JsonRejection) -> Self {
        warn!("Request body rejected due: {}", error.to_string());
        ServerError::UnprocessableEntity("Failed to deserialize the JSON body.".to_string())
    }
}

impl From<PathRejection> for ServerError {
    fn from(error: PathRejection) -> Self {
        warn!("Request path rejected due: {}", error.to_string());
        ServerError::UnprocessableEntity("Failed to extract the path parameter.".to_string())
    }
}

impl From<ApplicationError> for ServerError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::InternalError => ServerError::InternalServerError,
            ApplicationError::ValidationError(e) => ServerError::BadRequest(e),
            ApplicationError::ResourceNotFound => ServerError::NotFound,
        }
    }
}

#[derive(Default, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}