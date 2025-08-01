use anyhow::{Context, Result};
use clap::Parser;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};

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

fn resolve_target(target: &str, port: u16) -> Result<Vec<SocketAddr>> {
    let targets: Vec<SocketAddr> = match parse_as_ip(target, port) {
        Ok(addrs) => {
            println!("Parsed IP address input:\n * {}", addrs[0].ip());
            addrs
        }
        Err(_) => match resolve_via_dns(target, port) {
            Ok(addrs) => {
                let as_string: String = addrs
                    .iter()
                    .map(|a| format!(" * {}", a.ip().to_string()))
                    .collect::<Vec<_>>()
                    .join("\n");
                println!("Resolved DNS input:\n{}", as_string);
                addrs
            }
            Err(e) => return Err(e).context(format!("DNS resolution failed for \"{}\"", target)),
        },
    };

    Ok(targets)
}

// While the `parse_as_ip` function will only return a single SocketAddr, it is returned in a
// collection to ensure a uniform interface regardless of whether an IP or domain name is provided
// by the user
fn parse_as_ip(ip_addr: &str, port: u16) -> Result<Vec<SocketAddr>, std::net::AddrParseError> {
    Ok(vec![SocketAddr::new(ip_addr.parse::<IpAddr>()?, port)])
}

fn resolve_via_dns(domain: &str, port: u16) -> Result<Vec<SocketAddr>, std::io::Error> {
    let full: String = format!("{}:{}", domain, port);

    match full.to_socket_addrs() {
        Ok(addrs) => Ok(addrs.collect()),
        Err(e) => Err(e),
    }
}
