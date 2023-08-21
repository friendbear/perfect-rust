use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // 画像ファイルのパスを指定
    let args: Vec<String> = std::env::args().collect();

    // ファイルを開いて読み込む
    let mut file = File::open(&args[1])?;

    // バイト配列として読み込む
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // バイト配列を出力
    println!("{:?}", buffer);

    Ok(())
}
