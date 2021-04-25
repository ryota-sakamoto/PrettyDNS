#[derive(Debug)]
pub struct Resource {
    name: Vec<u8>,
    _type: u16,
    class: u16,
    ttl: u32,
    rdlength: u16,
    rdata: u32,
}

impl Resource {
    pub async fn from_bytes(data: &[u8]) -> std::io::Result<Resource> {
        let mut name = vec![];
        let mut qindex = 12;
        loop {
            let label_count = data[qindex] as usize;
            qindex += 1;
            if label_count == 0 {
                break;
            }

            let domain = &data[qindex..(qindex + label_count)];
            name.extend_from_slice(domain);
            name.push(46);

            qindex += label_count;
        }

        qindex = 30;

        return Ok(Resource {
            name: name,
            _type: ((data[qindex] as u16) << 8) + (data[qindex + 1] as u16),
            class: ((data[qindex + 2] as u16) << 8) + (data[qindex + 3] as u16),
            ttl: ((data[qindex + 4] as u32) << 24)
                + ((data[qindex + 5] as u32) << 16)
                + ((data[qindex + 6] as u32) << 8)
                + (data[qindex + 7] as u32),
            rdlength: ((data[qindex + 8] as u16) << 8) + (data[qindex + 9] as u16),
            rdata: ((data[qindex + 10] as u32) << 24)
                + ((data[qindex + 11] as u32) << 16)
                + ((data[qindex + 12] as u32) << 8)
                + (data[qindex + 13] as u32),
        });
    }
}

mod tests {
    use super::Resource;

    #[tokio::test]
    async fn parse_resource() {
        let data = [
            190, 92, 129, 128, 0, 1, 0, 1, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1, 192, 12, 0, 1, 0, 1, 0, 0, 1, 43, 0, 4, 172, 217, 25, 238,
        ];
        let result = Resource::from_bytes(&data).await;

        let q = result.unwrap();
        assert_eq!(
            q.name,
            vec![103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 46]
        );
        assert_eq!(q._type, 1);
        assert_eq!(q.class, 1);
        assert_eq!(q.ttl, 299);
        assert_eq!(q.rdlength, 4);
        assert_eq!(q.rdata, 2899909102);
    }
}
