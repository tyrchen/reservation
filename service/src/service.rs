use std::{pin::Pin, task::Poll};

use abi::{
    reservation_service_server::ReservationService, CancelRequest, CancelResponse, Config,
    ConfirmRequest, ConfirmResponse, FilterRequest, FilterResponse, GetRequest, GetResponse,
    ListenRequest, QueryRequest, ReserveRequest, ReserveResponse, UpdateRequest, UpdateResponse,
};
use futures::Stream;
use reservation::{ReservationManager, Rsvp};
use tokio::sync::mpsc;
use tonic::{async_trait, Request, Response, Status};

use crate::{ReservationStream, RsvpService, TonicReceiverStream};

impl RsvpService {
    pub async fn from_config(config: &Config) -> Result<Self, anyhow::Error> {
        Ok(Self {
            manager: ReservationManager::from_config(&config.db).await?,
        })
    }
}

#[async_trait]
impl ReservationService for RsvpService {
    /// make a reservation
    async fn reserve(
        &self,
        request: Request<ReserveRequest>,
    ) -> Result<Response<ReserveResponse>, Status> {
        let request = request.into_inner();
        if request.reservation.is_none() {
            return Err(Status::invalid_argument("missing reservation"));
        }
        let reservation = self.manager.reserve(request.reservation.unwrap()).await?;
        Ok(Response::new(ReserveResponse {
            reservation: Some(reservation),
        }))
    }

    /// confirm a pending reservation, if reservation is not pending, do nothing
    async fn confirm(
        &self,
        request: Request<ConfirmRequest>,
    ) -> Result<Response<ConfirmResponse>, Status> {
        let request = request.into_inner();
        let reservation = self.manager.change_status(request.id).await?;
        Ok(Response::new(ConfirmResponse {
            reservation: Some(reservation),
        }))
    }

    /// update the reservation note
    async fn update(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        let request = request.into_inner();
        let reservation = self.manager.update_note(request.id, request.note).await?;
        Ok(Response::new(UpdateResponse {
            reservation: Some(reservation),
        }))
    }

    /// cancel a reservation
    async fn cancel(
        &self,
        request: Request<CancelRequest>,
    ) -> Result<Response<CancelResponse>, Status> {
        let request = request.into_inner();
        let reservation = self.manager.delete(request.id).await?;
        Ok(Response::new(CancelResponse {
            reservation: Some(reservation),
        }))
    }

    /// get a reservation by id
    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let request = request.into_inner();
        let reservation = self.manager.get(request.id).await?;
        Ok(Response::new(GetResponse {
            reservation: Some(reservation),
        }))
    }

    ///Server streaming response type for the query method.
    type queryStream = ReservationStream;
    /// query reservations by resource id, user id, status, start time, end time
    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<Self::queryStream>, Status> {
        let request = request.into_inner();
        if request.query.is_none() {
            return Err(Status::invalid_argument("missing query params"));
        }
        let rsvps = self.manager.query(request.query.unwrap()).await;
        let stream = TonicReceiverStream::new(rsvps);
        Ok(Response::new(Box::pin(stream)))
    }

    /// filter reservations, order by reservation id
    async fn filter(
        &self,
        request: Request<FilterRequest>,
    ) -> Result<Response<FilterResponse>, Status> {
        let request = request.into_inner();

        if request.filter.is_none() {
            return Err(Status::invalid_argument("missing filter params"));
        }
        let (pager, reservations) = self.manager.filter(request.filter.unwrap()).await?;
        Ok(Response::new(FilterResponse {
            pager: Some(pager),
            reservations,
        }))
    }

    ///Server streaming response type for the listen method.
    type listenStream = ReservationStream;
    /// another system could monitor newly added/confirmed/cancelled reservations
    async fn listen(
        &self,
        _request: Request<ListenRequest>,
    ) -> Result<Response<Self::listenStream>, Status> {
        todo!()
    }
}

impl<T> TonicReceiverStream<T> {
    pub fn new(inner: mpsc::Receiver<Result<T, abi::Error>>) -> Self {
        Self { inner }
    }
}

impl<T> Stream for TonicReceiverStream<T> {
    type Item = Result<T, Status>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.inner.poll_recv(cx) {
            Poll::Ready(Some(Ok(item))) => Poll::Ready(Some(Ok(item))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::Reservation;
    use sqlx_db_tester::TestDb;
    use std::ops::Deref;

    struct TestConfig {
        #[allow(dead_code)]
        tdb: TestDb,
        pub config: Config,
    }

    impl Deref for TestConfig {
        type Target = Config;

        fn deref(&self) -> &Self::Target {
            &self.config
        }
    }

    impl TestConfig {
        pub fn new() -> Self {
            let mut config = Config::load("fixtures/config.yml").unwrap();
            let tdb = TestDb::new(
                &config.db.host,
                config.db.port,
                &config.db.user,
                &config.db.password,
                "../migrations",
            );

            config.db.dbname = tdb.dbname.clone();
            Self { tdb, config }
        }
    }

    #[tokio::test]
    async fn rpc_reserve_should_work() {
        let config = TestConfig::new();

        let service = RsvpService::from_config(&config).await.unwrap();
        let reservation = Reservation::new_pending(
            "tyr",
            "ixia-3230",
            "2022-12-26T15:00:00-0700".parse().unwrap(),
            "2022-12-30T12:00:00-0700".parse().unwrap(),
            "test device reservation",
        );
        let request = tonic::Request::new(ReserveRequest {
            reservation: Some(reservation.clone()),
        });
        let response = service.reserve(request).await.unwrap();
        let reservation1 = response.into_inner().reservation;
        assert!(reservation1.is_some());
        let reservation1 = reservation1.unwrap();
        assert_eq!(reservation1.user_id, reservation.user_id);
        assert_eq!(reservation1.resource_id, reservation.resource_id);
        assert_eq!(reservation1.start, reservation.start);
        assert_eq!(reservation1.end, reservation.end);
        assert_eq!(reservation1.note, reservation.note);
        assert_eq!(reservation1.status, reservation.status);
    }
}
