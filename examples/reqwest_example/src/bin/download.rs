use std::fs::File;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let res = client
        .get("https://httpbin.org/image/png")
        .send()
        .await?
        .bytes()
        .await?;

    let mut data = res.as_ref();

    let mut f = File::create("i.png")?;

    io::copy(&mut data, &mut f)?;

    Ok(())
}
