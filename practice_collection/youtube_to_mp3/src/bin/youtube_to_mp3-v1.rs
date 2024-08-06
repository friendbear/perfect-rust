use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

// API Response のjson を構造体で定義
#[derive(Serialize, Deserialize, Debug)]
struct SearchResponse {
    items: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult {
    id: VideoId,
    snippet: Snippet,
}

#[derive(Serialize, Deserialize, Debug)]
struct VideoId {
    videoId: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Snippet {
    title: String,
}

async fn search_videos(api_key: &str, query: &str) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let url = format!("https://www.googleapis.com/youtube/v3/search?key={}&part=snippet&q={}&maxResults=1&type=video", api_key, query);
    let client = Client::builder().build()?;
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<SearchResponse>()
        .await?;
    Ok(response.items)
}

async fn download_video(
    api_key: &str,
    video_id: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?key={}&id={}&part=snippet",
        api_key, video_id
    );
    let client = Client::builder().build()?;
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<SearchResponse>()
        .await?;
    let title = &response.items[0].snippet.title;
    println!("Downloading video: {}", title);
    let video_url = format!("https://www.youtube.com/watch?v={}", video_id);
    let audio_url = format!("https://www.convertmp3.io/fetch/?video={}", video_url);
    let audio_response = client.get(&audio_url).send().await?.bytes().await?;
    let mut byte = audio_response.as_ref();
    let mut output_file = File::create(Path::new(output_dir).join(format!("{}.mp3", title)))?;
    std::io::copy(&mut byte, &mut output_file)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set");
    let query = "英会話リスニング"; // 検索クエリを設定する

    match search_videos(&api_key, query).await {
        Ok(results) => {
            if let Some(video) = results.first() {
                dbg!(video);
                if let Err(err) = download_video(&api_key, &video.id.videoId, "~/Music/Miku").await
                {
                    eprintln!("Failed to download video: {}", err);
                } else {
                    println!("Video downloaded successfully!");
                }
            } else {
                eprintln!("No videos found for the query.");
            }
        }
        Err(err) => eprintln!("Search failed: {}", err),
    }
}
