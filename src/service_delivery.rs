use crate::general_message::GeneralMessageDelivery;
use crate::shared::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ArrivalStatus {
    OnTime,
    Early,
    Delayed,
    Cancelled,
    Missed,
    Arrived,
    NotExpected,
    NoReport,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MonitoredCall {
    pub order: u16,
    pub stop_point_name: String,
    /// true if the vehicle is at the stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_at_stop: Option<bool>,
    /// Destination on the headsign of the vehicle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_display: Option<String>,
    /// Scheduled arrival time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aimed_arrival_time: Option<DateTime>,
    /// Scheduled departure time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aimed_departure_time: Option<DateTime>,
    /// Estimated arrival time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_arrival_time: Option<DateTime>,
    /// Estimated departure time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_departure_time: Option<DateTime>,
    /// Status on the arrival at the stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_status: Option<ArrivalStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceInfoGroup {
    /// Id of the operator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_ref: Option<String>,
    /* TODO find the right documentation for the type of this
    /// Specific features available in the vehicle
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vehicle_feature_ref: Vec<String>,
    */
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MonitoredVehicleJourney {
    /// Id of the line
    pub line_ref: String,
    #[serde(flatten)]
    pub service_info: ServiceInfoGroup,
    /// Id of the journey pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_pattern_ref: Option<String>,
    pub monitored_call: Option<MonitoredCall>,
    // pub onward_calls: Option<OnwardCall>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MonitoredStopVisit {
    /// Id of the stop point
    pub monitoring_ref: String,
    /// Datetime of the information update
    pub recorded_at_time: chrono::DateTime<chrono::Utc>,
    /// Id of the couple Stop / VehicleJourney
    pub item_identifier: String,
    pub monitored_vehicle_journey: MonitoredVehicleJourney,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopMonitoringDelivery {
    /// Version of the siri's response
    pub version: String,
    /// Datetime of the response's production
    pub response_timestamp: String,
    /// Id of the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_message_ref: Option<String>, // Note: this is mandatory for idf profil
    /// Status of the response, true if the response has been correctly treated, false otherwise
    pub status: bool,
    pub monitored_stop_visit: Vec<MonitoredStopVisit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EstimatedTimetableDelivery {
    pub response_timestamp: String,
    pub version: String,
    pub status: String,
    pub estimated_journey_version_frame: Vec<EstimatedJourneyVersionFrame>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EstimatedJourneyVersionFrame {
    pub estimated_vehicle_journey: Vec<EstimatedVehicleJourney>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EstimatedVehicleJourney {
    pub recorded_at_time: String,
    pub line_ref: LineRef,
    pub direction_ref: DirectionRef,
    pub dated_vehicle_journey_ref: DatedVehicleJourneyRef,
    pub route_ref: RouteRef,
    pub published_line_name: Vec<PublishedLineName>,
    pub origin_ref: OriginRef,
    pub destination_ref: DestinationRef,
    #[serde(default)]
    pub destination_name: Vec<DestinationName>,
    pub operator_ref: OperatorRef,
    pub product_category_ref: ProductCategoryRef,
    #[serde(default)]
    pub journey_note: Vec<JourneyNote>,
    pub first_or_last_journey: String,
    pub estimated_calls: EstimatedCalls,
    #[serde(default)]
    pub vehicle_mode: Vec<String>,
    #[serde(default)]
    pub direction_name: Vec<Name>,
    #[serde(default)]
    pub origin_name: Vec<Name>,
    #[serde(default)]
    pub vehicle_journey_name: Vec<VehicleJourneyName>,
    #[serde(default)]
    pub via: Vec<Vum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineRef {
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionRef {
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatedVehicleJourneyRef {
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteRef {
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishedLineName {
    pub value: String,
    pub lang: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginRef {
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinationRef {
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinationName {
    pub value: String,
    pub lang: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperatorRef {
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductCategoryRef {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JourneyNote {
    pub value: String,
    pub lang: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EstimatedCalls {
    pub estimated_call: Vec<EstimatedCall>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EstimatedCall {
    pub stop_point_ref: StopPointRef,
    pub expected_arrival_time: Option<DateTime>,
    pub expected_departure_time: Option<DateTime>,
    pub aimed_arrival_time: Option<DateTime>,
    pub aimed_departure_time: Option<DateTime>,
    #[serde(default)]
    pub destination_display: Vec<DestinationDisplay>,
    pub arrival_status: Option<String>,
    pub arrival_proximity_text: ArrivalProximityText,
    pub arrival_platform_name: ArrivalPlatformName,
    pub departure_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopPointRef {
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinationDisplay {
    pub value: String,
    pub lang: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrivalProximityText {
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrivalPlatformName {
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub value: String,
    pub lang: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleJourneyName {
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Vum {
    pub place_name: Vec<PlaceName>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaceName {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDelivery {
    #[serde(flatten)]
    pub common: crate::shared::CommonDelivery,
    /// Id of the producer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_ref: Option<String>,
    /// Address of the service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Id of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message_identifier: Option<String>, // Note: this is mandatory for idf profil
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub stop_monitoring_delivery: Vec<StopMonitoringDelivery>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub general_message_delivery: Vec<GeneralMessageDelivery>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub estimated_timetable_delivery: Vec<EstimatedTimetableDelivery>,
}
