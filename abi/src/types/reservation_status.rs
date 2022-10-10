use crate::{ReservationStatus, RsvpStatus};
use std::fmt;

impl From<RsvpStatus> for ReservationStatus {
    fn from(status: RsvpStatus) -> Self {
        match status {
            RsvpStatus::Unknown => ReservationStatus::Unknown,
            RsvpStatus::Pending => ReservationStatus::Pending,
            RsvpStatus::Confirmed => ReservationStatus::Confirmed,
            RsvpStatus::Blocked => ReservationStatus::Blocked,
        }
    }
}

impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReservationStatus::Pending => write!(f, "pending"),
            ReservationStatus::Blocked => write!(f, "blocked"),
            ReservationStatus::Confirmed => write!(f, "confirmed"),
            ReservationStatus::Unknown => write!(f, "unknown"),
        }
    }
}
