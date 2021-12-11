use actix_web::{http::StatusCode, HttpRequest};
use serde::Serialize;

use crate::app::{
    error::AppError,
    response::{AppResponse, ClientResponse},
};

#[derive(Debug, Serialize, Clone)]
struct TestResponse {
    test: String,
}

pub async fn test_response(req: HttpRequest) -> AppResponse {
    let name = req
        .match_info()
        .get("name")
        .ok_or(AppError::NOT_FOUND.message("No name".to_string()))?;
    let status_code = StatusCode::OK;
    let data = TestResponse {
        test: name.to_string(),
    };
    Ok(ClientResponse::<TestResponse>::build()
        .with_status(status_code)
        .with_data(data)
        .with_message("Hello".to_string())
        .send())
}
