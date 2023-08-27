use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder().build()?;

    // Perform the actual execution of the network request
    let res = client.get("https://httpbin.org/ip").send().await?;

    // Parse the response body as Json in this case
    let ip = res.json::<HashMap<String, String>>().await?;

    println!("{:?}", ip);
    Ok(())
}
