use std::error::Error;
use std::env;
use std::process::Command;
use std::path::Path;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
// Remove the unused import

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

fn search_videos(api_key: &str, query: &str) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let url = format!("https://www.googleapis.com/youtube/v3/search?key={}&part=snippet&q={}&maxResults=1&type=video", api_key, query);
    let client = Client::new();
    let response = client.get(&url).send()?.json::<SearchResponse>()?;
    Ok(response.items)
}

fn download_video(video_id: &str, output_dir: &str) -> Result<String, Box<dyn Error>> {
    let video_url = format!("https://www.youtube.com/watch?v={}", video_id);
    let output_path = Path::new(output_dir).join(format!("{}.mp4", video_id));

    let status = Command::new("youtube-dl")
        .arg("-o")
        .arg(output_path.to_string_lossy().to_string())
        .arg(&video_url)
        .status()?;

    if !status.success() {
        return Err("Failed to download video".into());
    }

    Ok(output_path.to_string_lossy().to_string())
}

fn convert_to_mp3(input_path: &str, output_dir: &str, title: &str) -> Result<(), Box<dyn Error>> {
    let output_path = Path::new(output_dir).join(format!("{}.mp3", title));

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg(output_path.to_string_lossy().to_string())
        .status()?;

    if !status.success() {
        return Err("Failed to convert to mp3".into());
    }

    Ok(())
}

fn main() {
    dotenv::dotenv().ok();
    
    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set");
    let query = env::var("YOUTUBE_QUERY").unwrap_or("Hatsune Miku".to_string());
    let download_dir = env::var("DOWNLOAD_DIR").expect("DOWNLOAD_DIR must be set");

    print!("Searching for videos... {query}");
    match search_videos(&api_key, &query) {
        Ok(results) => {
            if let Some(video) = results.first() {
                let video_id = &video.id.videoId;
                let title = &video.snippet.title;

                println!("Found video: {} ({})", title, video_id);
                
                match download_video(video_id, &download_dir) {

                    Ok(video_path) => {
                        if let Err(err) = convert_to_mp3(&video_path, &download_dir, title) {
                            eprintln!("Failed to convert video to mp3: {}", err);
                        } else {
                            println!("Video converted to mp3 successfully!");
                        }
                    },
                    Err(err) => eprintln!("Failed to download video: {}", err),
                }
            } else {
                eprintln!("No videos found for the query.");
            }
        },
        Err(err) => eprintln!("Search failed: {}", err),
    }
}
