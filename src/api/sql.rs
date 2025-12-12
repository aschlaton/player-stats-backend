use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use tokio_postgres::Client;

use crate::llm::SQL_PROMPT;
use super::query::AppState;

#[derive(Deserialize)]
pub struct SqlRequest {
    pub query: String,
}

#[derive(Serialize)]
pub struct SqlResponse {
    pub data: Vec<Value>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
    pub explicit_limit: bool,
    pub query_params: Value,
}

pub async fn execute_sql_query(
    client: &Client,
    sql: &str,
) -> Result<Vec<Value>, String> {

    let rows: Vec<tokio_postgres::Row> = client
        .query(sql, &[])
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    // Convert rows to JSON
    let mut result_rows = Vec::new();
    for row in &rows {
        let mut obj = serde_json::Map::new();
        for (idx, column) in row.columns().iter().enumerate() {
            let value: Value = match column.type_().name() {
                "int4" => {
                    row.try_get::<_, Option<i32>>(idx)
                        .unwrap_or(None)
                        .map(|v| serde_json::json!(v))
                        .unwrap_or(Value::Null)
                }
                "int8" => {
                    row.try_get::<_, Option<i64>>(idx)
                        .unwrap_or(None)
                        .map(|v| serde_json::json!(v))
                        .unwrap_or(Value::Null)
                }
                "float4" | "float8" => {
                    row.try_get::<_, Option<f64>>(idx)
                        .unwrap_or(None)
                        .map(|v| serde_json::json!(v))
                        .unwrap_or(Value::Null)
                }
                "varchar" | "text" => {
                    row.try_get::<_, Option<String>>(idx)
                        .unwrap_or(None)
                        .map(|v| serde_json::json!(v))
                        .unwrap_or(Value::Null)
                }
                "bool" => {
                    row.try_get::<_, Option<bool>>(idx)
                        .unwrap_or(None)
                        .map(|v| serde_json::json!(v))
                        .unwrap_or(Value::Null)
                }
                "timestamp" => {
                    row.try_get::<_, Option<String>>(idx)
                        .unwrap_or(None)
                        .map(|v| serde_json::json!(v))
                        .unwrap_or(Value::Null)
                }
                _ => Value::Null,
            };
            obj.insert(column.name().to_string(), value);
        }
        result_rows.push(Value::Object(obj));
    }

    Ok(result_rows)
}

pub async fn post_sql(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SqlRequest>,
) -> Result<Json<SqlResponse>, (StatusCode, String)> {
    println!("User query: {}", req.query);

    // Get SQL from LLM
    let sql = state
        .llm_provider
        .prompt(SQL_PROMPT, &req.query)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let sql = sql.trim().to_string();

    println!("Generated SQL: {}", sql);

    // Execute with 10 second timeout
    let rows = timeout(
        Duration::from_secs(10),
        execute_sql_query(&state.readonly_db_client, &sql)
    )
    .await
    .map_err(|_| (
        StatusCode::REQUEST_TIMEOUT,
        "Query execution exceeded 10 second timeout".to_string()
    ))?
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let row_count = rows.len();

    Ok(Json(SqlResponse {
        data: rows,
        total: row_count,
        limit: row_count,
        offset: 0,
        explicit_limit: true,
        query_params: serde_json::json!({"sql": sql}),
    }))
}
