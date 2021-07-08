#[async_std::main]

async fn main() -> tide::Result<()> {
    tide::log::start();

    let app = parker::app();
    app.listen("127.0.0.1:4000").await?;
    Ok(())
}
