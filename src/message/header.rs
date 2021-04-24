#[derive(Debug)]
pub struct Header {
    id: u16,
    flag: u16,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16,
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
