use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use core::fmt::Formatter;
use serde::{Serialize, Serializer};
use std::convert::From;
use tracing::{error, info, warn};

use super::response::ResponseStatus;

// pub fn json_error(err: JsonPayloadError, _req: &HttpRequest) {
//     let error = err.to_string();
//     error::InternalError::from_response(
//         err,
//         HttpResponse::BadRequest()
//             .json(AppErrorResponse {
//                 code: 400,
//                 error,
//                 status: ResponseStatus::Fail,
//             })
//             .into(),
//     )
//     .into()
// }

#[derive(Debug, Serialize)]
pub enum AppErrorType {
    DB,
    APP,
}

#[derive(Debug, Serialize)]
pub struct AppError {
    message: String,
    code: AppErrorCode,
    cause: Option<String>,
    error_type: AppErrorType,
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    pub status: ResponseStatus,
    pub code: i32,
    pub error: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppErrorCode(i32);

impl AppErrorCode {
    pub fn message(self, message: String) -> AppError {
        AppError {
            message,
            code: self,
            cause: None,
            error_type: AppErrorType::APP,
        }
    }

    pub fn default(self) -> AppError {
        let message = match self {
            AppError::BAD_REQUEST => "У вас какие-то проблемы",
            AppError::UNAUTHORIZED => "Незнакомец, назови себя",
            AppError::FORBIDDEN => "Вам сюда нельзя",
            AppError::NOT_FOUND => "По этому адресу никто не живёт",
            AppError::METHOD_NOT_ALLOWED => "Нельзя такое делать",
            AppError::UNPROCESSABLE_ENTITY => "Исправьте и пришлите снова",
            _ => "Всё сломалось, но мы скоро починим.",
        };
        AppError {
            message: message.to_string(),
            code: self,
            cause: None,
            error_type: AppErrorType::APP,
        }
    }
}

impl AppError {
    pub const INTERNAL_SERVER_ERROR: AppErrorCode = AppErrorCode(500);
    pub const SERVICE_ERROR: AppErrorCode = AppErrorCode(500);
    pub const DB_ERROR: AppErrorCode = AppErrorCode(500);
    pub const BAD_REQUEST: AppErrorCode = AppErrorCode(400);
    pub const UNAUTHORIZED: AppErrorCode = AppErrorCode(401);
    pub const FORBIDDEN: AppErrorCode = AppErrorCode(403);
    pub const NOT_FOUND: AppErrorCode = AppErrorCode(404);
    pub const METHOD_NOT_ALLOWED: AppErrorCode = AppErrorCode(405);
    pub const UNPROCESSABLE_ENTITY: AppErrorCode = AppErrorCode(422);
    pub const CRYPTO: AppErrorCode = AppErrorCode(500);

    pub fn with_cause(&self, error: String) -> Self {
        Self {
            error_type: AppErrorType::DB,
            cause: Some(error),
            message: self.message.clone(),
            code: self.code.clone(),
        }
    }
}

impl Serialize for AppErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.code {
            AppError::INTERNAL_SERVER_ERROR => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NOT_FOUND => StatusCode::NOT_FOUND,
            AppError::BAD_REQUEST => StatusCode::BAD_REQUEST,
            AppError::UNAUTHORIZED => StatusCode::UNAUTHORIZED,
            AppError::FORBIDDEN => StatusCode::FORBIDDEN,
            AppError::METHOD_NOT_ALLOWED => StatusCode::METHOD_NOT_ALLOWED,
            AppError::UNPROCESSABLE_ENTITY => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let status = if status_code.is_success() {
            info!("{:?}", self);
            ResponseStatus::Success
        } else if status_code.is_client_error() {
            warn!("{:?}", self);
            ResponseStatus::Fail
        } else {
            error!("{:?}", self);
            ResponseStatus::Error
        };
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message.clone(),
            code: self.code.0,
            status,
        })
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<AppErrorCode> for AppError {
    fn from(error: AppErrorCode) -> Self {
        error.default()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> AppError {
        AppError::DB_ERROR.default().with_cause(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> AppError {
        AppError::BAD_REQUEST.message(error.to_string())
    }
}

impl From<argonautica::Error> for AppError {
    fn from(error: argonautica::Error) -> AppError {
        AppError::CRYPTO.message(error.to_string())
    }
}

impl From<actix_web::error::Error> for AppError {
    fn from(error: actix_web::error::Error) -> AppError {
        AppError::CRYPTO.message(error.to_string())
    }
}
