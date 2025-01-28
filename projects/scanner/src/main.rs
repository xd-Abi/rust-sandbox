use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(name = "scanner")]
#[command(about = "A simple CLI port scanner written in Rust")]
struct Args {
    #[arg()]
    target: String,
    
    #[arg(default_value = "1-1000")]
    ports: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Ok(())
}
