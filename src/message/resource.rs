use nom::{
    combinator::peek,
    multi::count,
    number::complete::{be_u16, be_u32, be_u8},
    IResult,
};
use tokio::io::AsyncWriteExt;

#[derive(Debug, PartialEq)]
pub struct Resource {
    pub name: Vec<u8>,
    pub _type: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl Resource {
    pub fn read(data: &[u8]) -> IResult<&[u8], Resource> {
        let (data, name) = Resource::__read(data)?;
        let (data, _type) = be_u16(data)?;
        let (data, class) = be_u16(data)?;
        let (data, ttl) = be_u32(data)?;
        let (data, rdlength) = be_u16(data)?;
        let (data, rdata) = count(be_u8, rdlength.into())(data)?;

        return Ok((
            data,
            Resource {
                name: name,
                _type: _type,
                class: class,
                ttl: ttl,
                rdlength: rdlength,
                rdata: rdata,
            },
        ));
    }

    fn __read(data: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (data, m1) = peek(be_u8)(data)?;

        // check message compaction
        if (m1 >> 6) == 3 {
            let (data, m1) = be_u8(data)?;
            let (data, m2) = be_u8(data)?;

            return Ok((data, vec![m1, m2]));
        } else {
            return crate::message::query::Query::read_domain(data);
        }
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
        for d in &self.rdata {
            v.write_u8(*d).await?;
        }

        return Ok(v);
    }
}

#[cfg(test)]
mod tests {
    use super::Resource;

    #[tokio::test]
    async fn parse_resource() {
        let data: Vec<u8> = vec![192, 12, 0, 1, 0, 1, 0, 0, 1, 43, 0, 4, 172, 217, 25, 238];
        let (_, q) = Resource::read(&data).unwrap();

        assert_eq!(
            q,
            Resource {
                name: vec![192, 12],
                _type: 1,
                class: 1,
                ttl: 299,
                rdlength: 4,
                rdata: vec![172, 217, 25, 238],
            }
        );
    }

    #[tokio::test]
    async fn write_resource() {
        let h = Resource {
            name: vec![103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 46],
            _type: 1,
            class: 1,
            ttl: 299,
            rdlength: 4,
            rdata: vec![172, 217, 25, 238],
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
