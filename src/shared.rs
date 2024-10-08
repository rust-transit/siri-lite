use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DateTime(pub chrono::DateTime<chrono::FixedOffset>);

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> ::serde::Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(DateTime(chrono::DateTime::parse_from_rfc3339(&s).map_err(
            |e| serde::de::Error::custom(format!("datetime format not valid: {}", e)),
        )?))
    }
}

/// Common fields used by all the siri's Delivery
///
/// Note: it is referenced as `xxxDelivery` in the siri specifications
#[derive(Serialize, Deserialize, Debug)]
pub struct CommonDelivery {
    pub version: Option<String>,
    #[serde(rename = "ResponseTimestamp")]
    pub response_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Id of the query
    pub request_message_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

impl Default for CommonDelivery {
    fn default() -> Self {
        CommonDelivery {
            version: Some("2.0".to_string()),
            response_timestamp: chrono::Utc::now().to_rfc3339(),
            // error_condition: None,
            status: Some(true),
            request_message_ref: None,
        }
    }
}
