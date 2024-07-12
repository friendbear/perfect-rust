//use dotenvy::dotenv;
//use dotenvy_macro;
use clap::Parser;

#[derive(Debug, Parser)]
struct Config {
    #[arg(long, env, hide_env_values = true)]
    host: String,
    #[arg(long, env, hide_env_values = true)]
    port: u16,
}
fn main() {
    let config = Config::parse();

    dbg!(&config);
}
