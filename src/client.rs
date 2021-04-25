use super::message;
use std::io;
use tokio::{
    net::UdpSocket,
    time::{self, Duration},
};

pub async fn dig(req: message::Message) -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let data = req.to_vec().await?;
    sock.send_to(&data, "8.8.8.8:53").await?;

    time::timeout(Duration::from_secs(3), async {
        let mut buf = [0; 1024];

        match sock.recv_from(&mut buf).await {
            Ok(v) => {
                let res = message::from_bytes(&buf).await.unwrap();
                println!("res: {:?}", res);
                println!("buf: {:?}", &buf[..v.0]);
            }
            Err(v) => {}
        }
    })
    .await?;

    return Ok(());
}
