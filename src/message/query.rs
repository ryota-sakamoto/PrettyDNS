#[derive(Debug)]
pub struct Query {
    qname: Vec<u8>,
    qtype: u16,
    qclass: u16,
}

impl Query {
    pub async fn from_bytes(data: &[u8]) -> std::io::Result<Query> {
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
