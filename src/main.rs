use std::io;
use tokio::net::UdpSocket;

mod message;

use message::Message;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:53").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        // println!("{:?} bytes received from {:#?}", len, addr);

        println!("---");
        println!("data: {:?}", &buf[..len]);
        println!("len: {:?}", len);

        let req = Message::from_bytes(&buf).unwrap();
        println!("req: {:?}", req);
    }
}
