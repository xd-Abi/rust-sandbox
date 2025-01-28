use std::sync::{Arc, Mutex};
use clap::{arg, command, Parser};
use indicatif::{ProgressBar, ProgressStyle};
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
    
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let mut tasks = vec![];
    
    let progress = ProgressBar::new(range.len() as u64);
    progress.set_style(ProgressStyle::default_bar()
        .template("⏳ Scanning {wide_bar} {pos}/{len} ports...")
        .unwrap());

    for &port in &range {
        let target = Arc::clone(&target);
        let concurrency_limit = Arc::clone(&concurrency_limit);
        let progress = progress.clone();
        let open_ports = Arc::clone(&open_ports);

        tasks.push(task::spawn(async move {
            let _permit = concurrency_limit.acquire().await.unwrap();
            if is_port_open(&target, port).await {
                let mut ports = open_ports.lock().unwrap();
                ports.push(port);
            }

            progress.inc(1);
        }));
    }

    for task in tasks {
        let _ = task.await?;
    }

    progress.finish_and_clear();
    let open_ports = open_ports.lock().unwrap();
    if open_ports.is_empty() {
        println!("✅ Scan complete. No open ports found.");
    } else {
        println!("✅ Scan complete. Open ports found: {:?}", open_ports);
    }

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