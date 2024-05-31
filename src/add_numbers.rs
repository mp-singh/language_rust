#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddNumbersRequest {
    #[prost(int32, tag = "1")]
    pub a: i32,
    #[prost(int32, tag = "2")]
    pub b: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddNumbersResponse {
    #[prost(int32, tag = "1")]
    pub message: i32,
}
