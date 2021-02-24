use openapi_schema::OpenapiSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, OpenapiSchema)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    pub line_ref: String,
}

#[derive(Debug, Serialize, Deserialize, OpenapiSchema)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Serialize, Deserialize, OpenapiSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AnnotatedStopPoint {
    pub stop_point_ref: String,
    pub stop_name: String,
    pub lines: Vec<Line>,
    pub location: Location,
}

#[derive(Debug, Serialize, Deserialize, OpenapiSchema)]
#[serde(rename_all = "PascalCase")]
pub struct StopPointsDelivery {
    #[serde(flatten)]
    pub common: crate::shared::CommonDelivery,
    pub annotated_stop_point: Vec<AnnotatedStopPoint>,
}
