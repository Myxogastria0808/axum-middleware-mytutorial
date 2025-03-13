use std::collections::HashMap;

use axum::{
    Json, Router,
    extract::Request,
    extract::{DefaultBodyLimit, Path, Query},
    http::{Method, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use error::AppError;
use model::{RequestData, ResponseData};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod error;
mod model;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .expose_headers([header::CONTENT_DISPOSITION])
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin(Any);

    // Router
    let app: Router<()> = Router::new()
        .route("/", get(ping_handler))
        .route("/sample/:path", post(sample_handler))
        .layer(middleware::from_fn(sample_middleware))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)); //100MB

    // Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

//Middleware
async fn sample_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    //preprocess
    tracing::info!("Preprocess");
    tracing::info!(
        "Method: {}, URI: {}, headers: {:?}, request: {:?}",
        request.method(),
        request.uri(),
        request.headers(),
        request.body()
    );
    //handler
    tracing::info!("Handler");
    let response = next.run(request).await;
    //postprocess
    tracing::info!("Postprocess");
    tracing::info!(
        "Status: {}, headers: {:?}, request: {:?}",
        response.status(),
        response.headers(),
        response.body()
    );
    Ok(response)
}

//Handler
#[utoipa::path(
    get,
    path = "/",
    tag = "Sample",
    responses(
        (status = 200, description = "OK"),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn ping_handler() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::OK, "pong".to_string()).into_response())
}

#[utoipa::path(
    post,
    path = "/sample/{path}",
    tag = "Sample",
    params(
        ("path", Path, description = "path"),
        ("query", Query, description = "query"),),
    request_body(
        description = "RequestData",
        content = RequestData,
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn sample_handler(
    Path(path): Path<i32>,
    Query(query): Query<HashMap<String, String>>,
    Json(body): Json<RequestData>,
) -> Result<impl IntoResponse + Send, AppError> {
    let query = match query.get("query") {
        Some(query) => query,
        None => "",
    };
    tracing::info!(
        "path: {}, query: {}, body: {{ name: {}, message: {} }}",
        path,
        query,
        body.name,
        body.message
    );
    let result: ResponseData = ResponseData {
        message: format!(
            "path: {}, query: {}, body: {{ name: {}, message: {} }}",
            path, query, body.name, body.message
        ),
    };
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "axum-middleware-mytutorial",
        version = "0.0.1",
        description = "This is a axum-middleware-mytutorial API document.",
        contact(
            name = "Myxogastria0808",
            email = "r.rstudio.c@gmail.com",
            url = "https://yukiosada.work",
        ),
        license(
            name = "WTFPL",
            url = "http://www.wtfpl.net"
        ),
    ),
    servers((url = "http://0.0.0.0:5000")),
    tags(
        (name = "Sample", description = "Sample API"),
    ),
    paths(
        crate::ping_handler,
        crate::sample_handler,
    ),
    components(schemas(
        crate::error::ResponseError,
        crate::model::RequestData,
    ))
)]
struct ApiDoc;
