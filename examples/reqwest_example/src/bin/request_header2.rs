use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("X-HEADER-1", header::HeaderValue::from_static("val1"));
    headers.insert("X-HEADER-2", header::HeaderValue::from_static("val2"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client
        .get("https://httpbin.org/anything")
        .body("whatever")
        .header("X-HEADER-1", "overriden val1")
        .send()
        .await?;

    println!("{}", res.text().await?);
    Ok(())
}