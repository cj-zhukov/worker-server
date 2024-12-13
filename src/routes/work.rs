use axum::extract::State;

use crate::AppState;

pub async fn work(State(state): State<AppState>) -> String {
    // sleep(Duration::from_secs(2)).await;
    format!("Response from worker: {}", state.server_name)
}