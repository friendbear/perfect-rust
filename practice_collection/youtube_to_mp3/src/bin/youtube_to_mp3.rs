use std::error::Error;
use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
//use serde_json::*;

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

const MAX_RESULTS: u32 = 350;

fn search_videos(api_key: &str, query: &str) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let url = format!("https://www.googleapis.com/youtube/v3/search?key={}&part=snippet&q={}&maxResults={}&type=video", api_key, query, MAX_RESULTS);
    let client = Client::new();
    let response = client.get(url).send()?.json::<SearchResponse>()?;
    //println!("{:?}", seade_json::to_json::<SearchResponse>(response));
    Ok(response.items)
}
fn sanitize_filename(filename: &str) -> String {
    filename.replace('/', "_")
}

fn download_video(video_id: &str, output_dir: &str) -> Result<PathBuf, Box<dyn Error>> {
    let video_url = format!("https://www.youtube.com/watch?v={}", video_id);
    let output_path = Path::new(output_dir).join(format!("{}.webm", video_id));

    let status = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo[ext=webm]+bestaudio[ext=webm]/best[ext=webm]")
        .arg("-o")
        .arg(output_path.to_string_lossy().to_string())
        .arg(&video_url)
        .status()?;

    if !status.success() {
        return Err("Failed to download video".into());
    }

    Ok(output_path)
}

fn convert_to_mp3(input_path: &Path, output_dir: &str, title: &str) -> Result<(), Box<dyn Error>> {
    let sanitized_title = sanitize_filename(title);
    let output_path = Path::new(output_dir).join(format!("{}.mp3", sanitized_title));

    // 出力ディレクトリが存在しない場合は作成
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path.to_string_lossy().to_string())
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

    for video in search_videos(&api_key, &query).unwrap() {
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
    }
}
