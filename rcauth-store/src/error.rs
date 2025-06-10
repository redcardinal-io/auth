use rcauth_core::error::Error as AppError;
use snafu::prelude::*;
use sqlx::postgres::PgDatabaseError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
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

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn conflict(message: impl Into<String>) -> Self {
        Error::Conflict {
            message: message.into(),
        }
    }

    pub fn serialization_error(message: impl Into<String>) -> Self {
        Error::Serialization {
            message: message.into(),
        }
    }
}

// Handle common SQLx error cases
pub fn handle_sqlx_error(error: sqlx::Error) -> Error {
    match &error {
        sqlx::Error::RowNotFound => Error::NotFound,
        sqlx::Error::Database(db_err) => {
            let pg_err = db_err.downcast_ref::<PgDatabaseError>();
            match pg_err.code() {
                // Unique violation
                "23505" => Error::conflict("Record already exists"),
                // Foreign key violation
                "23503" => Error::conflict("Related record not found"),
                // Serialization failure
                "40001" => Error::serialization_error("Transaction conflict"),
                _ => Error::Query { source: error },
            }
        }
        _ => Error::Query { source: error },
    }
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        use rcauth_core::error::ErrorCode;

        match error {
            Error::Connection { source } => AppError::new(
                ErrorCode::DatabaseError,
                "Database connection failed",
                source,
            ),
            Error::Query { source } => {
                AppError::new(ErrorCode::DatabaseError, "Database query failed", source)
            }
            Error::Transaction { source } => AppError::new(
                ErrorCode::DatabaseError,
                "Database transaction failed",
                source,
            ),
            Error::NotFound => AppError::new_simple(ErrorCode::NotFound, "Record not found"),
            Error::Conflict { message } => {
                AppError::new_simple(ErrorCode::Conflict, format!("Conflict: {}", message))
            }
            Error::Migration { source } => AppError::new(
                ErrorCode::DatabaseError,
                "Database migration failed",
                source,
            ),
            Error::Serialization { message } => AppError::new_simple(
                ErrorCode::Conflict,
                format!("Serialization error: {}", message),
            )
            .with_internal(format!("DB serialization conflict: {}", message)),
        }
    }
}
