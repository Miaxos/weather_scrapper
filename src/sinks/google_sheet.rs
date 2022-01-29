use chrono_tz::Tz::NZ;
use google_sheets4::api::ValueRange;
use google_sheets4::Error;
use google_sheets4::Sheets;
use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use tokio::io::AsyncWriteExt as _;
use yup_oauth2::authenticator::Authenticator;

/// Authentificate to Google doc.
pub async fn auth(
    config: &str,
) -> anyhow::Result<Authenticator<hyper_rustls::HttpsConnector<HttpConnector>>> {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    let secret = yup_oauth2::read_service_account_key(config).await?;

    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace`None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = yup_oauth2::ServiceAccountAuthenticator::builder(secret)
        .build()
        .await?;

    Ok(auth)
}

#[derive(Debug)]
pub struct GoogleRow {
    pub date: chrono::DateTime<chrono::Utc>,
    pub weather: Option<String>,
    pub temperature: Option<f32>,
    pub feels_like: Option<f32>,
    pub precipitation: Option<f32>,
    pub wind_speed: Option<f32>,
    pub wind_orientation: Option<f32>,
    pub pressure: Option<f32>,
    pub humidity: Option<f32>,
    pub dew_point: Option<f32>,
    pub cloud_cover: Option<f32>,
    pub fog: Option<f32>,
    pub low: Option<f32>,
    pub middle: Option<f32>,
    pub high: Option<f32>,
}

/// Function to get the last row, we want to write to the next one.
pub async fn get_last_row_to_write(
    auth: &Authenticator<hyper_rustls::HttpsConnector<HttpConnector>>,
    sheed_id: &str,
    page: &str,
) -> anyhow::Result<usize> {
    let hub = Sheets::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
        auth.clone(),
    );

    let (_, result) = hub
        .spreadsheets()
        .values_get(sheed_id, &format!("{}!A1:A1000000", page))
        .doit()
        .await?;

    let next_index_to_write = result.values.map(|x| x.len()).unwrap_or(0);
    Ok(next_index_to_write)
}

/// Write a ROW inside a google doc Sheets
pub async fn write_row(
    auth: &Authenticator<hyper_rustls::HttpsConnector<HttpConnector>>,
    row: GoogleRow,
    sheed_id: &str,
    page: &str,
    index: usize,
) -> anyhow::Result<()> {
    let hub = Sheets::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
        auth.clone(),
    );

    let GoogleRow {
        date,
        weather,
        temperature,
        feels_like,
        precipitation,
        wind_speed,
        wind_orientation,
        pressure,
        humidity,
        dew_point,
        cloud_cover,
        fog,
        low,
        middle,
        high,
    } = row;

    let nz = date.with_timezone(&NZ);

    let req = ValueRange {
        values: Some(vec![vec![
            format!("{}", date),
            format!("{}", nz),
            nz.format("%d/%m/%y").to_string(),
            nz.format("%R").to_string(),
            weather.unwrap_or(String::new()),
            temperature.map(|x| x.to_string()).unwrap_or(String::new()),
            feels_like.map(|x| x.to_string()).unwrap_or(String::new()),
            precipitation
                .map(|x| x.to_string())
                .unwrap_or(String::new()),
            wind_speed.map(|x| x.to_string()).unwrap_or(String::new()),
            wind_orientation
                .map(|x| x.to_string())
                .unwrap_or(String::new()),
            pressure.map(|x| x.to_string()).unwrap_or(String::new()),
            humidity.map(|x| x.to_string()).unwrap_or(String::new()),
            dew_point.map(|x| x.to_string()).unwrap_or(String::new()),
            cloud_cover.map(|x| x.to_string()).unwrap_or(String::new()),
            fog.map(|x| x.to_string()).unwrap_or(String::new()),
            low.map(|x| x.to_string()).unwrap_or(String::new()),
            middle.map(|x| x.to_string()).unwrap_or(String::new()),
            high.map(|x| x.to_string()).unwrap_or(String::new()),
        ]]),
        range: Some(format!(
            "{page}!A{index}:R{index}",
            page = page,
            index = index
        )),
        major_dimension: Some("ROWS".to_string()),
    };

    let result = hub
        .spreadsheets()
        .values_update(
            req,
            sheed_id,
            &format!("{page}!A{index}:R{index}", page = page, index = index),
        )
        .value_input_option("RAW")
        .doit()
        .await;

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
            Error::Failure(mut e) => {
                println!("{:?}", e);
                while let Some(chunk) = e.body_mut().data().await {
                    tokio::io::stdout().write_all(&chunk?).await?;
                }
            }
        },
        Ok(_) => {}
    }

    Ok(())
}
