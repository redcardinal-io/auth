use hyper::StatusCode;
use serde::Serialize;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InternalServer { message: String },
    NotFound,
    BadRequest { message: String },
    Unauthorized,
    Forbidden,
    Conflict { message: String },
    Validation { message: String },
    Timeout,
    Unavailable,
    Database { message: String },
    Configuration { message: String },
}

pub type Result<T> = std::result::Result<T, AppError>;

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalServer { message } => write!(f, "Internal server error: {}", message),
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::BadRequest { message } => write!(f, "Bad request: {}", message),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Forbidden => write!(f, "Forbidden"),
            AppError::Conflict { message } => write!(f, "Conflict: {}", message),
            AppError::Validation { message } => write!(f, "Validation error: {}", message),
            AppError::Timeout => write!(f, "Request timeout"),
            AppError::Unavailable => write!(f, "Service unavailable"),
            AppError::Database { message } => write!(f, "Database error: {}", message),
            AppError::Configuration { message } => {
                write!(f, "Configuration error: {}", message)
            }
        }
    }
}

impl std::error::Error for AppError {}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    Conflict,
    Internal,
    Invalid,
    NotFound,
    Unauthorized,
    Forbidden,
    Timeout,
    Unavailable,
    UnprocessableEntity,
    DatabaseError,
    MessageBrokerError,
    ValidationError,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServer { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::Conflict { .. } => StatusCode::CONFLICT,
            AppError::Validation { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Timeout => StatusCode::REQUEST_TIMEOUT,
            AppError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Database { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Configuration { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(&self) -> ErrorCode {
        match self {
            AppError::Conflict { .. } => ErrorCode::Conflict,
            AppError::InternalServer { .. } => ErrorCode::Internal,
            AppError::BadRequest { .. } => ErrorCode::Invalid,
            AppError::NotFound => ErrorCode::NotFound,
            AppError::Unauthorized => ErrorCode::Unauthorized,
            AppError::Forbidden => ErrorCode::Forbidden,
            AppError::Timeout => ErrorCode::Timeout,
            AppError::Unavailable => ErrorCode::Unavailable,
            AppError::Validation { .. } => ErrorCode::ValidationError,
            AppError::Database { .. } => ErrorCode::DatabaseError,
            AppError::Configuration { .. } => ErrorCode::Internal,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub status: StatusCode,
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub details: HashMap<String, serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(error: &AppError) -> Self {
        let mut details = HashMap::new();
        if let AppError::Validation {
            message: val_details,
        } = error
        {
            details.insert(
                "validation".to_string(),
                serde_json::Value::String(val_details.clone()),
            );
        }

        Self {
            status: error.status_code(),
            code: error.code(),
            message: error.to_string(),
            details,
        }
    }

    pub fn builder(error: AppError) -> ErrorResponseBuilder {
        ErrorResponseBuilder::new(error)
    }
}

pub struct ErrorResponseBuilder {
    error: AppError,
    details: HashMap<String, serde_json::Value>,
    operation: Option<String>,
}

impl ErrorResponseBuilder {
    pub fn new(error: AppError) -> Self {
        Self {
            error,
            details: HashMap::new(),
            operation: None,
        }
    }

    pub fn with_operation(mut self, op: &str) -> Self {
        self.operation = Some(op.to_string());
        self
    }

    pub fn with_data<T: Serialize>(mut self, key: &str, value: T) -> Self {
        self.details.insert(
            key.to_string(),
            serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        );
        self
    }

    pub fn build(mut self) -> ErrorResponse {
        let mut response = ErrorResponse::new(&self.error);
        if let Some(op) = self.operation {
            self.details
                .insert("operation".to_string(), serde_json::Value::String(op));
        }
        response.details.extend(self.details);
        response
    }
}
