use crate::{
    CancelRequest, ConfirmRequest, FilterRequest, GetRequest, Reservation, ReservationFilter,
    ReserveRequest, UpdateRequest,
};

macro_rules! impl_new {
    ($name:ident, $field:ident, $type:ty) => {
        impl $name {
            pub fn new(value: $type) -> Self {
                Self {
                    $field: Some(value),
                }
            }
        }
    };
    ($name:ident) => {
        impl $name {
            pub fn new(id: i64) -> Self {
                Self { id }
            }
        }
    };
}

impl_new!(ReserveRequest, reservation, Reservation);
impl_new!(FilterRequest, filter, ReservationFilter);
impl_new!(ConfirmRequest);
impl_new!(GetRequest);
impl_new!(CancelRequest);

impl UpdateRequest {
    pub fn new(id: i64, note: String) -> Self {
        Self { id, note }
    }
}
