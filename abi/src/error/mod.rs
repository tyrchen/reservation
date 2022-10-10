mod conflict;

use sqlx::postgres::PgDatabaseError;
use thiserror::Error;

pub use conflict::{ReservationConflict, ReservationConflictInfo, ReservationWindow};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error")]
    DbError(sqlx::Error),
    #[error("Invalid start or end time for the reservation")]
    InvalidTime,

    #[error("Conflict reservation")]
    ConflictReservation(ReservationConflictInfo),

    #[error("No reservation found by the given condition")]
    NotFound,

    #[error("Invalid reservation id: {0}")]
    InvalidReservationId(String),

    #[error("Invalid user id: {0}")]
    InvalidUserId(String),

    #[error("Invalid resource id: {0}")]
    InvalidResourceId(String),

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
