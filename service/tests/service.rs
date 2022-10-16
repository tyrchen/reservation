#[path = "../src/test_utils.rs"]
mod test_utils;

use abi::{
    reservation_service_client::ReservationServiceClient, ConfirmRequest, FilterRequest,
    FilterResponse, Reservation, ReservationFilterBuilder, ReservationStatus, ReserveRequest,
};
use reservation_service::start_server;
use std::time::Duration;
use test_utils::TestConfig;
use tokio::time;

#[tokio::test]
async fn grpc_server_should_work() {
    let tconfig = TestConfig::default();
    let config = tconfig.config.clone();
    let config_cloned = config.clone();
    tokio::spawn(async move {
        start_server(&config_cloned).await.unwrap();
    });
    time::sleep(Duration::from_millis(100)).await;

    let mut client = ReservationServiceClient::connect(config.server.url(false))
        .await
        .unwrap();

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

    // then we make 100 reservations without confliction
    for i in 0..100 {
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

    assert_eq!(pager.next, filter.page_size + 1 + 1); // we already had an item
    assert_eq!(pager.prev, -1);
    // assert_eq!(pager.total, 100); // not implemented yet

    assert_eq!(reservations.len(), filter.page_size as usize);

    let mut next_filter = filter.clone();
    next_filter.cursor = pager.next;
    // then we get next page
    // let FilterResponse {
    //     pager,
    //     reservations,
    // } = client
    //     .filter(FilterRequest::new(next_filter.clone()))
    //     .await
    //     .unwrap()
    //     .into_inner();
    // let pager = pager.unwrap();

    // println!("pager: {:?}", pager);
    // println!("reservations: {:?}", reservations);

    // assert_eq!(pager.next, next_filter.cursor + filter.page_size);
    // assert_eq!(pager.prev, next_filter.cursor - 1);

    // assert_eq!(reservations.len(), filter.page_size as usize);
}
