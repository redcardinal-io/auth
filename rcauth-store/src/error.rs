use rcauth_core::error::AppError;
use snafu::prelude::*;
use sqlx::postgres::PgDatabaseError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum StoreError {
    #[snafu(display("Database connection error: {}", source))]
    Connection { source: sqlx::Error },

    #[snafu(display("Database query error: {}", source))]
    Query { source: sqlx::Error },

    #[snafu(display("Database transaction error: {}", source))]
    Transaction { source: sqlx::Error },

    #[snafu(display("Record not found"))]
    NotFound,

    #[snafu(display("Conflict with existing record: {}", message))]
    Conflict { message: String },

    #[snafu(display("Database migration error: {}", source))]
    Migration { source: sqlx::migrate::MigrateError },

    #[snafu(display("Database serialization error: {}", message))]
    Serialization { message: String },
}

pub type Result<T> = std::result::Result<T, StoreError>;

impl From<StoreError> for AppError {
    fn from(err: StoreError) -> Self {
        match err {
            StoreError::Connection { source } => AppError::Database {
                message: format!("Connection error: {}", source),
            },
            StoreError::Query { source } => AppError::Database {
                message: format!("Query error: {}", source),
            },
            StoreError::Transaction { source } => AppError::Database {
                message: format!("Transaction error: {}", source),
            },
            StoreError::NotFound => AppError::NotFound,
            StoreError::Conflict { message } => AppError::Conflict { message },
            StoreError::Migration { source } => AppError::Database {
                message: format!("Migration error: {}", source),
            },
            StoreError::Serialization { message } => AppError::Database {
                message: format!("Serialization error: {}", message),
            },
        }
    }
}

impl StoreError {
    pub fn conflict(message: impl Into<String>) -> Self {
        StoreError::Conflict {
            message: message.into(),
        }
    }

    pub fn serialization_error(message: impl Into<String>) -> Self {
        StoreError::Serialization {
            message: message.into(),
        }
    }
}

// Handle common SQLx error cases
pub fn handle_sqlx_error(error: sqlx::Error) -> StoreError {
    match &error {
        sqlx::Error::RowNotFound => StoreError::NotFound,
        sqlx::Error::Database(db_err) => {
            let pg_err = db_err.downcast_ref::<PgDatabaseError>();
            match pg_err.code() {
                // Unique violation
                "23505" => StoreError::conflict("Record already exists"),
                // Foreign key violation
                "23503" => StoreError::conflict("Related record not found"),
                // Serialization failure
                "40001" => StoreError::serialization_error("Transaction conflict"),
                _ => StoreError::Query { source: error },
            }
        }
        _ => StoreError::Query { source: error },
    }
}
