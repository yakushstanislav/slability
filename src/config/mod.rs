use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "slability",
    bin_name = "slability",
    version,
    about = "Address availability monitoring tool."
)]
pub struct Config {
    #[arg(short = 'a', long, required = true, value_parser = parse_socket_address, num_args = 1.., help = "List of addresses")]
    pub addresses: Vec<(String, SocketAddr)>,

    #[arg(short = 't', long, required = false, default_value = "1000", value_parser = parse_duration_ms, help = "Connection timeout (in ms)")]
    pub timeout: Duration,

    #[arg(short = 'i', long, required = false, default_value = "5000", value_parser = parse_duration_ms, help = "Update interval (in ms)")]
    pub interval: Duration,
}

fn parse_socket_address(
    value: &str,
) -> Result<(String, SocketAddr), Box<dyn Error + Send + Sync + 'static>> {
    let addresses: Vec<SocketAddr> = value.to_socket_addrs()?.collect();

    match addresses.first() {
        Some(address) => Ok((value.to_string(), address.clone())),
        None => Err("socket address not found".into()),
    }
}

fn parse_duration_ms(arg: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let ms = arg.parse()?;

    Ok(Duration::from_millis(ms))
}
