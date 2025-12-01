use axum::{extract::Query, Json};
use tokio_postgres::NoTls;

use crate::api::db::query_boxscores;
use super::models::{CountResponse, QueryParams, PaginatedResponse};

#[utoipa::path(
    get,
    path = "/api/boxscores/count",
    responses(
        (status = 200, description = "Get total box score count", body = CountResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_count() -> Result<Json<CountResponse>, String> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL not set".to_string())?;

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|e| format!("Connection error: {}", e))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let row = client
        .query_one("SELECT COUNT(*) FROM player_box_scores", &[])
        .await
        .map_err(|e| format!("Query error: {}", e))?;

    let count: i64 = row.get(0);

    Ok(Json(CountResponse { count }))
}

#[utoipa::path(
    get,
    path = "/api/boxscores",
    params(QueryParams),
    responses(
        (status = 200, description = "Get box scores with optional filters, sorting, and pagination", body = PaginatedResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_boxscores(
    Query(params): Query<QueryParams>,
) -> Result<Json<PaginatedResponse>, String> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL not set".to_string())?;

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|e| format!("Connection error: {}", e))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let response = query_boxscores(&client, params).await?;

    Ok(Json(response))
}
