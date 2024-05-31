#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckResponse {
    #[prost(enumeration = "ServingStatus", tag = "1")]
    pub status: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServingStatus {
    Good = 0,
    Bad = 1,
    Degraded = 2,
}
impl ServingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServingStatus::Good => "GOOD",
            ServingStatus::Bad => "BAD",
            ServingStatus::Degraded => "DEGRADED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GOOD" => Some(Self::Good),
            "BAD" => Some(Self::Bad),
            "DEGRADED" => Some(Self::Degraded),
            _ => None,
        }
    }
}
