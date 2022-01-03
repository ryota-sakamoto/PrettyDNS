use crate::message::*;
use nom::{combinator::cond, multi::count, IResult};

#[derive(Debug, PartialEq)]
pub struct Message {
    pub header: header::Header,
    pub query: Option<query::Query>,
    pub answer: Option<Vec<resource::Resource>>,
    pub authority: Option<Vec<resource::Resource>>,
    pub additional: Option<Vec<resource::Resource>>,
}

pub fn from_bytes(data: &[u8]) -> IResult<&[u8], Message> {
    let (data, h) = header::Header::read(data)?;
    let (data, q) = cond(h.qd_count > 0, query::Query::read)(data)?;
    let (data, a) = cond(
        h.an_count > 0,
        count(resource::Resource::read, h.an_count.into()),
    )(data)?;
    let (data, au) = cond(
        h.ns_count > 0,
        count(resource::Resource::read, h.ns_count.into()),
    )(data)?;
    let (_data, ad) = cond(
        h.ar_count > 0,
        count(resource::Resource::read, h.ar_count.into()),
    )(data)?;

    return Ok((
        data,
        Message {
            header: h,
            query: q,
            answer: a,
            authority: au,
            additional: ad,
        },
    ));
}

impl Message {
    pub async fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut result = vec![];

        let h = self.header.to_vec().await?;
        result.extend_from_slice(&h);

        if let Some(ref v) = self.query {
            let q = v.to_vec().await?;
            result.extend_from_slice(&q);
        }

        if let Some(ref v) = self.answer {
            for v in v {
                let a = v.to_vec().await?;
                result.extend_from_slice(&a);
            }
        }

        if let Some(ref v) = self.authority {
            for v in v {
                let a = v.to_vec().await?;
                result.extend_from_slice(&a);
            }
        }

        if let Some(ref v) = self.additional {
            for v in v {
                let a = v.to_vec().await?;
                result.extend_from_slice(&a);
            }
        }

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    use super::from_bytes;
    use super::qtype::QType;

    #[tokio::test]
    async fn parse_message() {
        let data = vec![
            245, 212, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];
        let (_, result) = from_bytes(&data).unwrap();

        assert_eq!(
            result,
            super::Message {
                header: super::header::Header {
                    id: 62932,
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
                query: Some(super::query::Query {
                    qname: "google.com.".to_owned(),
                    qclass: 1,
                    qtype: QType::A,
                }),
                answer: None,
                authority: None,
                additional: None,
            }
        );
    }
}
