use pretty_dns::server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    server::start(server::Config::default()).await
}
