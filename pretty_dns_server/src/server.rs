use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::UdpSocket;
use tracing::{error, info, warn};
use tracing_subscriber;

use pretty_dns_cache::cache;
use pretty_dns_client::client;
use pretty_dns_message::{message, qtype::QType, query::Query};

#[derive(Default)]
pub struct Config {
    addr: &'static str,
    port: u64,
}

pub async fn start(c: Config) -> io::Result<()> {
    tracing_subscriber::fmt::init();
    info!("start");

    let addr = if c.addr != "" { c.addr } else { "0.0.0.0" };

    let port = if c.port != 0 { c.port } else { 53 };

    let sock = UdpSocket::bind(format!("{}:{}", addr, port)).await?;
    let sock = Arc::new(sock);

    let mut buf = [0; 1024];
    loop {
        let sock = sock.clone();
        let (len, addr) = sock.recv_from(&mut buf).await?;

        tokio::spawn(async move {
            match handler(buf[..len].to_vec()).await {
                Ok(result) => {
                    sock.send_to(&result.to_vec().await.unwrap(), addr)
                        .await
                        .unwrap();
                }
                Err(e) => {
                    error!("handler error: {:?}", e);
                }
            }
        });
    }

    return Ok(());
}

async fn handler(buf: Vec<u8>) -> io::Result<message::Message> {
    info!("---");
    info!("data: {:?}", buf);

    let result = message::from_bytes(&buf);
    if result.is_err() {
        error!("error: {:?}", result.unwrap_err());
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }

    let (_, req) = result.unwrap();
    info!("req: {:?}", req);

    if req.query.is_none() {
        return Err(std::io::Error::from(std::io::ErrorKind::Other));
    }

    let q = req.query.unwrap();
    let mut resolve_list = vec![];
    let domain_list = get_domain_list(&q.qname);
    for v in domain_list {
        resolve_list.push(v.clone());

        let record = cache::resolve(v);
        info!("cache: {:?}", record);
    }

    resolve_list.reverse();
    info!("resolve_list: {:?}", &resolve_list);

    let mut ns: SocketAddr = "202.12.27.33:53".parse().unwrap();
    for r in resolve_list {
        let q = Query {
            qname: r.clone(),
            qtype: QType::NS.into(),
            qclass: 1,
        };

        info!("resolve: {:?}, ns: {:?}", q, ns);
        let _result = client::resolve(q, ns).await?;
        info!("answer: {:?}", _result.answer);
        if let Some(additional) = _result.additional {
            for a in additional {
                // info!("additional: {:?}", a);
                if a._type != QType::A {
                    continue;
                }

                if a.rdata.len() != 4 {
                    warn!("rdata is not wrong length: {:?}", a.rdata);
                    continue;
                }

                ns = SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(
                        a.rdata[0], a.rdata[1], a.rdata[2], a.rdata[3],
                    )),
                    53,
                );
            }
        }
    }

    let domain = q.qname;
    let query = Query {
        qname: domain.clone(),
        qtype: q.qtype,
        qclass: 1,
    };
    info!("resolve: {:?}, ns: {:?}", query, ns);
    let mut result = client::resolve(query, ns).await?;
    result.header.id = req.header.id;

    info!("result: {:?}", result);
    if let Some(answer) = result.answer.as_ref() {
        cache::cache(domain, answer.clone()).unwrap();
    }

    Ok(result)
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
