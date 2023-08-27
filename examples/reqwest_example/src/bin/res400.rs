#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder().build()?;

    let res = client.get("https://httpbin.org/status/400").send().await?;

    match res.status() {
        reqwest::StatusCode::BAD_REQUEST => println!(
            "content-length:{:?} server:{:?}",
            res.headers().get(reqwest::header::CONTENT_LENGTH),
            res.headers().get(reqwest::header::SERVER),
        ),
        status => println!("status: {}", status),
    }

    // content-length:Some("0") server:Some("gunicorn/19.9.0")
    Ok(())
}
