use crate::application::item_service::{CreateItemsCommand, ItemModel, ItemService};
use crate::web::errors::ServerError;
use crate::web::response::{CreateItemsResponse, ListItemsResponse};
use axum::extract::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::Router;
use axum_extra::extract::WithRejection;
use std::sync::Arc;
use uuid::Uuid;

pub struct ItemRouter;

impl ItemRouter {
    pub fn new(service: Arc<dyn ItemService + Send + Sync>) -> Router {
        Router::new()
            .route("/tables/:table_id/items", post(create_items))
            .route("/tables/:table_id/items", get(list_items))
            .route("/tables/:table_id/items/:item_id", get(get_item))
            .route("/tables/:table_id/items/:item_id", delete(delete_item))
            .with_state(service)
    }
}

async fn create_items(
    State(service): State<Arc<dyn ItemService + Send + Sync>>,
    WithRejection(Path(table_id), _): WithRejection<Path<i64>, ServerError>,
    WithRejection(Json(command), _): WithRejection<Json<CreateItemsCommand>, ServerError>,
) -> Result<(StatusCode, Json<CreateItemsResponse>), ServerError> {
    let items = service.create_items(table_id, command).await?;

    Ok((StatusCode::CREATED, Json(CreateItemsResponse::from(items))))
}

async fn list_items(
    State(service): State<Arc<dyn ItemService + Send + Sync>>,
    WithRejection(Path(table_id), _): WithRejection<Path<i64>, ServerError>,
) -> Result<(StatusCode, Json<ListItemsResponse>), ServerError> {
    let items = service.get_items(table_id).await?;

    Ok((StatusCode::OK, Json(ListItemsResponse::from(items))))
}

async fn get_item(
    State(service): State<Arc<dyn ItemService + Send + Sync>>,
    WithRejection(Path((table_id, item_id)), _): WithRejection<Path<(i64, Uuid)>, ServerError>,
) -> Result<(StatusCode, Json<ItemModel>), ServerError> {
    let item = service.get_item(table_id, item_id).await?;

    Ok((StatusCode::OK, Json(item)))
}

async fn delete_item(
    State(service): State<Arc<dyn ItemService + Send + Sync>>,
    WithRejection(Path((table_id, item_id)), _): WithRejection<Path<(i64, Uuid)>, ServerError>,
) -> Result<StatusCode, ServerError> {
    service.delete_item(table_id, item_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
