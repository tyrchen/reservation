use std::collections::VecDeque;

use crate::{
    Error, FilterPager, Normalizer, Reservation, ReservationFilter, ReservationFilterBuilder,
    ReservationStatus, ToSql, Validator,
};

impl ReservationFilterBuilder {
    pub fn build(&self) -> Result<ReservationFilter, Error> {
        let mut filter = self
            .private_build()
            .expect("failed to build ReservationFilter");
        filter.normalize()?;
        Ok(filter)
    }
}

impl Validator for ReservationFilter {
    fn validate(&self) -> Result<(), Error> {
        if self.page_size < 10 || self.page_size > 100 {
            return Err(Error::InvalidPageSize(self.page_size));
        }

        if let Some(cursor) = self.cursor {
            if cursor < 0 {
                return Err(Error::InvalidCursor(cursor));
            }
        }

        ReservationStatus::from_i32(self.status).ok_or(Error::InvalidStatus(self.status))?;

        Ok(())
    }
}

impl Normalizer for ReservationFilter {
    fn do_normalize(&mut self) {
        if self.status == ReservationStatus::Unknown as i32 {
            self.status = ReservationStatus::Pending as i32;
        }
    }
}

impl ReservationFilter {
    pub fn get_pager(&self, data: &mut VecDeque<Reservation>) -> Result<FilterPager, Error> {
        let has_prev = self.cursor.is_some();
        let start = if has_prev { data.pop_front() } else { None };

        let has_next = data.len() as i64 > self.page_size;
        let end = if has_next { data.pop_back() } else { None };

        let pager = FilterPager {
            prev: start.map(|r| r.id),
            next: end.map(|r| r.id),

            // TODO: how to get total efficiently?
            total: None,
        };

        Ok(pager)
    }

    pub fn get_cursor(&self) -> i64 {
        self.cursor.unwrap_or(if self.desc { i64::MAX } else { 0 })
    }

    pub fn get_status(&self) -> ReservationStatus {
        ReservationStatus::from_i32(self.status).unwrap()
    }
}

impl ToSql for ReservationFilter {
    fn to_sql(&self) -> Result<String, Error> {
        let middle_plus = if self.cursor.is_none() { 0 } else { 1 };

        let mut sql = format!(
            "SELECT * FROM rsvp.reservations WHERE status = '{}'::rsvp.reservation_status AND ",
            self.get_status()
        );
        if self.desc {
            sql.push_str(&format!("id <= {} AND ", self.get_cursor()));
        } else {
            sql.push_str(&format!("id >= {} AND ", self.get_cursor()));
        }

        if self.user_id.is_empty() && self.resource_id.is_empty() {
            sql.push_str("TRUE");
        } else if self.user_id.is_empty() {
            sql.push_str(&format!("resource_id = '{}'", self.resource_id));
        } else if self.resource_id.is_empty() {
            sql.push_str(&format!("user_id = '{}'", self.user_id));
        } else {
            sql.push_str(&format!(
                "user_id = '{}' AND resource_id = '{}'",
                self.user_id, self.resource_id
            ));
        }
        sql.push_str(&format!(
            " ORDER BY id {} LIMIT {}",
            if self.desc { "DESC" } else { "ASC" },
            self.page_size + 1 + middle_plus
        ));
        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReservationFilterBuilder;

    #[test]
    fn filter_should_generate_correct_sql() {
        let filter = ReservationFilterBuilder::default()
            .user_id("tyr")
            .build()
            .unwrap();

        let sql = filter.to_sql().unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM rsvp.reservations WHERE status = 'pending'::rsvp.reservation_status AND id >= 0 AND user_id = 'tyr' ORDER BY id ASC LIMIT 11"
        );

        let filter = ReservationFilterBuilder::default()
            .user_id("tyr")
            .resource_id("test")
            .build()
            .unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM rsvp.reservations WHERE status = 'pending'::rsvp.reservation_status AND id >= 0 AND user_id = 'tyr' AND resource_id = 'test' ORDER BY id ASC LIMIT 11"
        );

        let filter = ReservationFilterBuilder::default()
            .desc(true)
            .build()
            .unwrap();

        let sql = filter.to_sql().unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM rsvp.reservations WHERE status = 'pending'::rsvp.reservation_status AND id <= 9223372036854775807 AND TRUE ORDER BY id DESC LIMIT 11"
        );

        let filter = ReservationFilterBuilder::default()
            .user_id("tyr")
            .cursor(100)
            .build()
            .unwrap();

        let sql = filter.to_sql().unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM rsvp.reservations WHERE status = 'pending'::rsvp.reservation_status AND id >= 100 AND user_id = 'tyr' ORDER BY id ASC LIMIT 12"
        );

        let filter = ReservationFilterBuilder::default()
            .user_id("tyr")
            .cursor(10)
            .desc(true)
            .build()
            .unwrap();

        let sql = filter.to_sql().unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM rsvp.reservations WHERE status = 'pending'::rsvp.reservation_status AND id <= 10 AND user_id = 'tyr' ORDER BY id DESC LIMIT 12"
        );
    }
}
