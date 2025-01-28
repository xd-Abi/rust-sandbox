use clap::{arg, command, Parser};
use tokio::{net::TcpStream, task, time::{timeout, Duration}};

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

    let range: Vec<u16> = match args.ports.split_once('-') {
        Some((start, end)) => (start.parse().unwrap_or(1)..=end.parse().unwrap_or(65535)).collect(),
        None => vec![args.ports.parse().unwrap_or(1)],
    };

    println!("🔍 Scanning {} on ports {}...", args.target, args.ports);

    for &port in &range {
        let target = args.target.clone();

        task::spawn(async move {
            if is_port_open(&target, port).await {
                println!("✅ Port {} is open!", port);
            }
            else {
                println!("❌ Port {} is closed!", port);
            }
        }).await?;
    }

    println!("Scan complete.");
    Ok(())
}

async fn is_port_open(target: &str, port: u16) -> bool {
    let address = format!("{}:{}", target, port);
    let result = timeout(Duration::from_secs(2), TcpStream::connect(&address)).await;
    result.is_ok() && result.unwrap().is_ok()
}