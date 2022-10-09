use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    #[error("Database error")]
    DbError(#[from] sqlx::Error),

    #[error("Invalid start or end time for the reservation")]
    InvalidTime,

    #[error("unknown error")]
    Unknown,
}
