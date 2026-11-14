pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentRedactionEntitiesItem {
    Payment,
    Name,
    DateOfBirth,
    Age,
    PhoneNumber,
    Email,
    Address,
    Location,
    Date,
    Medical,
    RecordId,
    InsuranceId,
    Ssn,
    Username,
    Password,
    OtherPii,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentRedactionEntitiesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Payment => serializer.serialize_str("payment"),
            Self::Name => serializer.serialize_str("name"),
            Self::DateOfBirth => serializer.serialize_str("date_of_birth"),
            Self::Age => serializer.serialize_str("age"),
            Self::PhoneNumber => serializer.serialize_str("phone_number"),
            Self::Email => serializer.serialize_str("email"),
            Self::Address => serializer.serialize_str("address"),
            Self::Location => serializer.serialize_str("location"),
            Self::Date => serializer.serialize_str("date"),
            Self::Medical => serializer.serialize_str("medical"),
            Self::RecordId => serializer.serialize_str("record_id"),
            Self::InsuranceId => serializer.serialize_str("insurance_id"),
            Self::Ssn => serializer.serialize_str("ssn"),
            Self::Username => serializer.serialize_str("username"),
            Self::Password => serializer.serialize_str("password"),
            Self::OtherPii => serializer.serialize_str("other_pii"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentRedactionEntitiesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payment" => Ok(Self::Payment),
            "name" => Ok(Self::Name),
            "date_of_birth" => Ok(Self::DateOfBirth),
            "age" => Ok(Self::Age),
            "phone_number" => Ok(Self::PhoneNumber),
            "email" => Ok(Self::Email),
            "address" => Ok(Self::Address),
            "location" => Ok(Self::Location),
            "date" => Ok(Self::Date),
            "medical" => Ok(Self::Medical),
            "record_id" => Ok(Self::RecordId),
            "insurance_id" => Ok(Self::InsuranceId),
            "ssn" => Ok(Self::Ssn),
            "username" => Ok(Self::Username),
            "password" => Ok(Self::Password),
            "other_pii" => Ok(Self::OtherPii),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentRedactionEntitiesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Payment => write!(f, "payment"),
            Self::Name => write!(f, "name"),
            Self::DateOfBirth => write!(f, "date_of_birth"),
            Self::Age => write!(f, "age"),
            Self::PhoneNumber => write!(f, "phone_number"),
            Self::Email => write!(f, "email"),
            Self::Address => write!(f, "address"),
            Self::Location => write!(f, "location"),
            Self::Date => write!(f, "date"),
            Self::Medical => write!(f, "medical"),
            Self::RecordId => write!(f, "record_id"),
            Self::InsuranceId => write!(f, "insurance_id"),
            Self::Ssn => write!(f, "ssn"),
            Self::Username => write!(f, "username"),
            Self::Password => write!(f, "password"),
            Self::OtherPii => write!(f, "other_pii"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
