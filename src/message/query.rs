#[derive(Debug)]
pub struct Query {
    qname: Vec<u8>,
    qtype: u16,
    qclass: u16,
}

impl Query {
    pub fn from_bytes(data: &[u8]) -> Result<Query, String> {
        let mut qname = vec![];
        let mut qindex = 12;
        loop {
            let label_count = data[qindex] as usize;
            qindex += 1;
            if label_count == 0 {
                break;
            }

            let domain = &data[qindex..(qindex + label_count)];
            qname.extend_from_slice(domain);
            qname.push(46);

            qindex += label_count;
        }

        return Ok(Query {
            qname: qname,
            qtype: ((data[qindex] as u16) << 8) + (data[qindex + 1] as u16),
            qclass: ((data[qindex + 2] as u16) << 8) + (data[qindex + 3] as u16),
        });
    }
}
