mod service;

use abi::Reservation;
use futures::Stream;
use reservation::ReservationManager;
use std::pin::Pin;
use tonic::Status;

pub struct RsvpService {
    manager: ReservationManager,
}

type ReservationStream = Pin<Box<dyn Stream<Item = Result<Reservation, Status>> + Send>>;
