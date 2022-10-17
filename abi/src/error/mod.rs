mod conflict;

use sqlx::postgres::PgDatabaseError;

pub use conflict::{ReservationConflict, ReservationConflictInfo, ReservationWindow};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error")]
    DbError(sqlx::Error),

    #[error("Failed to read configuration file")]
    ConfigReadError,

    #[error("Failed to parse configuration file")]
    ConfigParseError,

    #[error("Invalid start or end time for the reservation")]
    InvalidTime,

    #[error("Conflict reservation")]
    ConflictReservation(ReservationConflictInfo),

    #[error("No reservation found by the given condition")]
    NotFound,

    #[error("Invalid reservation id: {0}")]
    InvalidReservationId(i64),

    #[error("Invalid user id: {0}")]
    InvalidUserId(String),

    #[error("Invalid resource id: {0}")]
    InvalidResourceId(String),

    #[error("Invalid page size: {0}")]
    InvalidPageSize(i64),

    #[error("Invalid cursor: {0}")]
    InvalidCursor(i64),

    #[error("Invalid status: {0}")]
    InvalidStatus(i32),

    #[error("unknown error")]
    Unknown,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // TODO: this is not a good way to compare DB errors, but we don't do that in the code
            (Self::DbError(_), Self::DbError(_)) => true,
            (Self::InvalidTime, Self::InvalidTime) => true,
            (Self::ConflictReservation(v1), Self::ConflictReservation(v2)) => v1 == v2,
            (Self::NotFound, Self::NotFound) => true,
            (Self::InvalidReservationId(v1), Self::InvalidReservationId(v2)) => v1 == v2,
            (Self::InvalidUserId(v1), Self::InvalidUserId(v2)) => v1 == v2,
            (Self::InvalidResourceId(v1), Self::InvalidResourceId(v2)) => v1 == v2,
            (Self::Unknown, Self::Unknown) => true,
            _ => false,
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e) => {
                let err: &PgDatabaseError = e.downcast_ref();
                match (err.code(), err.schema(), err.table()) {
                    ("23P01", Some("rsvp"), Some("reservations")) => {
                        Error::ConflictReservation(err.detail().unwrap().parse().unwrap())
                    }
                    _ => Error::DbError(sqlx::Error::Database(e)),
                }
            }
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DbError(e),
        }
    }
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        match e {
            Error::DbError(_) | Error::ConfigReadError | Error::ConfigParseError => {
                tonic::Status::internal(e.to_string())
            }
            Error::InvalidTime
            | Error::InvalidReservationId(_)
            | Error::InvalidUserId(_)
            | Error::InvalidResourceId(_)
            | Error::InvalidPageSize(_)
            | Error::InvalidCursor(_)
            | Error::InvalidStatus(_) => tonic::Status::invalid_argument(e.to_string()),
            Error::ConflictReservation(info) => {
                tonic::Status::failed_precondition(format!("Conflict reservation: {:?}", info))
            }
            Error::NotFound => {
                tonic::Status::not_found("No reservation found by the given condition")
            }
            Error::Unknown => tonic::Status::unknown("unknown error"),
        }
    }
}
