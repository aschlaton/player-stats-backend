use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tokio_postgres::NoTls;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, ToSchema)]
struct CountResponse {
    count: i64,
}

#[derive(OpenApi)]
#[openapi(
    paths(get_count),
    components(schemas(CountResponse))
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api/boxscores/count",
    responses(
        (status = 200, description = "Get total box score count", body = CountResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_count() -> Result<Json<CountResponse>, String> {
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

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/api/boxscores/count", get(get_count))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("server running on http://{}", addr);
    println!("docs at http://{}/docs", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
