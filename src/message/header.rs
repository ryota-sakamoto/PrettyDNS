use std::io::Cursor;
use tokio::io::AsyncReadExt;

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
}
