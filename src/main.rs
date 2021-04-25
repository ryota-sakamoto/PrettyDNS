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
        let result = client::dig(req).await;
        println!("result: {:?}", result);

        sock.send_to(
            &vec![
                buf[0], buf[1], buf[2], buf[3], 0, 1, 0, 1, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108,
                101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 192, 12, 0, 1, 0, 1, 0, 0, 0, 163, 0, 4, 172,
                217, 25, 78,
            ],
            addr,
        )
        .await?;
        // });
    }
}
