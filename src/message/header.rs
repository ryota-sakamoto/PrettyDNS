use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct Header {
    id: u16,
    qr: u8,
    opcode: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    ad: u8,
    cd: u8,
    rcode: u8,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl Header {
    pub async fn from_cursor(c: &mut Cursor<&[u8]>) -> std::io::Result<Header> {
        let id = c.read_u16().await?;
        let flag = c.read_u16().await?;
        return Ok(Header {
            id: id,
            qr: ((flag & (1 << 15)) != 0) as u8,
            opcode: (flag & 0b0111100000000000) as u8,
            aa: ((flag & (1 << 10)) != 0) as u8,
            tc: ((flag & (1 << 9)) != 0) as u8,
            rd: ((flag & (1 << 8)) != 0) as u8,
            ra: ((flag & (1 << 7)) != 0) as u8,
            z: ((flag & (1 << 6)) != 0) as u8,
            ad: ((flag & (1 << 5)) != 0) as u8,
            cd: ((flag & (1 << 4)) != 0) as u8,
            rcode: (flag & 0b1111) as u8,
            qd_count: c.read_u16().await?,
            an_count: c.read_u16().await?,
            ns_count: c.read_u16().await?,
            ar_count: c.read_u16().await?,
        });
    }

    pub async fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut v = vec![];
        v.write_u16(self.id).await?;

        let mut flag = 0;
        flag += (self.qr as u16) << 15;
        flag += (self.opcode as u16) << 11;
        flag += (self.aa as u16) << 10;
        flag += (self.tc as u16) << 9;
        flag += (self.rd as u16) << 8;
        flag += (self.ra as u16) << 7;
        flag += (self.z as u16) << 6;
        flag += (self.ad as u16) << 5;
        flag += (self.cd as u16) << 4;
        flag += self.rcode as u16;
        v.write_u16(flag).await?;

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
        let data: Vec<u8> = vec![
            196, 171, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];
        let mut c = std::io::Cursor::new(data.as_ref());
        let result = Header::from_cursor(&mut c).await;

        let h = result.unwrap();
        assert_eq!(h.id, 50347);
        assert_eq!(h.qr, 0);
        assert_eq!(h.opcode, 0);
        assert_eq!(h.aa, 0);
        assert_eq!(h.tc, 0);
        assert_eq!(h.rd, 1);
        assert_eq!(h.ra, 0);
        assert_eq!(h.z, 0);
        assert_eq!(h.ad, 1);
        assert_eq!(h.cd, 0);
        assert_eq!(h.rcode, 0);
        assert_eq!(h.qd_count, 1);
        assert_eq!(h.an_count, 0);
        assert_eq!(h.ns_count, 0);
        assert_eq!(h.ar_count, 0);
    }

    #[tokio::test]
    async fn write_header() {
        let h = Header {
            id: 50347,
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
        };

        let result = h.to_vec().await.unwrap();
        assert_eq!(result, vec![196, 171, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0]);
    }
}
