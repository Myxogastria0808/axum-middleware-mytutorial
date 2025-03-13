use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{Method, StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use error::AppError;
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
        .route("/login", get(login_handler))
        .route("/profile", get(login_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)); //100MB

    // Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
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
    path = "/login",
    tag = "Sample",
    responses(
        (status = 200, description = "OK"),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn login_handler() -> Result<impl IntoResponse + Send, AppError> {
    Ok((StatusCode::OK, ()).into_response())
}

#[utoipa::path(
    post,
    path = "/profile",
    tag = "Sample",
    responses(
        (status = 200, description = "OK"),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn profile_handler() -> Result<impl IntoResponse + Send, AppError> {
    Ok((StatusCode::OK, ()).into_response())
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
        crate::login_handler,
        crate::profile_handler,
    ),
    components(schemas(
        crate::error::ResponseError,
    ))
)]
struct ApiDoc;
