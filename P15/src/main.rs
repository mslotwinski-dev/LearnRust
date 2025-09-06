use reqwest;

async fn download_webpage(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "http://example.com";

    let page = download_webpage(url).await?;
    Ok(())
}
