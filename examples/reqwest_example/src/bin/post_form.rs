use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = Person {
        first_name: "Foo".into(),
        last_name: "Bar".into(),
    };

    let res = reqwest::Client::new()
        .post("https://httpbin.org/anything")
        .form(&p)
        .send()
        .await?;

    let t = res.text().await?;

    println!("{}", t);
    Ok(())
}
