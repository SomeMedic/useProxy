use axum::{
    Router,
    routing::get_service,
};
use tower_http::services::ServeDir;
use std::path::PathBuf;

pub fn static_files_service(directory: PathBuf) -> Router {
    Router::new().nest_service(
        "/",
        get_service(ServeDir::new(directory))
            .handle_error(|error| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
    )
} 