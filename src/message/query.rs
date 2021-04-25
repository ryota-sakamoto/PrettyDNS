use std::io::Cursor;
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub struct Query {
    qname: Vec<u8>,
    qtype: u16,
    qclass: u16,
}

impl Query {
    pub async fn from_bytes(data: &[u8]) -> std::io::Result<Query> {
        let mut c = Cursor::new(data);

        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;

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

        return Ok(Query {
            qname: qname,
            qtype: c.read_u16().await?,
            qclass: c.read_u16().await?,
        });
    }

    pub fn get_qname(&self) -> &Vec<u8> {
        return &self.qname;
    }
}

mod tests {
    use super::Query;

    #[tokio::test]
    async fn parse_query() {
        let data = [
            196, 171, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];
        let result = Query::from_bytes(&data).await;

        let q = result.unwrap();
        assert_eq!(
            q.qname,
            vec![103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 46]
        );
        assert_eq!(q.qclass, 1);
        assert_eq!(q.qtype, 1);
    }
}
