use axum::{
    Router,
    routing::{get, post},
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;
use super::Logger;

#[derive(Debug, Deserialize)]
pub struct LogFilter {
    method: Option<String>,
    status: Option<u16>,
    path: Option<String>,
}

pub fn create_logger_routes(logger: Arc<Logger>) -> Router {
    Router::new()
        .route("/api/logs", get(get_logs))
        .route("/api/logs/filter", get(filter_logs))
        .route("/api/logs/clear", post(clear_logs))
        .with_state(logger)
}

async fn get_logs(
    State(logger): State<Arc<Logger>>,
) -> Json<Vec<super::RequestLog>> {
    Json(logger.get_logs().await)
}

async fn filter_logs(
    State(logger): State<Arc<Logger>>,
    Query(filter): Query<LogFilter>,
) -> Json<Vec<super::RequestLog>> {
    Json(logger.filter_logs(
        filter.method,
        filter.status,
        filter.path,
    ).await)
}

async fn clear_logs(
    State(logger): State<Arc<Logger>>,
) -> &'static str {
    logger.clear_logs().await;
    "Logs cleared"
} 