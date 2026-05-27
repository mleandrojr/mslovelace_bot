use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::server::AppState;
use crate::integrations::telegram::types::UpdateType;
use crate::services::telegram;

pub async fn handle(State(state): State<AppState>, Json(update): Json<UpdateType>) -> impl IntoResponse {
    telegram::process(&state.pool, &state.api, update).await;
    StatusCode::OK
}
