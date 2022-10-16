/// Core reservation object. Contains all the information for a reservation
/// if ListenResponse op is DELETE, only id will be populated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reservation {
    /// unique id for the reservation, if put into ReservationRequest, id should be empty
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// user id for the reservation
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// reservation status, used for differentating purpose
    #[prost(enumeration = "ReservationStatus", tag = "3")]
    pub status: i32,
    /// resource id for the reservation
    #[prost(string, tag = "4")]
    pub resource_id: ::prost::alloc::string::String,
    /// start time for the reservation
    #[prost(message, optional, tag = "5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    /// end time for the reservation
    #[prost(message, optional, tag = "6")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    /// extra note
    #[prost(string, tag = "7")]
    pub note: ::prost::alloc::string::String,
}
/// To make a reservation, send a ReservationRequest with Reservation object (id should be empty)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveRequest {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// Created reservation will be returned in ReserveResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To update a reservation, send an UpdateRequest. Only note is updatable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub note: ::prost::alloc::string::String,
}
/// Updated reservation will be returned in UpdateResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To change a reservation from pending to confirmed, send a ConfirmRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// Confirmed reservation will be returned in ConfirmResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To cancel a reservation, send a CancelRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// Canceled reservation will be returned in CancelResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To get a reservation, send a GetRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// Reservation will be returned in GetResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// query reservations with user id, resource id, start time, end time, and status
#[derive(derive_builder::Builder, Clone, PartialEq, ::prost::Message)]
pub struct ReservationQuery {
    /// resource id for the reservation query. If empty, query all resources
    #[prost(string, tag = "1")]
    #[builder(setter(into), default)]
    pub resource_id: ::prost::alloc::string::String,
    /// user id for the reservation query. If empty, query all users
    #[prost(string, tag = "2")]
    #[builder(setter(into), default)]
    pub user_id: ::prost::alloc::string::String,
    /// use status to filter result. If UNKNOWN, return all reservations
    #[prost(enumeration = "ReservationStatus", tag = "3")]
    #[builder(setter(into), default)]
    pub status: i32,
    /// start time for the reservation query, if 0, use Infinity for start time
    #[prost(message, optional, tag = "4")]
    #[builder(setter(into, strip_option))]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    /// end time for the reservation query, if 0, use Infinity for end time
    #[prost(message, optional, tag = "5")]
    #[builder(setter(into, strip_option))]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    /// current page for the query
    #[prost(int32, tag = "6")]
    #[builder(setter(into), default)]
    pub page: i32,
    /// page size for the query
    #[prost(int32, tag = "7")]
    #[builder(setter(into), default)]
    pub page_size: i32,
    /// sort direction
    #[prost(bool, tag = "8")]
    #[builder(setter(into), default)]
    pub desc: bool,
}
/// To query reservations, send a QueryRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<ReservationQuery>,
}
/// query reservations, order by reservation id
#[derive(derive_builder::Builder, Clone, PartialEq, ::prost::Message)]
pub struct ReservationFilter {
    /// resource id for the reservation query. If empty, query all resources
    #[prost(string, tag = "1")]
    #[builder(setter(into), default)]
    pub resource_id: ::prost::alloc::string::String,
    /// user id for the reservation query. If empty, query all users
    #[prost(string, tag = "2")]
    #[builder(setter(into), default)]
    pub user_id: ::prost::alloc::string::String,
    /// use status to filter result. If UNKNOWN, return all reservations
    #[prost(enumeration = "ReservationStatus", tag = "3")]
    #[builder(setter(into), default)]
    pub status: i32,
    #[prost(int64, tag = "4")]
    #[builder(setter(into), default)]
    pub cursor: i64,
    /// page size for the query
    #[prost(int32, tag = "5")]
    #[builder(setter(into), default)]
    pub page_size: i32,
    /// sort direction
    #[prost(bool, tag = "6")]
    #[builder(setter(into), default)]
    pub desc: bool,
}
/// To query reservations, send a QueryRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterRequest {
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<ReservationFilter>,
}
/// filter pager info
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterPager {
    #[prost(int64, tag = "1")]
    pub prev: i64,
    #[prost(int64, tag = "2")]
    pub next: i64,
    #[prost(int32, tag = "3")]
    pub total: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterResponse {
    #[prost(message, repeated, tag = "1")]
    pub reservations: ::prost::alloc::vec::Vec<Reservation>,
    #[prost(message, optional, tag = "2")]
    pub pager: ::core::option::Option<FilterPager>,
}
/// Client can listen to reservation updates by sending a ListenRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenRequest {}
/// Server will send ListenResponse to client in streaming response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenResponse {
    /// update type
    #[prost(enumeration = "ReservationUpdateType", tag = "1")]
    pub op: i32,
    /// id for updated reservation
    #[prost(message, optional, tag = "2")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// reservation status for a given time period
#[derive(
    sqlx::Type, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
)]
#[repr(i32)]
pub enum ReservationStatus {
    Unknown = 0,
    Pending = 1,
    Confirmed = 2,
    Blocked = 3,
}
impl ReservationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReservationStatus::Unknown => "RESERVATION_STATUS_UNKNOWN",
            ReservationStatus::Pending => "RESERVATION_STATUS_PENDING",
            ReservationStatus::Confirmed => "RESERVATION_STATUS_CONFIRMED",
            ReservationStatus::Blocked => "RESERVATION_STATUS_BLOCKED",
        }
    }
}
/// when reservation is updated, record the update type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReservationUpdateType {
    Unknown = 0,
    Create = 1,
    Update = 2,
    Delete = 3,
}
impl ReservationUpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReservationUpdateType::Unknown => "RESERVATION_UPDATE_TYPE_UNKNOWN",
            ReservationUpdateType::Create => "RESERVATION_UPDATE_TYPE_CREATE",
            ReservationUpdateType::Update => "RESERVATION_UPDATE_TYPE_UPDATE",
            ReservationUpdateType::Delete => "RESERVATION_UPDATE_TYPE_DELETE",
        }
    }
}
/// Generated client implementations.
pub mod reservation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Reservation service
    #[derive(Debug, Clone)]
    pub struct ReservationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReservationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ReservationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ReservationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ReservationServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// make a reservation
        pub async fn reserve(
            &mut self,
            request: impl tonic::IntoRequest<super::ReserveRequest>,
        ) -> Result<tonic::Response<super::ReserveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/reserve");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// confirm a pending reservation, if reservation is not pending, do nothing
        pub async fn confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmRequest>,
        ) -> Result<tonic::Response<super::ConfirmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/confirm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// update the reservation note
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRequest>,
        ) -> Result<tonic::Response<super::UpdateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/update");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// cancel a reservation
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelRequest>,
        ) -> Result<tonic::Response<super::CancelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/cancel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// get a reservation by id
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/reservation.ReservationService/get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query reservations by resource id, user id, status, start time, end time
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Reservation>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/query");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        /// filter reservations, order by reservation id
        pub async fn filter(
            &mut self,
            request: impl tonic::IntoRequest<super::FilterRequest>,
        ) -> Result<tonic::Response<super::FilterResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/filter");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// another system could monitor newly added/confirmed/cancelled reservations
        pub async fn listen(
            &mut self,
            request: impl tonic::IntoRequest<super::ListenRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Reservation>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/listen");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod reservation_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ReservationServiceServer.
    #[async_trait]
    pub trait ReservationService: Send + Sync + 'static {
        /// make a reservation
        async fn reserve(
            &self,
            request: tonic::Request<super::ReserveRequest>,
        ) -> Result<tonic::Response<super::ReserveResponse>, tonic::Status>;
        /// confirm a pending reservation, if reservation is not pending, do nothing
        async fn confirm(
            &self,
            request: tonic::Request<super::ConfirmRequest>,
        ) -> Result<tonic::Response<super::ConfirmResponse>, tonic::Status>;
        /// update the reservation note
        async fn update(
            &self,
            request: tonic::Request<super::UpdateRequest>,
        ) -> Result<tonic::Response<super::UpdateResponse>, tonic::Status>;
        /// cancel a reservation
        async fn cancel(
            &self,
            request: tonic::Request<super::CancelRequest>,
        ) -> Result<tonic::Response<super::CancelResponse>, tonic::Status>;
        /// get a reservation by id
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
        ///Server streaming response type for the query method.
        type queryStream: futures_core::Stream<Item = Result<super::Reservation, tonic::Status>>
            + Send
            + 'static;
        /// query reservations by resource id, user id, status, start time, end time
        async fn query(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> Result<tonic::Response<Self::queryStream>, tonic::Status>;
        /// filter reservations, order by reservation id
        async fn filter(
            &self,
            request: tonic::Request<super::FilterRequest>,
        ) -> Result<tonic::Response<super::FilterResponse>, tonic::Status>;
        ///Server streaming response type for the listen method.
        type listenStream: futures_core::Stream<Item = Result<super::Reservation, tonic::Status>>
            + Send
            + 'static;
        /// another system could monitor newly added/confirmed/cancelled reservations
        async fn listen(
            &self,
            request: tonic::Request<super::ListenRequest>,
        ) -> Result<tonic::Response<Self::listenStream>, tonic::Status>;
    }
    /// Reservation service
    #[derive(Debug)]
    pub struct ReservationServiceServer<T: ReservationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ReservationService> ReservationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ReservationServiceServer<T>
    where
        T: ReservationService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/reservation.ReservationService/reserve" => {
                    #[allow(non_camel_case_types)]
                    struct reserveSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::ReserveRequest> for reserveSvc<T> {
                        type Response = super::ReserveResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReserveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reserve(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = reserveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/confirm" => {
                    #[allow(non_camel_case_types)]
                    struct confirmSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::ConfirmRequest> for confirmSvc<T> {
                        type Response = super::ConfirmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfirmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).confirm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = confirmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/update" => {
                    #[allow(non_camel_case_types)]
                    struct updateSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::UpdateRequest> for updateSvc<T> {
                        type Response = super::UpdateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/cancel" => {
                    #[allow(non_camel_case_types)]
                    struct cancelSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::CancelRequest> for cancelSvc<T> {
                        type Response = super::CancelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = cancelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::GetRequest> for getSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/query" => {
                    #[allow(non_camel_case_types)]
                    struct querySvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService>
                        tonic::server::ServerStreamingService<super::QueryRequest> for querySvc<T>
                    {
                        type Response = super::Reservation;
                        type ResponseStream = T::queryStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = querySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/filter" => {
                    #[allow(non_camel_case_types)]
                    struct filterSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::FilterRequest> for filterSvc<T> {
                        type Response = super::FilterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).filter(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = filterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/listen" => {
                    #[allow(non_camel_case_types)]
                    struct listenSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService>
                        tonic::server::ServerStreamingService<super::ListenRequest>
                        for listenSvc<T>
                    {
                        type Response = super::Reservation;
                        type ResponseStream = T::listenStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).listen(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ReservationService> Clone for ReservationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ReservationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ReservationService> tonic::server::NamedService for ReservationServiceServer<T> {
        const NAME: &'static str = "reservation.ReservationService";
    }
}
