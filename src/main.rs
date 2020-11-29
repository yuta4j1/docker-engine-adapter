#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let base_url = "http://localhost/v1.40";
    let body = reqwest::get("http:localhost/v1.40/containers/json?all=1").await?.text().await?;
    println!("body: {}", body);

    Ok(())
}
