use anyhow::{Context, Result};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};

pub fn resolve_target(target: &str, port: u16) -> Result<Vec<SocketAddr>> {
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
            Err(e) => return Err(e).context(format!("Unable to parse input \"{}\" as an IP address or resolve with DNS. Check the input is valid and try again.", target)),
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
