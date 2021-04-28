pub mod header;
pub mod query;
pub mod resource;

use std::io::Cursor;

#[derive(Debug)]
pub struct Message {
    pub header: header::Header,
    pub query: Option<query::Query>,
    pub answer: Option<Vec<resource::Resource>>,
    pub authority: Option<Vec<resource::Resource>>,
    pub additional: Option<Vec<resource::Resource>>,
}

pub async fn from_bytes(data: &[u8]) -> std::io::Result<Message> {
    let mut c = Cursor::new(data);

    let h = header::Header::from_cursor(&mut c).await?;
    let q = if h.qd_count > 0 {
        Some(query::Query::from_cursor(&mut c).await?)
    } else {
        None
    };
    let a = if h.an_count > 0 {
        Some(resource::Resource::from_cursor(&mut c, h.an_count).await?)
    } else {
        None
    };
    let au = if h.ns_count > 0 {
        Some(resource::Resource::from_cursor(&mut c, h.ns_count).await?)
    } else {
        None
    };
    let ad = if h.ar_count > 0 {
        Some(resource::Resource::from_cursor(&mut c, h.ar_count).await?)
    } else {
        None
    };

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

mod tests {
    use super::from_bytes;

    #[tokio::test]
    async fn parse_message() {
        let data = vec![
            245, 212, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];
        let result = from_bytes(&data).await;

        let q = result.unwrap();
    }
}
