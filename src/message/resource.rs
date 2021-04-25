use std::io::{Cursor, SeekFrom};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

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
    pub async fn from_cursor(c: &mut Cursor<&[u8]>) -> std::io::Result<Resource> {
        let m1 = c.read_u8().await?;
        let m2 = c.read_u8().await?;

        let mut name = vec![];
        if (m1 >> 6) == 3 {
            name.push(m1);
            name.push(m2);
        } else {
            c.seek(SeekFrom::Current(-2)).await?;

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
        }

        return Ok(Resource {
            name: name,
            _type: c.read_u16().await?,
            class: c.read_u16().await?,
            ttl: c.read_u32().await?,
            rdlength: c.read_u16().await?,
            rdata: c.read_u32().await?,
        });
    }

    pub async fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut v = vec![];

        let mut name = vec![];
        if self.name.len() == 2 {
            name.extend_from_slice(&self.name);
        } else {
            for v in self.name.split(|v| *v == 46) {
                name.push(v.len() as u8);
                name.extend_from_slice(v);
            }
        }

        v.write_all(&name).await?;

        v.write_u16(self._type).await?;
        v.write_u16(self.class).await?;
        v.write_u32(self.ttl).await?;
        v.write_u16(self.rdlength).await?;
        v.write_u32(self.rdata).await?;

        return Ok(v);
    }
}

mod tests {
    use super::Resource;

    #[tokio::test]
    async fn parse_resource() {
        let data: Vec<u8> = vec![192, 12, 0, 1, 0, 1, 0, 0, 1, 43, 0, 4, 172, 217, 25, 238];
        let mut c = std::io::Cursor::new(data.as_ref());
        let result = Resource::from_cursor(&mut c).await;

        let q = result.unwrap();
        assert_eq!(q.name, vec![192, 12]);
        assert_eq!(q._type, 1);
        assert_eq!(q.class, 1);
        assert_eq!(q.ttl, 299);
        assert_eq!(q.rdlength, 4);
        assert_eq!(q.rdata, 2899909102);
    }

    #[tokio::test]
    async fn write_resource() {
        let h = Resource {
            name: vec![103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 46],
            _type: 1,
            class: 1,
            ttl: 299,
            rdlength: 4,
            rdata: 2899909102,
        };

        let result = h.to_vec().await.unwrap();
        assert_eq!(
            result,
            vec![
                6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 0, 0, 1, 43, 0, 4,
                172, 217, 25, 238
            ]
        );
    }
}
