use anyhow::Result;
use clap::Parser;
use std::net::SocketAddr;

mod target_resolver;
use crate::target_resolver::resolve_target;

#[derive(Parser)]
#[command(name = "port-scan", version, about, long_about = None)]
struct Args {
    /// Target host to scan
    target: String,

    /// Start port
    start_port: u16,

    /// Optional end port
    end_port: Option<u16>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Args = Args::parse();

    let start_port: u16 = args.start_port;
    let end_port: u16 = match args.end_port {
        Some(i) => i,
        None => start_port,
    };

    let targets: Vec<SocketAddr> = resolve_target(&args.target, start_port)?;

    Ok(())
}
