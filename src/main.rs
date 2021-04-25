use std::{io, sync::Arc};
use tokio::{net::UdpSocket, sync::mpsc};

mod client;
mod message;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:53").await?;
    let sock = Arc::new(sock);
    let (tx, rx) = mpsc::channel::<(Vec<u8>, std::net::SocketAddr)>(1000);
    tokio::spawn(handler(sock.clone(), rx));

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}

async fn handler(
    sock: Arc<UdpSocket>,
    rx: mpsc::Receiver<(Vec<u8>, std::net::SocketAddr)>,
) -> io::Result<()> {
    let mut rx = rx;
    while let Some((buf, addr)) = rx.recv().await {
        tokio::spawn(_handler(sock.clone(), buf, addr));
    }

    return Ok(());
}

async fn _handler(
    sock: Arc<UdpSocket>,
    buf: Vec<u8>,
    addr: std::net::SocketAddr,
) -> io::Result<()> {
    println!("---");
    println!("data: {:?}", buf);

    let req = message::from_bytes(&buf).await?;
    println!("req: {:?}", req);

    let result = client::dig(req).await?;
    println!("result: {:?}", result);

    sock.send_to(&result, addr).await?;

    Ok(())
}
