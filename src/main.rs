mod api;

use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::boxscores::{BoxScore, CountResponse, filter_boxscores, get_count};

#[derive(OpenApi)]
#[openapi(
    paths(
        api::boxscores::routes::get_count,
        api::boxscores::routes::filter_boxscores
    ),
    components(schemas(CountResponse, BoxScore))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/api/boxscores/count", get(get_count))
        .route("/api/boxscores/filter", get(filter_boxscores))
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
