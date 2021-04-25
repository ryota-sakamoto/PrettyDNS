use std::io::Cursor;

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
    pub fn from_bytes(data: &[u8]) -> Result<Header, String> {
        return Ok(Header {
            id: ((data[0] as u16) << 8) + (data[1] as u16),
            flag: ((data[2] as u16) << 8) + (data[3] as u16),
            qd_count: ((data[4] as u16) << 8) + (data[5] as u16),
            an_count: ((data[6] as u16) << 8) + (data[7] as u16),
            ns_count: ((data[8] as u16) << 8) + (data[9] as u16),
            ar_count: ((data[10] as u16) << 8) + (data[11] as u16),
        });
    }
}

mod tests {
    use super::Header;

    #[test]
    fn parse_header() {
        let data = [
            196, 171, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];
        let result = Header::from_bytes(&data);
        assert!(result.is_ok());

        let h = result.unwrap();
        assert_eq!(h.id, 50347);
        assert_eq!(h.flag, 288);
        assert_eq!(h.qd_count, 1);
        assert_eq!(h.an_count, 0);
        assert_eq!(h.ns_count, 0);
        assert_eq!(h.ar_count, 0);
    }
}
