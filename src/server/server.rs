use std::{io, sync::Arc};
use tokio::{net::UdpSocket, sync::mpsc};

use pretty_dns::{client, message};

#[derive(Default)]
pub struct Config {
    addr: &'static str,
    port: u64,
}

pub async fn start(c: Config) -> io::Result<()> {
    let addr = if c.addr != "" { c.addr } else { "0.0.0.0" };

    let port = if c.port != 0 { c.port } else { 53 };

    let sock = UdpSocket::bind(format!("{}:{}", addr, port)).await?;
    let sock = Arc::new(sock);

    let (tx, rx) = mpsc::channel::<(Vec<u8>, std::net::SocketAddr)>(1000);
    tokio::spawn(handler(sock.clone(), rx));
    let handle = tokio::spawn(receiver(sock.clone(), tx));

    return handle.await?;
}

async fn receiver(
    sock: Arc<UdpSocket>,
    tx: mpsc::Sender<(Vec<u8>, std::net::SocketAddr)>,
) -> io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }

    return Ok(());
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

    let result = client::forward(req).await?;
    println!("result: {:?}", result);

    sock.send_to(&result, addr).await?;

    Ok(())
}
