#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let res = client
        .post("https://httpbin.org/anything")
        .body("arbitrary text")
        .header("X-Person-First", "Foo!")
        .header("X-Person-Last", "Bar!!")
        .send()
        .await?;

    let t = res.text().await?;

    println!("{}", t);
    Ok(())
}
