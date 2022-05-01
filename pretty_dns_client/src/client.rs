use pretty_dns_message::{header::Header, message::Message, query::Query};
use std::io;
use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    time::{self, Duration},
};
use tracing::info;

pub async fn resolve<T: ToSocketAddrs>(query: Query, ns: T) -> io::Result<Message> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let message = Message {
        header: Header {
            id: 41693,
            qr: 0,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 1,
            ra: 0,
            z: 0,
            ad: 1,
            cd: 0,
            rcode: 0,
            qd_count: 1,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
        },
        query: Some(query),
        answer: vec![],
        authority: vec![],
        additional: vec![],
    };

    sock.send_to(&message.to_vec().await?, ns).await?;

    let result = time::timeout(Duration::from_secs(3), async {
        let mut buf = [0; 1024];

        match sock.recv_from(&mut buf).await {
            Ok(_v) => {
                let (_, res) = Message::from_bytes(&buf).unwrap();
                return Ok(res);
            }
            Err(v) => return Err(v),
        }
    })
    .await?;

    return result;
}

pub async fn forward(req: Message) -> io::Result<Vec<u8>> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let data = req.to_vec().await?;
    sock.send_to(&data, "8.8.8.8:53").await?;

    let result = time::timeout(Duration::from_secs(3), async {
        let mut buf = [0; 1024];

        match sock.recv_from(&mut buf).await {
            Ok(v) => {
                let (_, res) = Message::from_bytes(&buf).unwrap();
                info!("raw: {:?}", &buf[..v.0]);
                info!("res: {:?}", res);
                return Ok(res.to_vec().await?);
            }
            Err(v) => return Err(v),
        }
    })
    .await?;

    return result;
}
