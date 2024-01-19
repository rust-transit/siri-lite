use crate::service_delivery::ServiceDelivery;
use crate::stop_points_delivery::StopPointsDelivery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Siri {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_points_delivery: Option<StopPointsDelivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_delivery: Option<ServiceDelivery>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SiriResponse {
    pub siri: Siri,
}
