use http::StatusCode;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use thiserror::Error;

/// AppError is the primary error type for the application.
///
/// It uses `thiserror` to automatically implement the `std::error::Error` trait.
/// - `#[error("{message}")]` makes the `Display` trait output the `message` field.
/// - `#[source]` indicates that `source` is the underlying cause of the error.
#[derive(Debug, Error)]
#[error("{message}")]
pub struct Error {
    pub message: String,
    pub code: ErrorCode,
    pub status: StatusCode,
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
    pub internal: Option<String>,
    pub op: Option<String>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    Conflict,
    Internal,
    Invalid,
    NotFound,
    ServerError,
    Unauthorized,
    Forbidden,
    Timeout,
    Unavailable,
    UnprocessableEntity,
    DatabaseError,
    ValidationError,
    ConfigurationError,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self).unwrap().as_str().unwrap()
        )
    }
}

impl ErrorCode {
    /// Returns the corresponding HTTP status code for the error code.
    ///
    /// Maps each `ErrorCode` variant to an appropriate `StatusCode` value, enabling consistent translation of application errors to HTTP responses.
    ///
    /// # Examples
    ///
    /// ```
    /// use rcauth_core::error::ErrorCode;
    /// use http::StatusCode;
    ///
    /// assert_eq!(ErrorCode::Conflict.to_status_code(), StatusCode::CONFLICT);
    /// assert_eq!(ErrorCode::ServerError.to_status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    /// assert_eq!(ErrorCode::ValidationError.to_status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    /// ```
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            ErrorCode::Conflict => StatusCode::CONFLICT,
            ErrorCode::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::Invalid => StatusCode::BAD_REQUEST,
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorCode::Forbidden => StatusCode::FORBIDDEN,
            ErrorCode::Timeout => StatusCode::REQUEST_TIMEOUT,
            ErrorCode::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            ErrorCode::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorCode::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::ValidationError => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorCode::ConfigurationError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Error {
    /// Creates a new `AppError`.
    ///
    /// This is the main constructor. It takes a source error, an error code, and a
    /// user-facing message. The source error is wrapped in the `AppError`.
    pub fn new<E>(code: ErrorCode, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        let msg = message.into();
        let internal_message = source.to_string();
        Self {
            status: code.to_status_code(),
            message: format!("{}: {}", msg, internal_message),
            internal: Some(internal_message),
            source: Some(Box::new(source)),
            code,
            op: None,
            data: None,
        }
    }

    /// Creates a new `AppError` without a source error.
    pub fn new_simple(code: ErrorCode, message: impl Into<String>) -> Self {
        let msg = message.into();
        Self {
            status: code.to_status_code(),
            internal: Some(msg.clone()),
            message: msg,
            source: None,
            code,
            op: None,
            data: None,
        }
    }

    /// Sets a custom HTTP status code.
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// Sets the operation context string.
    pub fn with_op(mut self, op: impl Into<String>) -> Self {
        self.op = Some(op.into());
        self
    }

    pub fn with_internal(mut self, internal: impl Into<String>) -> Self {
        self.internal = Some(internal.into());
        self
    }

    pub fn with_data(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.data
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value);
        self
    }
}

/// Represents the final JSON response sent to the client.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// The HTTP status code of the response. This is not serialized.
    #[serde(skip)]
    pub status: StatusCode,
    /// The machine-readable error code.
    pub code: String,
    /// The human-readable error message.
    pub message: String,
    /// Additional details about the error. Omitted if empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, serde_json::Value>>,
}

impl ErrorResponse {
    pub fn from_error(err: &(dyn std::error::Error + 'static)) -> Self {
        // Try to downcast the error to our specific AppError type.
        if let Some(app_err) = err.downcast_ref::<Error>() {
            let mut details = app_err.data.clone();
            if let Some(op) = &app_err.op {
                let d = details.get_or_insert_with(HashMap::new);
                d.insert("operation".to_string(), serde_json::json!(op));
            }

            Self {
                status: app_err.status,
                code: app_err.code.to_string(),
                message: app_err.message.clone(),
                details,
            }
        } else {
            // If it's not an AppError, create a generic 500 response.
            Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                code: ErrorCode::Internal.to_string(),
                message: "An unexpected internal error occurred.".to_string(),
                details: None,
            }
        }
    }
}
