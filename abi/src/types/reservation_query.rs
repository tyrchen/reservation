use crate::{
    convert_to_utc_time, Error, Normalizer, ReservationQuery, ReservationQueryBuilder,
    ReservationStatus, ToSql, Validator,
};
use prost_types::Timestamp;

impl ReservationQueryBuilder {
    pub fn build(&self) -> Result<ReservationQuery, Error> {
        let mut query = self
            .private_build()
            .expect("failed to build ReservationQuery");
        query.normalize()?;
        Ok(query)
    }
}

impl ReservationQuery {
    pub fn get_status(&self) -> ReservationStatus {
        ReservationStatus::from_i32(self.status).unwrap()
    }
}

impl Validator for ReservationQuery {
    fn validate(&self) -> Result<(), Error> {
        ReservationStatus::from_i32(self.status).ok_or(Error::InvalidStatus(self.status))?;

        if let (Some(start), Some(end)) = (self.start.as_ref(), self.end.as_ref()) {
            if start.seconds >= end.seconds {
                return Err(Error::InvalidTime);
            }
        }

        Ok(())
    }
}

impl Normalizer for ReservationQuery {
    fn do_normalize(&mut self) {
        if self.status == ReservationStatus::Unknown as i32 {
            self.status = ReservationStatus::Pending as i32;
        }
    }
}

impl ToSql for ReservationQuery {
    fn to_sql(&self) -> String {
        let status = self.get_status();

        let timespan = format!(
            "tstzrange('{}', '{}')",
            get_time_string(self.start.as_ref(), true),
            get_time_string(self.end.as_ref(), false)
        );

        let condition = match (self.user_id.is_empty(), self.resource_id.is_empty()) {
            (true, true) => "TRUE".into(),
            (true, false) => format!("resource_id = '{}'", self.resource_id),
            (false, true) => format!("user_id = '{}'", self.user_id),
            (false, false) => format!(
                "user_id = '{}' AND resource_id = '{}'",
                self.user_id, self.resource_id
            ),
        };

        let direction = if self.desc { "DESC" } else { "ASC" };

        format!("SELECT * FROM rsvp.reservations WHERE {} @> timespan AND status = '{}'::rsvp.reservation_status AND {} ORDER BY lower(timespan) {}", timespan, status, condition, direction)
    }
}

fn get_time_string(ts: Option<&Timestamp>, start: bool) -> String {
    match ts {
        Some(ts) => convert_to_utc_time(ts).to_rfc3339(),
        None => (if start { "-infinity" } else { "infinity" }).into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_should_generate_valid_sql() {
        let query = ReservationQueryBuilder::default()
            .user_id("tyr")
            .build()
            .unwrap();

        let sql = query.to_sql();

        assert_eq!(sql, "SELECT * FROM rsvp.reservations WHERE tstzrange('-infinity', 'infinity') @> timespan AND status = 'pending'::rsvp.reservation_status AND user_id = 'tyr' ORDER BY lower(timespan) ASC");

        let query = ReservationQueryBuilder::default()
            .resource_id("test")
            .start("2021-11-01T15:00:00-0700".parse::<Timestamp>().unwrap())
            .build()
            .unwrap();

        let sql = query.to_sql();
        assert_eq!(sql, "SELECT * FROM rsvp.reservations WHERE tstzrange('2021-11-01T22:00:00+00:00', 'infinity') @> timespan AND status = 'pending'::rsvp.reservation_status AND resource_id = 'test' ORDER BY lower(timespan) ASC");

        let query = ReservationQueryBuilder::default()
            .end("2021-11-01T16:00:00-0700".parse::<Timestamp>().unwrap())
            .build()
            .unwrap();

        let sql = query.to_sql();
        assert_eq!(sql, "SELECT * FROM rsvp.reservations WHERE tstzrange('-infinity', '2021-11-01T23:00:00+00:00') @> timespan AND status = 'pending'::rsvp.reservation_status AND TRUE ORDER BY lower(timespan) ASC");
    }
}
