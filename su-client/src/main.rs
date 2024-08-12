use std::net::IpAddr;

use su_client::run_su_client;
use clap::Parser;
use share::defaults::{IP, PORT};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to host server on
    #[arg(short, long, default_value_t = PORT)]
    port: u16,

    /// IP to host server on
    #[arg(short, long, default_value_t = IP)]
    ip: IpAddr,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();

    run_su_client(args.ip, args.port)?;

    Ok(())
}
