use std::collections::VecDeque;

use crate::{
    pager::{Id, PageInfo, Pager, Paginator},
    Error, FilterPager, Normalizer, ReservationFilter, ReservationFilterBuilder, ReservationStatus,
    ToSql, Validator,
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

impl From<Pager> for FilterPager {
    fn from(pager: Pager) -> Self {
        Self {
            prev: pager.prev,
            next: pager.next,
            total: pager.total,
        }
    }
}

impl From<&FilterPager> for Pager {
    fn from(pager: &FilterPager) -> Self {
        Self {
            prev: pager.prev,
            next: pager.next,
            total: pager.total,
        }
    }
}

impl ReservationFilter {
    pub fn get_pager<T: Id>(&self, data: &mut VecDeque<T>) -> FilterPager {
        let page_info = self.page_info();
        let pager = page_info.get_pager(data);
        pager.into()
    }

    pub fn get_cursor(&self) -> i64 {
        self.cursor.unwrap_or(if self.desc { i64::MAX } else { 0 })
    }

    pub fn get_status(&self) -> ReservationStatus {
        ReservationStatus::from_i32(self.status).unwrap()
    }

    pub fn next_page(&self, pager: &FilterPager) -> Option<Self> {
        let page_info = self.page_info();
        let pager = pager.into();
        let page_info = page_info.next_page(&pager);
        page_info.map(|page_info| Self {
            cursor: page_info.cursor,
            page_size: page_info.page_size,
            desc: page_info.desc,
            status: self.status,
            resource_id: self.resource_id.clone(),
            user_id: self.user_id.clone(),
        })
    }

    fn page_info(&self) -> PageInfo {
        PageInfo {
            cursor: self.cursor,
            page_size: self.page_size,
            desc: self.desc,
        }
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
    use crate::{pager::pager_test_utils::generate_test_ids, ReservationFilterBuilder};

    #[test]
    fn filter_should_generate_correct_pager() {}

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

    #[test]
    fn filter_with_pager_should_generate_correct_sql() {
        let filter = ReservationFilterBuilder::default()
            .resource_id("router-1")
            .build()
            .unwrap();
        let mut data = generate_test_ids(1, 11);
        let pager = filter.get_pager(&mut data);
        assert_eq!(pager.prev, None);
        assert_eq!(pager.next, Some(10));

        let filter = filter.next_page(&pager).unwrap();
        let sql = filter.to_sql().unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM rsvp.reservations WHERE status = 'pending'::rsvp.reservation_status AND id >= 10 AND resource_id = 'router-1' ORDER BY id ASC LIMIT 12"
        );

        let mut data = generate_test_ids(10, 20);
        let pager = filter.get_pager(&mut data);
        assert_eq!(pager.prev, Some(11));
        assert_eq!(pager.next, None);
    }
}