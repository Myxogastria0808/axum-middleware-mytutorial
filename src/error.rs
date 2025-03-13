use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseError {
    message: String,
}

#[derive(Debug)]
pub struct AppError(anyhow::Error);

//anyhow::error => AppError への型変換
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

//AppError => axum::response::Response への型変換
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(ResponseError {
                message: self.0.to_string(),
            })),
        )
            .into_response()
    }
}
