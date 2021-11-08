mod sources;

const AXEL_HEIBERG_GLACIER: &str = "2-6635591";
const MOUNT_WISTING: &str = "2-6628219";
const AMUNDSEN_SCOTT: &str = "2-6299995";

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    // -------
    sources::weather_yr::weather(AXEL_HEIBERG_GLACIER).await?;
    sources::weather_yr::weather(MOUNT_WISTING).await?;
    sources::weather_yr::weather(AMUNDSEN_SCOTT).await?;
    // -------

    Ok(())
}
