use super::message;
use std::io;
use tokio::{
    net::UdpSocket,
    time::{self, Duration},
};

pub async fn dig(req: message::Message) -> io::Result<String> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    sock.send_to(
        &vec![
            190, 92, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ],
        "8.8.8.8:53",
    )
    .await?;

    time::timeout(Duration::from_secs(3), async {
        let mut buf = [0; 1024];
        let (len, _) = sock.recv_from(&mut buf).await.unwrap();

        let res = message::from_bytes(&buf).await.unwrap();
        println!("res: {:?}", res);
        println!("buf: {:?}", &buf[..len]);
    })
    .await?;

    return Ok(format!(""));
}
