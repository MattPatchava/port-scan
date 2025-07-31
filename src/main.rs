use clap::Parser;

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
    let args: Args = Args::parse();

    // TODO: Parse target into IP address struct and use this for later access (rather than the
    // args struct)

    let start_port: u16 = args.start_port;
    let end_port: u16 = match args.end_port {
        Some(i) => i,
        None => start_port,
    };

    println!(
        "IP to scan: {}, port(s) to scan: {} to {} (inclusive)",
        args.target, start_port, end_port
    );
}
