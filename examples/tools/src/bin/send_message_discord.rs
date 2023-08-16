use reqwest::blocking::Client;
use reqwest::blocking::multipart;
use reqwest::Error;


/// Sample code
const CHANNEL_ID: &str = "950663298456641557";
const PATH: &str = "/Users/k22/Pictures/upload.jpg";
fn main() -> Result<(), Error> {
    let bot_token: String = std::env::var("BOT_TOKEN").expect("Please Set Your Discord BOT_TOKEN");

    let client = Client::new();
    let url = format!("https://discord.com/api/v10/channels/{}/messages", CHANNEL_ID);

    let form = multipart::Form::new()
        .text("content", "This is a message with an attachment.")
        .file("file", PATH).expect("File not found."); // ファイルのパスを指定

    let response = client
        .post(&url)
        .header("Authorization", format!("Bot {}", bot_token))
        .multipart(form)
        .send()?;

    println!("{:?}", response.text()?);

    Ok(())
}