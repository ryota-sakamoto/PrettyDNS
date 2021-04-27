use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct Query {
    qname: String,
    qtype: u16,
    qclass: u16,
}

impl Query {
    pub async fn from_cursor(c: &mut Cursor<&[u8]>) -> std::io::Result<Query> {
        let mut qname = vec![];
        loop {
            let label_count = c.read_u8().await?;
            if label_count == 0 {
                break;
            }

            let mut buf = vec![0; label_count as usize];
            c.read_exact(&mut buf).await?;

            qname.extend_from_slice(&buf);
            qname.push(46);
        }

        let q = String::from_utf8(qname).unwrap();

        return Ok(Query {
            qname: q,
            qtype: c.read_u16().await?,
            qclass: c.read_u16().await?,
        });
    }

    pub async fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut v = vec![];

        let mut qname = vec![];
        for v in self.qname.split(|v| v == '.') {
            qname.push(v.len() as u8);
            qname.extend_from_slice(v.as_bytes());
        }

        v.write_all(&qname).await?;

        v.write_u16(self.qtype).await?;
        v.write_u16(self.qclass).await?;

        return Ok(v);
    }
}

mod tests {
    use super::Query;

    #[tokio::test]
    async fn parse_query() {
        let data: Vec<u8> = vec![
            6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1,
        ];
        let mut c = std::io::Cursor::new(data.as_ref());
        let result = Query::from_cursor(&mut c).await;

        let q = result.unwrap();
        assert_eq!(q.qname, "google.com.");
        assert_eq!(q.qclass, 1);
        assert_eq!(q.qtype, 1);
    }

    #[tokio::test]
    async fn write_query() {
        let q = Query {
            qname: "google.com.".to_owned(),
            qclass: 1,
            qtype: 1,
        };

        let result = q.to_vec().await.unwrap();
        assert_eq!(
            result,
            vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1]
        );
    }
}
