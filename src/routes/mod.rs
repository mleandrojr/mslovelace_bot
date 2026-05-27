use axum::{Router, routing::post};

use crate::server::AppState;
use crate::handlers::incoming::handle;

pub fn router() -> Router<AppState> {
    Router::new().route("/incoming", post(handle))
}
