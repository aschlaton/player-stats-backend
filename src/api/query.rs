use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio_postgres::Client as PgClient;

use crate::llm::{LLMProvider, QUERY_PROMPT};
use super::boxscores::models::{PaginatedResponse, QueryParams};
use super::db::query_boxscores;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

pub struct AppState {
    pub llm_provider: LLMProvider,
    pub db_client: Arc<PgClient>,
    pub readonly_db_client: Arc<PgClient>,
}

pub async fn post_query(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QueryRequest>,
) -> Result<Json<PaginatedResponse>, (StatusCode, String)> {
    let schema = json!({
        "type": "object",
        "properties": {
            "reasoning": {"type": "string", "description": "Explain your reasoning for extracting these parameters from the query"},
            "player": {"type": "string", "description": "Player name"},
            "team": {"type": "string", "description": "Team abbreviation (e.g., 'LAL', 'GSW')"},
            "season": {"type": "string", "description": "Season in format '2024-25'"},
            "game_date": {"type": "string", "description": "Specific game date in YYYY-MM-DD format"},
            "pts": {"type": "integer", "description": "Minimum points"},
            "reb": {"type": "integer", "description": "Minimum rebounds"},
            "ast": {"type": "integer", "description": "Minimum assists"},
            "stl": {"type": "integer", "description": "Minimum steals"},
            "blk": {"type": "integer", "description": "Minimum blocks"},
            "fg_percent": {"type": "number", "description": "Minimum field goal percentage"},
            "three_pm": {"type": "integer", "description": "Minimum three-pointers made"},
            "limit": {"type": "integer", "description": "Max number of results"},
            "sort_by": {
                "type": "string",
                "description": "Field to sort by",
                "enum": ["pts", "reb", "ast", "stl", "blk", "fg_percent", "three_pm", "game_date"]
            },
            "asc": {"type": "boolean", "description": "Sort ascending or descending"}
        },
        "required": ["reasoning"]
    });

    println!("User query: {}", req.query);

    let response = state
        .llm_provider
        .prompt_with_schema(QUERY_PROMPT, &req.query, schema)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    println!("LLM response: {}", response);

    let params: QueryParams = serde_json::from_str(response.trim())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse JSON: {} | Response: {}", e, response)))?;

    let box_scores = query_boxscores(&state.db_client, params)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(box_scores))
}
