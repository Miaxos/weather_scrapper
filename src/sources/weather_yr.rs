use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Symbol {
    pub sunup: bool,
    pub n: i32,
    pub clouds: i32,
    pub precip: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SymbolCode {
    pub next1_hour: Option<String>,   // cloudy
    pub next6_hours: Option<String>,  // cloudy
    pub next12_hours: Option<String>, // cloudy
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Precipitation {
    pub value: Option<f32>, // mm
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    pub value: Option<f32>, // celsius
    pub min: Option<f32>,   // celsius
    pub max: Option<f32>,   // celsius
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub direction: Option<f32>, // deg
    pub speed: Option<f32>,     // m/s
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeelsLike {
    pub value: Option<f32>, // celsius
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pressure {
    pub value: Option<f32>, // unit?
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloudCover {
    pub value: Option<f32>,  // percentage
    pub high: Option<f32>,   // percentage
    pub middle: Option<f32>, // percentage
    pub low: Option<f32>,    // percentage
    pub fog: Option<f32>,    // percentage
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Humidity {
    pub value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DewPoint {
    pub value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShortPositions {
    pub symbol: Symbol,
    pub symbol_code: SymbolCode,
    pub precipitation: Precipitation,
    pub temperature: Temperature,
    pub wind: Wind,
    pub feels_like: FeelsLike,
    pub pressure: Pressure,
    pub cloud_cover: CloudCover,
    pub humidity: Humidity,
    pub dew_point: DewPoint,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
    pub nominal_start: Option<chrono::DateTime<chrono::Utc>>,
    pub nominal_end: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAPI {
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub update: Option<chrono::DateTime<chrono::Utc>>,
    pub short_intervals: Option<Vec<ShortPositions>>,
    pub long_intervals: Option<Vec<ShortPositions>>,
}

pub async fn weather(id: &str) -> anyhow::Result<ResponseAPI> {
    let url = format!("https://www.yr.no/api/v0/locations/{}/forecast", id);
    let resp = reqwest::get(&url).await?.json::<ResponseAPI>().await?;

    Ok(resp)
}
