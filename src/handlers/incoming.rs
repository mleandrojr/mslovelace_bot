use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::server::AppState;
use crate::integrations::telegram::types::UpdateType;
use crate::services::telegram;

pub async fn handle(
    State(state): State<AppState>,
    Path(hash): Path<String>,
    Json(update): Json<UpdateType>,
) -> impl IntoResponse {
    if hash != state.auth {
        return StatusCode::UNAUTHORIZED;
    }

    telegram::process(&state.pool, &state.api, update).await;
    StatusCode::OK
}
