use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct Header {
    id: u16,
    flag: u16,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl Header {
    pub async fn from_bytes(data: &[u8]) -> std::io::Result<Header> {
        let mut c = Cursor::new(data);
        return Ok(Header {
            id: c.read_u16().await?,
            flag: c.read_u16().await?,
            qd_count: c.read_u16().await?,
            an_count: c.read_u16().await?,
            ns_count: c.read_u16().await?,
            ar_count: c.read_u16().await?,
        });
    }

    pub async fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut v = vec![];
        v.write_u16(self.id).await?;
        v.write_u16(self.flag).await?;
        v.write_u16(self.qd_count).await?;
        v.write_u16(self.an_count).await?;
        v.write_u16(self.ns_count).await?;
        v.write_u16(self.ar_count).await?;

        return Ok(v);
    }
}

mod tests {
    use super::Header;

    #[tokio::test]
    async fn parse_header() {
        let data = [
            196, 171, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];
        let result = Header::from_bytes(&data).await;

        let h = result.unwrap();
        assert_eq!(h.id, 50347);
        assert_eq!(h.flag, 288);
        assert_eq!(h.qd_count, 1);
        assert_eq!(h.an_count, 0);
        assert_eq!(h.ns_count, 0);
        assert_eq!(h.ar_count, 0);
    }

    #[tokio::test]
    async fn write_header() {
        let h = Header {
            id: 50347,
            flag: 288,
            qd_count: 1,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
        };

        let result = h.to_vec().await.unwrap();
        assert_eq!(result, vec![196, 171, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0]);
    }
}
