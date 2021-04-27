use std::io;

mod server;

#[tokio::main]
async fn main() -> io::Result<()> {
    server::start(server::Config::default()).await
}
