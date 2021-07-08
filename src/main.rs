use parker::settings::Settings;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let settings = Settings::new()?;

    let app = parker::app();
    app.listen(format!("127.0.0.1:{}", settings.port)).await?;
    Ok(())
}
