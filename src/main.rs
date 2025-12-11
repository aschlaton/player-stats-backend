mod api;
mod llm;

use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use tokio_postgres::NoTls;
use tower_http::cors::{CorsLayer, Any};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::boxscores::{BoxScore, CountResponse, get_boxscores, get_count};
use api::query::{post_query, AppState};
use api::sql::post_sql;
use llm::get_provider;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::boxscores::routes::get_count,
        api::boxscores::routes::get_boxscores
    ),
    components(schemas(CountResponse, BoxScore))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let llm_provider = get_provider();

    let (db_client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    let readonly_url = std::env::var("DATABASE_URL_READONLY")
        .expect("DATABASE_URL_READONLY must be set");

    let (readonly_db_client, readonly_connection) = tokio_postgres::connect(&readonly_url, NoTls)
        .await
        .expect("Failed to connect to read-only database");

    tokio::spawn(async move {
        if let Err(e) = readonly_connection.await {
            eprintln!("Read-only database connection error: {}", e);
        }
    });

    let state = Arc::new(AppState {
        llm_provider,
        db_client: Arc::new(db_client),
        readonly_db_client: Arc::new(readonly_db_client),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app: Router = Router::new()
        .route("/api/boxscores/count", get(get_count))
        .route("/api/boxscores", get(get_boxscores))
        .route("/api/query", post(post_query))
        .route("/api/sql", post(post_sql))
        .with_state(state)
        .layer(cors)
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
