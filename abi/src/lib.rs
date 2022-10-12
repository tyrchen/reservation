mod error;
mod pb;
mod types;
mod utils;

pub use error::{Error, ReservationConflict, ReservationConflictInfo, ReservationWindow};
pub use pb::*;

pub use utils::*;

pub trait Validator {
    fn validate(&self) -> Result<(), Error>;
}

/// database equivalent of the "reservation_status" enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "reservation_status", rename_all = "lowercase")]
pub enum RsvpStatus {
    Unknown,
    Pending,
    Confirmed,
    Blocked,
}
