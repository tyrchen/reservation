#[path = "../src/test_utils.rs"]
mod test_utils;

use abi::{
    reservation_service_client::ReservationServiceClient, Config, ConfirmRequest, FilterRequest,
    FilterResponse, QueryRequest, Reservation, ReservationFilterBuilder, ReservationQueryBuilder,
    ReservationStatus, ReserveRequest,
};
use futures::StreamExt;
use reservation_service::start_server;
use std::time::Duration;
use test_utils::TestConfig;
use tokio::time;
use tonic::transport::Channel;

#[tokio::test]
async fn grpc_server_should_work() {
    let tconfig = TestConfig::with_server_port(50000);
    let mut client = get_test_client(&tconfig).await;

    // first we make a reservation
    let mut rsvp = Reservation::new_pending(
        "tyr",
        "ixia-3230",
        "2022-12-26T15:00:00-0700".parse().unwrap(),
        "2022-12-30T12:00:00-0700".parse().unwrap(),
        "test device reservation",
    );
    let ret = client
        .reserve(ReserveRequest::new(rsvp.clone()))
        .await
        .unwrap()
        .into_inner()
        .reservation
        .unwrap();

    rsvp.id = ret.id;
    assert_eq!(ret, rsvp);

    // then we try to make a conflicting reservation
    let rsvp2 = Reservation::new_pending(
        "tyr",
        "ixia-3230",
        "2022-12-26T15:00:00-0700".parse().unwrap(),
        "2022-12-30T12:00:00-0700".parse().unwrap(),
        "test device reservation",
    );
    let ret = client.reserve(ReserveRequest::new(rsvp2.clone())).await;

    assert!(ret.is_err());

    // then we confirm first reservation
    let ret = client
        .confirm(ConfirmRequest::new(rsvp.id))
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        ret.reservation.unwrap().status,
        ReservationStatus::Confirmed as i32
    );
}

#[tokio::test]
async fn grpc_query_should_work() {
    let tconfig = TestConfig::with_server_port(50001);
    let mut client = get_test_client(&tconfig).await;
    make_reservations(&mut client, 100).await;

    let query = ReservationQueryBuilder::default()
        .user_id("alice")
        .build()
        .unwrap();
    // query for all reservations
    let mut ret = client
        .query(QueryRequest::new(query))
        .await
        .unwrap()
        .into_inner();

    while let Some(Ok(rsvp)) = ret.next().await {
        assert_eq!(rsvp.user_id, "alice");
    }
}

#[tokio::test]
async fn grpc_filter_should_work() {
    let tconfig = TestConfig::with_server_port(50002);
    let mut client = get_test_client(&tconfig).await;

    make_reservations(&mut client, 25).await;

    // then we filter by user
    let filter = ReservationFilterBuilder::default()
        .user_id("alice")
        .status(abi::ReservationStatus::Pending as i32)
        .build()
        .unwrap();
    let FilterResponse {
        pager,
        reservations,
    } = client
        .filter(FilterRequest::new(filter.clone()))
        .await
        .unwrap()
        .into_inner();

    let pager = pager.unwrap();

    assert_eq!(pager.next, Some(filter.page_size));
    assert_eq!(pager.prev, None);
    assert_eq!(pager.total, None);

    assert_eq!(reservations.len(), filter.page_size as usize);

    let filter = filter.next_page(&pager).unwrap();
    // then we get next page
    let FilterResponse {
        pager,
        reservations,
    } = client
        .filter(FilterRequest::new(filter.clone()))
        .await
        .unwrap()
        .into_inner();
    let pager = pager.unwrap();

    assert_eq!(pager.next, filter.cursor.map(|v| v + filter.page_size));
    assert_eq!(pager.prev, filter.cursor.map(|v| v + 1));

    assert_eq!(reservations.len(), filter.page_size as usize);

    // then we get next page (last page)
    let filter = filter.next_page(&pager).unwrap();
    let FilterResponse {
        pager,
        reservations,
    } = client
        .filter(FilterRequest::new(filter.clone()))
        .await
        .unwrap()
        .into_inner();
    let pager = pager.unwrap();

    assert_eq!(pager.next, None);
    assert_eq!(pager.prev, filter.cursor.map(|v| v + 1));

    assert_eq!(reservations.len(), 5);
}

async fn get_test_client(tconfig: &TestConfig) -> ReservationServiceClient<Channel> {
    let config = &tconfig.config;
    setup_server(config);

    let fut = async move {
        // if error on conn keep retry until timeout
        while ReservationServiceClient::connect(config.server.url(false))
            .await
            .is_err()
        {
            time::sleep(Duration::from_millis(10)).await;
        }
        ReservationServiceClient::connect(config.server.url(false))
            .await
            .unwrap()
    };

    time::timeout(Duration::from_secs(5), fut).await.unwrap()
}

fn setup_server(config: &Config) {
    let config_cloned = config.clone();
    tokio::spawn(async move {
        start_server(&config_cloned).await.unwrap();
    });
}

async fn make_reservations(client: &mut ReservationServiceClient<Channel>, count: u32) {
    // then we make 100 reservations without confliction
    for i in 0..count {
        let mut rsvp = Reservation::new_pending(
            "alice",
            format!("router-{}", i),
            "2022-12-26T15:00:00-0700".parse().unwrap(),
            "2022-12-30T12:00:00-0700".parse().unwrap(),
            &format!("test device reservation {}", i),
        );
        let ret = client
            .reserve(ReserveRequest::new(rsvp.clone()))
            .await
            .unwrap()
            .into_inner()
            .reservation
            .unwrap();

        rsvp.id = ret.id;
        assert_eq!(ret, rsvp);
    }
}
