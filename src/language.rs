#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Translation {
    #[prost(string, tag = "1")]
    pub application: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub page: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub lang: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub context: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub translations: ::prost::alloc::vec::Vec<Translation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTranslationByIdRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
