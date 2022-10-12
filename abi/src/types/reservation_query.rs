use super::{get_timespan, validate_range};
use crate::{Error, ReservationQuery, Validator};
use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgRange;

impl ReservationQuery {
    pub fn get_timespan(&self) -> PgRange<DateTime<Utc>> {
        get_timespan(self.start.as_ref(), self.end.as_ref())
    }
}

impl Validator for ReservationQuery {
    fn validate(&self) -> Result<(), Error> {
        validate_range(self.start.as_ref(), self.end.as_ref())?;

        Ok(())
    }
}
