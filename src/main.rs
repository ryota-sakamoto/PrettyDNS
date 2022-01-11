use pretty_dns_server::server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    server::start(server::Config::default()).await
}
