use std::sync::Arc;

use clap::{arg, command, Parser};
use tokio::{net::TcpStream, sync::Semaphore, task, time::{timeout, Duration}};

#[derive(Parser, Debug)]
#[command(name = "scanner")]
#[command(about = "A simple CLI port scanner written in Rust")]
struct Args {
    #[arg()]
    target: String,
    
    #[arg(default_value = "1-1000")]
    ports: String,

    #[arg(short, long, default_value = "100")]
    concurrency: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let target = Arc::new(args.target);
    let concurrency_limit =  Arc::new(Semaphore::new(args.concurrency));
    let range = parse_port_range(&args.ports);
    let mut tasks = vec![];

    println!("🔍 Scanning {} on ports {} with max {} concurrent scans...", target, args.ports, args.concurrency);

    for &port in &range {
        let target = Arc::clone(&target);
        let concurrency_limit = Arc::clone(&concurrency_limit);

        tasks.push(task::spawn(async move {
            let _permit = concurrency_limit.acquire().await.unwrap();
            if is_port_open(&target, port).await {
                println!("✅ Port {} is open!", port);
            } else {
                println!("❌ Port {} is closed!", port);
            }
        }));
    }

    for task in tasks {
        let _ = task.await?;
    }

    println!("Scan complete.");

    Ok(())
}

fn parse_port_range(port_str: &str) -> Vec<u16> {
    match port_str.split_once('-') {
        Some((start, end)) => (start.parse().unwrap_or(1)..=end.parse().unwrap_or(65535)).collect(),
        None => vec![port_str.parse().unwrap_or(1)],
    }
}

async fn is_port_open(target: &str, port: u16) -> bool {
    let address = format!("{}:{}", target, port);
    let result = timeout(Duration::from_secs(2), TcpStream::connect(&address)).await;
    result.is_ok() && result.unwrap().is_ok()
}