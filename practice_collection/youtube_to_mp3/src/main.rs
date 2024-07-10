use std::env;
use std::fs;
use std::process::Command;
use dotenv::dotenv;
use google_youtube3::YouTube;
use youtube_dl::YoutubeDl;
use tokio::runtime::Runtime;

const DOWNLOAD_DIR_DEFAULT: &str = "/path/to/your/default/download/directory";

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // 環境変数からAPIキーを取得
    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set");

    // 引数からダウンロードディレクトリと検索クエリを取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <download_dir> <search_query>", args[0]);
        return;
    }
    let download_dir = &args[1];
    let search_query = &args[2];

    // 動画を検索
    let video = search_youtube(&api_key, &search_query).await;
    if let Some((video_id, video_title)) = video {
        println!("Found video: {} (ID: {})", video_title, video_id);

        // 動画をダウンロードしてMP3に変換
        if download_and_convert(&video_id, &video_title, download_dir).await {
            println!("Downloaded and converted to {}", video_title);
        } else {
            println!("Failed to download and convert the video.");
        }
    } else {
        println!("No video found.");
    }
}

async fn search_youtube(api_key: &str, query: &str) -> Option<(String, String)> {
    let youtube = YouTube::new(google_youtube3::hyper::Client::new(), api_key);
    let result = youtube.search().list(&["snippet"]).q(query).max_results(1).doit().await;

    match result {
        Ok((_, response)) => {
            if let Some(items) = response.items {
                if let Some(item) = items.first() {
                    let video_id = item.id.as_ref().unwrap().video_id.as_ref().unwrap().clone();
                    let video_title = item.snippet.as_ref().unwrap().title.clone();
                    return Some((video_id, video_title));
                }
            }
        }
        Err(err) => {
            eprintln!("YouTube search error: {}", err);
        }
    }

    None
}

async fn download_and_convert(video_id: &str, video_title: &str, download_dir: &str) -> bool {
    let sanitized_title = sanitize_filename(video_title);
    let output_path = format!("{}/{}.mp3", download_dir, sanitized_title);

    // 既にファイルが存在するか確認
    if fs::metadata(&output_path).is_ok() {
        println!("{} already exists. Skipping download.", output_path);
        return false;
    }

    // 動画をダウンロード
    let url = format!("https://www.youtube.com/watch?v={}", video_id);
    let ydl_output = YoutubeDl::new(&url).download(true).run();

    match ydl_output {
        Ok(output) => {
            let temp_file = output.into_temp_file().unwrap();
            let temp_path = temp_file.path().to_str().unwrap();

            // ffmpegを使用してMP3に変換
            let status = Command::new("ffmpeg")
                .args(&["-i", temp_path, &output_path])
                .status();

            match status {
                Ok(status) if status.success() => true,
                _ => {
                    eprintln!("Failed to convert video to MP3.");
                    false
                }
            }
        }
        Err(err) => {
            eprintln!("youtube-dl error: {}", err);
            false
        }
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect()
}
