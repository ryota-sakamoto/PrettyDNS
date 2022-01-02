use std::{io, sync::Arc};
use tokio::{net::UdpSocket, sync::mpsc};

use crate::{
    client,
    message::{self, qtype::QType, query::Query},
    server::cache,
};

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

    let result = message::from_bytes(&buf);
    if result.is_err() {
        println!("error: {:?}", result.unwrap_err());
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }

    let (_, req) = result.unwrap();
    println!("req: {:?}", req);

    if let Some(q) = &req.query {
        let mut resolve_list = vec![];
        let domain_list = get_domain_list(&q.qname);
        for v in domain_list {
            let record = cache::resolve(v.clone(), q.qtype.clone().into());
            if let Some(_r) = record {
                break;
            } else {
                resolve_list.push(v);
            }
        }

        resolve_list.reverse();
        println!("resolve_list: {:?}", &resolve_list);

        let mut ns = "202.12.27.33:53";
        for r in resolve_list {
            let q = Query {
                qname: r.clone(),
                qtype: QType::A.into(),
                qclass: 1,
            };

            println!("resolve: {:?}, ns: {:?}", q, ns);
            let _result = client::resolve(q, ns).await?;

            if r == "com." {
                ns = "192.5.6.30:53";
            } else if r == "google.com." {
                ns = "216.239.34.10:53";
            }
        }

        let q = Query {
            qname: q.qname.clone(),
            qtype: QType::A.into(),
            qclass: 1,
        };
        println!("resolve: {:?}, ns: {:?}", q, ns);
        let mut result = client::resolve(q, ns).await?;
        result.header.id = req.header.id;
        sock.send_to(&result.to_vec().await?, addr).await?;

        println!("result: {:?}", result);
    }

    Ok(())
}

pub fn get_domain_list(domain: &str) -> Vec<String> {
    let mut domain = domain.to_owned();
    if !domain.ends_with(".") {
        domain += ".";
    }

    let mut result = vec![];
    let mut v: Vec<&str> = domain.split(".").collect();

    while v.len() > 0 {
        if v[0] == "" {
            break;
        }

        result.push(v.join("."));
        v.reverse();
        v.pop();
        v.reverse();
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::get_domain_list;

    #[tokio::test]
    async fn test_get_domain_list() {
        let list = get_domain_list("www.google.com.");
        assert_eq!(list, vec!["www.google.com.", "google.com.", "com."]);
    }
}
