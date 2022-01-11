use nom::{number::complete::be_u16, IResult};

use tokio::io::AsyncWriteExt;

#[derive(Debug, PartialEq)]
pub struct Header {
    pub id: u16,
    pub qr: u8,
    pub opcode: u8,
    pub aa: u8,
    pub tc: u8,
    pub rd: u8,
    pub ra: u8,
    pub z: u8,
    pub ad: u8,
    pub cd: u8,
    pub rcode: u8,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl Header {
    pub fn read(data: &[u8]) -> IResult<&[u8], Header> {
        let (data, id) = be_u16(data)?;
        let (data, flag) = be_u16(data)?;
        let (data, qd_count) = be_u16(data)?;
        let (data, an_count) = be_u16(data)?;
        let (data, ns_count) = be_u16(data)?;
        let (data, ar_count) = be_u16(data)?;

        return Ok((
            data,
            Header {
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
                qd_count: qd_count,
                an_count: an_count,
                ns_count: ns_count,
                ar_count: ar_count,
            },
        ));
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

#[cfg(test)]
mod tests {
    use super::Header;

    #[tokio::test]
    async fn parse_header() {
        let data: Vec<u8> = vec![
            196, 171, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];
        let (_, h) = Header::read(&data).unwrap();
        assert_eq!(
            h,
            Header {
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
            }
        );
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
