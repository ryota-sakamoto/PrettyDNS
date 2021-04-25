pub mod header;
pub mod query;
pub mod resource;

use std::io::Cursor;

#[derive(Debug)]
pub struct Message {
    header: header::Header,
    query: Option<query::Query>,
    answer: Option<resource::Resource>,
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
        Some(resource::Resource::from_cursor(&mut c).await?)
    } else {
        None
    };

    return Ok(Message {
        header: h,
        query: q,
        answer: a,
    });
}

impl Message {
    pub fn get_fqdn(&self) -> Option<String> {
        return String::from_utf8(self.query.as_ref()?.get_qname().clone()).ok();
    }

    pub async fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut result = vec![];

        let h = self.header.to_vec().await?;
        result.extend_from_slice(&h);

        if let Some(ref v) = self.query {
            let q = v.to_vec().await?;
            result.extend_from_slice(&q);
        }

        if let Some(ref v) = self.answer {
            let a = v.to_vec().await?;
            result.extend_from_slice(&a);
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
