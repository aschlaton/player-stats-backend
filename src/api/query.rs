use axum::{extract::State, http::StatusCode, Json};
use rig::completion::Prompt;
use rig::prelude::*;
use rig::providers::gemini;
use rig::providers::gemini::completion::gemini_api_types::{AdditionalParameters, GenerationConfig, Schema};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_postgres::Client as PgClient;

use super::boxscores::models::{PaginatedResponse, QueryParams};
use super::db::query_boxscores;
use super::prompts::QUERY_PROMPT;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

pub struct AppState {
    pub gemini_client: gemini::Client,
    pub db_client: Arc<PgClient>,
}

fn create_schema(type_name: &str, description: &str, enum_values: Option<Vec<String>>) -> Schema {
    Schema {
        r#type: type_name.to_string(),
        format: None,
        description: Some(description.to_string()),
        nullable: None,
        r#enum: enum_values,
        max_items: None,
        min_items: None,
        properties: None,
        required: None,
        items: None,
    }
}

pub async fn post_query(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QueryRequest>,
) -> Result<Json<PaginatedResponse>, (StatusCode, String)> {
    let mut properties = HashMap::new();
    properties.insert("pts".to_string(), create_schema("integer", "Minimum points", None));
    properties.insert("reb".to_string(), create_schema("integer", "Minimum rebounds", None));
    properties.insert("ast".to_string(), create_schema("integer", "Minimum assists", None));
    properties.insert("stl".to_string(), create_schema("integer", "Minimum steals", None));
    properties.insert("blk".to_string(), create_schema("integer", "Minimum blocks", None));
    properties.insert("fg_percent".to_string(), create_schema("number", "Minimum field goal percentage (e.g., 50.0 for 50%)", None));
    properties.insert("three_pm".to_string(), create_schema("integer", "Minimum three-pointers made", None));
    properties.insert("game_date".to_string(), create_schema("string", "Specific game date in YYYY-MM-DD format", None));
    properties.insert("team".to_string(), create_schema("string", "Team abbreviation (e.g., 'LAL', 'GSW')", None));
    properties.insert("season".to_string(), create_schema("string", "Season in format '2024-25'", None));
    properties.insert("player".to_string(), create_schema("string", "Player name", None));
    properties.insert("limit".to_string(), create_schema("integer", "Number of results", None));
    properties.insert("sort_by".to_string(), create_schema(
        "string",
        "Field to sort by",
        Some(vec![
            "pts".to_string(),
            "reb".to_string(),
            "ast".to_string(),
            "stl".to_string(),
            "blk".to_string(),
            "fg_percent".to_string(),
            "three_pm".to_string(),
            "game_date".to_string(),
        ])
    ));
    properties.insert("asc".to_string(), create_schema("boolean", "Sort ascending (true) or descending (false)", None));
    properties.insert("reasoning".to_string(), create_schema("string", "Explain your reasoning for extracting these parameters from the query", None));

    println!("Schema has these fields: {:?}", properties.keys().collect::<Vec<_>>());

    let schema = Schema {
        r#type: "object".to_string(),
        format: None,
        description: None,
        nullable: None,
        r#enum: None,
        max_items: None,
        min_items: None,
        properties: Some(properties),
        required: Some(vec!["reasoning".to_string()]),
        items: None,
    };

    let generation_config = GenerationConfig {
        response_mime_type: Some("application/json".to_string()),
        response_schema: Some(schema),
        ..Default::default()
    };

    let additional_params = AdditionalParameters::default()
        .with_config(generation_config);

    let agent = state
        .gemini_client
        .agent("gemini-2.5-pro")
        .preamble(QUERY_PROMPT)
        .additional_params(json!(additional_params))
        .build();

    println!("User query: {}", req.query);

    let response = agent
        .prompt(&req.query)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    println!("Gemini response: {}", response);

    let params: QueryParams = serde_json::from_str(response.trim())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse JSON: {} | Response: {}", e, response)))?;

    let box_scores = query_boxscores(&state.db_client, params)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(box_scores))
}
