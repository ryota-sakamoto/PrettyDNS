use pretty_dns_server::server;
use std::{io, net::Ipv4Addr};
use structopt::StructOpt;
use tracing;
use tracing_subscriber;

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(long, default_value = "0.0.0.0")]
    addr: Ipv4Addr,

    #[structopt(short, long, default_value = "53")]
    port: u16,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let c = Config::from_args();
    if c.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt().init();
    }

    server::start(server::Config {
        addr: c.addr,
        port: c.port,
    })
    .await
}
