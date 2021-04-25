use std::io::Cursor;
use tokio::io::AsyncReadExt;

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

        let mut name = vec![];
        loop {
            let label_count = c.read_u8().await?;
            if label_count == 0 {
                break;
            }

            let mut buf = vec![0; label_count as usize];
            c.read_exact(&mut buf).await?;

            name.extend_from_slice(&buf);
            name.push(46);
        }

        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;
        c.read_u8().await?;

        return Ok(Resource {
            name: name,
            _type: c.read_u16().await?,
            class: c.read_u16().await?,
            ttl: c.read_u32().await?,
            rdlength: c.read_u16().await?,
            rdata: c.read_u32().await?,
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
