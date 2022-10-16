use crate::{ReservationManager, Rsvp};
use abi::{DbConfig, FilterPager, ReservationId, Validator};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use sqlx::{
    postgres::{types::PgRange, PgPoolOptions},
    Either, PgPool, Row,
};
use tokio::sync::mpsc;
use tracing::{info, trace, warn};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        rsvp.validate()?;

        let status = abi::ReservationStatus::from_i32(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let timespan: PgRange<DateTime<Utc>> = rsvp.get_timespan();

        // generate a insert sql for the reservation
        // execute the sql
        let id = sqlx::query(
            "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?.get(0);

        rsvp.id = id;

        Ok(rsvp)
    }

    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        // if current status is pending, change it to confirmed, otherwise do nothing
        id.validate()?;
        let rsvp: abi::Reservation = sqlx::query_as(
            "UPDATE rsvp.reservations SET status = 'confirmed' WHERE id = $1 AND status = 'pending' RETURNING *"
        ).bind(id).fetch_one(&self.pool).await?;

        Ok(rsvp)
    }

    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, abi::Error> {
        // update the note of the reservation
        id.validate()?;
        let rsvp: abi::Reservation =
            sqlx::query_as("UPDATE rsvp.reservations SET note = $1 WHERE id = $2 RETURNING *")
                .bind(note)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(rsvp)
    }

    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        // get the reservation by id
        id.validate()?;
        let rsvp: abi::Reservation =
            sqlx::query_as("SELECT * FROM rsvp.reservations WHERE id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(rsvp)
    }

    async fn delete(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        // delete the reservation by id
        id.validate()?;
        let rsvp: abi::Reservation =
            sqlx::query_as("DELETE FROM rsvp.reservations WHERE id = $1 RETURNING *")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;
        Ok(rsvp)
    }

    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> mpsc::Receiver<Result<abi::Reservation, abi::Error>> {
        let user_id = string_to_option(&query.user_id);
        let resource_id = string_to_option(&query.resource_id);
        let range: PgRange<DateTime<Utc>> = query.get_timespan();
        let status = abi::ReservationStatus::from_i32(query.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let pool = self.pool.clone();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            let mut rsvps = sqlx::query_as(
                "SELECT * FROM rsvp.query($1, $2, $3, $4::rsvp.reservation_status, $5, $6, $7)",
            )
            .bind(user_id)
            .bind(resource_id)
            .bind(range)
            .bind(status.to_string())
            .bind(query.page)
            .bind(query.desc)
            .bind(query.page_size)
            .fetch_many(&pool);
            while let Some(ret) = rsvps.next().await {
                match ret {
                    Ok(Either::Left(r)) => {
                        info!("Query result: {:?}", r);
                    }
                    Ok(Either::Right(r)) => {
                        if tx.send(Ok(r)).await.is_err() {
                            // rx is dropped, so client disconnected
                            break;
                        }
                    }
                    Err(e) => {
                        warn!("Query error: {:?}", e);
                        if tx.send(Err(e.into())).await.is_err() {
                            // rx is dropped, so client disconnected
                            break;
                        }
                    }
                }
            }
        });

        rx
    }

    async fn filter(
        &self,
        filter: abi::ReservationFilter,
    ) -> Result<(FilterPager, Vec<abi::Reservation>), abi::Error> {
        // filter reservations by user_id, resource_id, status, and order by id
        let user_id = str_to_option(&filter.user_id);
        let resource_id = str_to_option(&filter.resource_id);
        let status = abi::ReservationStatus::from_i32(filter.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let page_size = if filter.page_size < 10 || filter.page_size > 100 {
            10
        } else {
            filter.page_size
        };
        let rsvps: Vec<abi::Reservation> = sqlx::query_as(
            "SELECT * FROM rsvp.filter($1, $2, $3::rsvp.reservation_status, $4, $5, $6)",
        )
        .bind(user_id)
        .bind(resource_id)
        .bind(status.to_string())
        .bind(filter.cursor)
        .bind(filter.desc)
        .bind(page_size)
        .fetch_all(&self.pool)
        .await?;

        // if the first id is current cursor, then we have prev, we start from 1
        // if len - start > page_size, then we have next, we end at len - 1
        let has_prev = !rsvps.is_empty() && rsvps[0].id == filter.cursor;
        let start = if has_prev { 1 } else { 0 };

        let has_next = (rsvps.len() - start) as i64 > page_size;
        let end = if has_next {
            rsvps.len() - 1
        } else {
            rsvps.len()
        };

        trace!("start: {}, rsvp len: {}, end: {}", start, rsvps.len(), end);

        let prev = if has_prev { rsvps[start - 1].id } else { -1 };
        let next = if has_next { rsvps[end - 1].id } else { -1 };

        // TODO: optimize this clone
        let result = rsvps[start..end].to_vec();

        let pager = FilterPager {
            next,
            prev,
            // TODO: how to get total efficiently?
            total: 0,
        };
        Ok((pager, result))
    }
}

impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn from_config(config: &DbConfig) -> Result<Self, abi::Error> {
        let url = config.url();
        let pool = PgPoolOptions::default()
            .max_connections(config.max_connections)
            .connect(&url)
            .await?;
        Ok(Self::new(pool))
    }
}

fn str_to_option(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn string_to_option(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.into())
    }
}

#[cfg(test)]
mod tests {
    use abi::{
        Reservation, ReservationConflict, ReservationConflictInfo, ReservationFilterBuilder,
        ReservationQueryBuilder, ReservationWindow,
    };
    use prost_types::Timestamp;

    use super::*;

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_valid_window() {
        let (rsvp, _manager) = make_tyr_reservation(migrated_pool.clone()).await;
        assert!(rsvp.id != 0);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_conflict_reservation_should_reject() {
        let (_rsvp1, manager) = make_tyr_reservation(migrated_pool.clone()).await;
        let rsvp2 = abi::Reservation::new_pending(
            "aliceid",
            "ocean-view-room-713",
            "2022-12-26T15:00:00-0700".parse().unwrap(),
            "2022-12-30T12:00:00-0700".parse().unwrap(),
            "hello.",
        );

        let err = manager.reserve(rsvp2).await.unwrap_err();

        let info = ReservationConflictInfo::Parsed(ReservationConflict {
            new: ReservationWindow {
                rid: "ocean-view-room-713".to_string(),
                start: "2022-12-26T15:00:00-0700".parse().unwrap(),
                end: "2022-12-30T12:00:00-0700".parse().unwrap(),
            },
            old: ReservationWindow {
                rid: "ocean-view-room-713".to_string(),
                start: "2022-12-25T15:00:00-0700".parse().unwrap(),
                end: "2022-12-28T12:00:00-0700".parse().unwrap(),
            },
        });

        assert_eq!(err, abi::Error::ConflictReservation(info));
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_change_status_should_work() {
        let (rsvp, manager) = make_alice_reservation(migrated_pool.clone()).await;

        let rsvp = manager.change_status(rsvp.id).await.unwrap();
        assert_eq!(rsvp.status, abi::ReservationStatus::Confirmed as i32);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_change_status_not_pending_should_do_nothging() {
        let (rsvp, manager) = make_alice_reservation(migrated_pool.clone()).await;

        let rsvp = manager.change_status(rsvp.id).await.unwrap();

        // change status again should do nothing
        let ret = manager.change_status(rsvp.id).await.unwrap_err();
        assert_eq!(ret, abi::Error::NotFound);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn update_note_should_work() {
        let (rsvp, manager) = make_alice_reservation(migrated_pool.clone()).await;
        let rsvp = manager
            .update_note(rsvp.id, "hello world".into())
            .await
            .unwrap();
        assert_eq!(rsvp.note, "hello world");
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn get_reservation_should_work() {
        let (rsvp, manager) = make_alice_reservation(migrated_pool.clone()).await;
        let rsvp1 = manager.get(rsvp.id).await.unwrap();
        assert_eq!(rsvp, rsvp1);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn delete_reservation_should_work() {
        let (rsvp, manager) = make_alice_reservation(migrated_pool.clone()).await;
        manager.delete(rsvp.id).await.unwrap();
        let rsvp1 = manager.get(rsvp.id).await.unwrap_err();
        assert_eq!(rsvp1, abi::Error::NotFound);
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn query_reservations_should_work() {
        let (rsvp, manager) = make_alice_reservation(migrated_pool.clone()).await;
        let query = ReservationQueryBuilder::default()
            .user_id("aliceid")
            .start("2021-11-01T15:00:00-0700".parse::<Timestamp>().unwrap())
            .end("2023-12-31T12:00:00-0700".parse::<Timestamp>().unwrap())
            .status(abi::ReservationStatus::Pending as i32)
            .build()
            .unwrap();

        let mut rx = manager.query(query).await;
        assert_eq!(rx.recv().await, Some(Ok(rsvp.clone())));
        assert_eq!(rx.recv().await, None);

        // if window is not in range, should return empty
        let query = ReservationQueryBuilder::default()
            .user_id("aliceid")
            .start("2023-01-01T15:00:00-0700".parse::<Timestamp>().unwrap())
            .end("2023-02-01T12:00:00-0700".parse::<Timestamp>().unwrap())
            .status(abi::ReservationStatus::Confirmed as i32)
            .build()
            .unwrap();
        let mut rx = manager.query(query).await;
        assert_eq!(rx.recv().await, None);

        // if status is not in correct, should return empty
        let query = ReservationQueryBuilder::default()
            .user_id("aliceid")
            .start("2021-11-01T15:00:00-0700".parse::<Timestamp>().unwrap())
            .end("2023-12-31T12:00:00-0700".parse::<Timestamp>().unwrap())
            .status(abi::ReservationStatus::Confirmed as i32)
            .build()
            .unwrap();
        let mut rx = manager.query(query.clone()).await;
        assert_eq!(rx.recv().await, None);

        // change state to confirmed, query should get result
        let rsvp = manager.change_status(rsvp.id).await.unwrap();
        let mut rx = manager.query(query).await;
        assert_eq!(rx.recv().await, Some(Ok(rsvp)));
    }

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn filter_reservations_should_work() {
        let (rsvp, manager) = make_alice_reservation(migrated_pool.clone()).await;
        let filter = ReservationFilterBuilder::default()
            .user_id("aliceid")
            .status(abi::ReservationStatus::Pending as i32)
            .build()
            .unwrap();

        let (pager, rsvps) = manager.filter(filter).await.unwrap();
        assert_eq!(pager.prev, -1);
        assert_eq!(pager.next, -1);
        assert_eq!(rsvps.len(), 1);
        assert_eq!(rsvps[0], rsvp);
    }

    // private none test functions
    async fn make_tyr_reservation(pool: PgPool) -> (Reservation, ReservationManager) {
        make_reservation(
            pool,
            "tyrid",
            "ocean-view-room-713",
            "2022-12-25T15:00:00-0700",
            "2022-12-28T12:00:00-0700",
            "I'll arrive at 3pm. Please help to upgrade to execuitive room if possible.",
        )
        .await
    }

    async fn make_alice_reservation(pool: PgPool) -> (Reservation, ReservationManager) {
        make_reservation(
            pool,
            "aliceid",
            "ixia-test-1",
            "2023-01-25T15:00:00-0700",
            "2023-02-25T12:00:00-0700",
            "I need to book this for xyz project for a month.",
        )
        .await
    }

    async fn make_reservation(
        pool: PgPool,
        uid: &str,
        rid: &str,
        start: &str,
        end: &str,
        note: &str,
    ) -> (Reservation, ReservationManager) {
        let manager = ReservationManager::new(pool.clone());
        let rsvp = abi::Reservation::new_pending(
            uid,
            rid,
            start.parse().unwrap(),
            end.parse().unwrap(),
            note,
        );

        (manager.reserve(rsvp).await.unwrap(), manager)
    }
}
