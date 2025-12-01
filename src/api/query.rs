use axum::{extract::State, http::StatusCode, Json};
use rig::completion::Prompt;
use rig::prelude::*;
use rig::providers::gemini;
use rig::providers::gemini::completion::gemini_api_types::AdditionalParameters;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use std::sync::Arc;
use tokio_postgres::Client as PgClient;

use super::tools::GetBoxScores;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub result: String,
}

pub struct AppState {
    pub gemini_client: gemini::Client,
    pub db_client: Arc<PgClient>,
}

pub async fn post_query(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, (StatusCode, String)> {
    let additional_params = AdditionalParameters::default();

    let agent = state
        .gemini_client
        .agent("gemini-2.5-flash")
        .preamble("You are a helpful assistant that helps users query NBA player statistics.")
        .additional_params(
            to_value(additional_params)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        )
        .tool(GetBoxScores {
            client: Arc::clone(&state.db_client),
        })
        .build();

    let response = agent
        .prompt(&req.query)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(QueryResponse {
        result: response,
    }))
}
