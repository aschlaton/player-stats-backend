mod api;

use axum::{routing::{get, post}, Router};
use rig::providers::gemini;
use std::sync::Arc;
use tokio_postgres::NoTls;
use tower_http::cors::{CorsLayer, Any};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::boxscores::{BoxScore, CountResponse, get_boxscores, get_count};
use api::query::{post_query, AppState};

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
    let gemini_api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set");

    let (db_client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    let gemini_client = gemini::Client::new(&gemini_api_key);

    let state = Arc::new(AppState {
        gemini_client,
        db_client: Arc::new(db_client),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app: Router = Router::new()
        .route("/api/boxscores/count", get(get_count))
        .route("/api/boxscores", get(get_boxscores))
        .route("/api/query", post(post_query))
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
