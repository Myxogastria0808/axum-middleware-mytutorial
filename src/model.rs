use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RequestData {
    pub name: String,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseData {
    pub message: String,
}
