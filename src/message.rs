pub mod header;
pub mod query;
pub mod resource;

use nom::{combinator::cond, multi::count};

#[derive(Debug)]
pub struct Message {
    pub header: header::Header,
    pub query: Option<query::Query>,
    pub answer: Option<Vec<resource::Resource>>,
    pub authority: Option<Vec<resource::Resource>>,
    pub additional: Option<Vec<resource::Resource>>,
}

pub async fn from_bytes(data: &[u8]) -> std::io::Result<Message> {
    let (data, h) = header::Header::read(data).unwrap();
    let (data, q) = cond(h.qd_count > 0, query::Query::read)(data).unwrap();
    let (data, a) = cond(
        h.an_count > 0,
        count(resource::Resource::read, h.an_count.into()),
    )(data)
    .unwrap();
    let (data, au) = cond(
        h.ns_count > 0,
        count(resource::Resource::read, h.ns_count.into()),
    )(data)
    .unwrap();
    let (_data, ad) = cond(
        h.ar_count > 0,
        count(resource::Resource::read, h.ar_count.into()),
    )(data)
    .unwrap();

    return Ok(Message {
        header: h,
        query: q,
        answer: a,
        authority: au,
        additional: ad,
    });
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

    #[tokio::test]
    async fn parse_message() {
        let data = vec![
            245, 212, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];
        let result = from_bytes(&data).await;

        let _q = result.unwrap();
    }
}
