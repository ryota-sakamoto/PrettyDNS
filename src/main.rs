use std::io;
use tokio::net::UdpSocket;

mod client;
mod message;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:53").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;

        println!("---");
        println!("data: {:?}", &buf[..len]);
        println!("len: {:?}", len);

        let req = message::from_bytes(&buf).await.unwrap();
        println!("req: {:?}", req);

        // tokio::spawn(async {
        let result = client::dig(req).await?;
        println!("result: {:?}", result);

        sock.send_to(&result, addr).await?;
        // });
    }
}
