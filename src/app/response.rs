use std::fmt::Debug;

use actix_web::{cookie::Cookie, http::StatusCode, HttpResponse, Responder};
use serde::Serialize;
use tracing::{error, info, warn};

use super::error::AppError;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ResponseStatus {
    Success,
    Fail,
    Error,
}

type AppResult<T> = Result<T, AppError>;
pub type AppResponse = AppResult<HttpResponse>;

#[derive(Debug, Serialize)]
pub struct ClientResponse<T: Debug + Serialize + Clone> {
    status: ResponseStatus,
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Debug + Serialize + Clone> ClientResponse<T> {
    pub fn default() -> HttpResponse {
        Self::build().send()
    }
    pub fn build() -> Self {
        Self {
            status: ResponseStatus::Success,
            code: 200,
            message: None,
            data: None,
        }
    }

    pub fn with_status(&self, status_code: StatusCode) -> Self {
        let status = if status_code.is_success() {
            ResponseStatus::Success
        } else if status_code.is_client_error() {
            ResponseStatus::Fail
        } else {
            ResponseStatus::Error
        };
        Self {
            status,
            code: status_code.as_u16(),
            message: self.message.clone(),
            data: self.data.clone(),
        }
    }

    pub fn with_message(&self, message: String) -> Self {
        Self {
            message: Some(message),
            status: self.status.clone(),
            code: self.code.clone(),
            data: self.data.clone(),
        }
    }

    pub fn with_data(&self, data: T) -> Self {
        Self {
            data: Some(data),
            status: self.status.clone(),
            code: self.code.clone(),
            message: self.message.clone(),
        }
    }
    pub fn send(&self) -> HttpResponse {
        let status_code = StatusCode::from_u16(self.code).unwrap();
        if status_code.is_success() {
            info!("{:?}", self);
        } else if status_code.is_client_error() {
            warn!("{:?}", self);
        } else {
            error!("{:?}", self);
        };
        HttpResponse::build(status_code)
            .insert_header((
                "set-cookie",
                "test=working; Path=/;Expires=Wed, 21 Oct 2022 07:28:00 GMT",
            ))
            .json(self)
        // .headers_mut()
        // .insert("Set-Cookie", "TEST")
    }
}
