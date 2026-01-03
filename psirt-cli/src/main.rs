use clap::Parser;

// TODO: define `clap` CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {}

#[tokio::main]
async fn main() {
    println!("Hello world!");
}