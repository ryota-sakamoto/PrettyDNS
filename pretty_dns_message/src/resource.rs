use crate::{compression::CompressionData, qtype::QType};
use nom::{
    combinator::map,
    multi::count,
    number::complete::{be_u16, be_u32, be_u8},
    IResult,
};
use tokio::io::AsyncWriteExt;

#[derive(Debug, PartialEq, Clone)]
pub struct Resource {
    pub name: CompressionData,
    pub _type: QType,
    pub class: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl Resource {
    pub fn read(data: &[u8]) -> IResult<&[u8], Resource> {
        let (data, name) = CompressionData::from_domain(data)?;
        let (data, _type) = map(be_u16, |q| q.into())(data)?;
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

    pub async fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut v = vec![];

        let name: Vec<u8> = self.name.clone().into();

        v.write_all(&name).await?;

        v.write_u16(self._type.into()).await?;
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
    use super::QType;
    use super::Resource;
    use crate::compression::{CompressionData, CompressionType, DataType};

    #[tokio::test]
    async fn parse_resource() {
        let data: Vec<u8> = vec![192, 12, 0, 1, 0, 1, 0, 0, 1, 43, 0, 4, 172, 217, 25, 238];
        let (_, q) = Resource::read(&data).unwrap();

        assert_eq!(
            q,
            Resource {
                name: CompressionData::new(
                    vec![DataType::Compression { position: 12 }],
                    CompressionType::Domain
                ),
                _type: QType::A,
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
            name: CompressionData::new(
                vec![
                    DataType::Raw(vec![103, 111, 111, 103, 108, 101]),
                    DataType::Raw(vec![99, 111, 109]),
                ],
                CompressionType::Domain,
            ),
            _type: QType::A,
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
