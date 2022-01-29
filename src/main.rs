use chrono::Timelike;
use dotenv::dotenv;
use hyper::client::HttpConnector;
use yup_oauth2::authenticator::Authenticator;

use crate::sinks::google_sheet::GoogleRow;

mod sinks;
mod sources;

// Should be inside a config file, but we don't care rn.
const AXEL_HEIBERG_GLACIER: &str = "2-6635591";
const MOUNT_WISTING: &str = "2-6628219";
const AMUNDSEN_SCOTT: &str = "2-6299995";

async fn process_glacier(
    auth: &Authenticator<hyper_rustls::HttpsConnector<HttpConnector>>,
    glacier: &str,
    sheet_id: &str,
    page: &str,
    targeted: &Vec<chrono::DateTime<chrono_tz::Tz>>,
) -> anyhow::Result<()> {
    let glacier = sources::weather_yr::weather(glacier).await?;

    let morning_positions = glacier
        .short_intervals
        .unwrap_or(vec![])
        .iter()
        .filter_map(|x| {
            if targeted.contains(&x.start.with_timezone(&chrono_tz::Tz::NZ)) {
                Some(GoogleRow {
                    date: x.start,
                    weather: x.symbol_code.next1_hour.clone(),
                    temperature: x.temperature.value,
                    feels_like: x.feels_like.value,
                    precipitation: x.precipitation.value,
                    wind_speed: x.wind.speed,
                    wind_orientation: x.wind.direction,
                    pressure: x.pressure.value,
                    humidity: x.humidity.value,
                    dew_point: x.dew_point.value,
                    cloud_cover: x.cloud_cover.value,
                    fog: x.cloud_cover.fog,
                    low: x.cloud_cover.low,
                    middle: x.cloud_cover.middle,
                    high: x.cloud_cover.high,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut index = sinks::google_sheet::get_last_row_to_write(&auth, sheet_id, page).await?;

    for morning in morning_positions {
        index = index + 1;
        sinks::google_sheet::write_row(&auth, morning, sheet_id, page, index).await?;
    }
    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let profile = dotenv::var("PROFILE").expect("file PROFILE key");
    let sheet_id = dotenv::var("SHEET_ID").expect("file SHEET_ID key");

    let auth = sinks::google_sheet::auth("auth.json").await?;

    let tomorrow = (chrono::Utc::now().with_timezone(&chrono_tz::Tz::NZ)
        + chrono::Duration::days(1))
    .with_minute(0)
    .unwrap()
    .with_second(0)
    .unwrap()
    .with_nanosecond(0)
    .unwrap();

    let targeted_hour_00 = tomorrow.with_hour(0).unwrap();
    let targeted_hour_06 = tomorrow.with_hour(6).unwrap();
    let targeted_hour_12 = tomorrow.with_hour(12).unwrap();
    let targeted_hour_18 = tomorrow.with_hour(18).unwrap();

    let targeted = match profile.as_ref() {
        "night" => vec![targeted_hour_12, targeted_hour_18],
        "morning" => vec![targeted_hour_00, targeted_hour_06],
        _ => panic!("Set morning / night"),
    };

    process_glacier(
        &auth,
        AXEL_HEIBERG_GLACIER,
        &sheet_id,
        "AXEL HEIBERG",
        &targeted,
    )
    .await?;
    process_glacier(&auth, MOUNT_WISTING, &sheet_id, "MOUNT WISTING", &targeted).await?;
    process_glacier(
        &auth,
        AMUNDSEN_SCOTT,
        &sheet_id,
        "AMUNDSEN SCOTT",
        &targeted,
    )
    .await?;

    Ok(())
}
