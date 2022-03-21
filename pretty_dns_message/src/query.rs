use crate::{domain::Domain, qtype::QType};
use nom::{combinator::map, number::complete::be_u16, IResult};
use tokio::io::AsyncWriteExt;

#[derive(Debug, PartialEq)]
pub struct Query {
    pub qname: Domain,
    pub qtype: QType,
    pub qclass: u16,
}

impl Query {
    pub fn read(data: &[u8]) -> IResult<&[u8], Query> {
        let (data, qname) = Domain::read(data)?;
        let (data, qtype) = map(be_u16, |q| q.into())(data)?;
        let (data, qclass) = be_u16(data)?;

        return Ok((
            data,
            Query {
                qname: qname,
                qtype: qtype,
                qclass: qclass,
            },
        ));
    }

    pub async fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut v = vec![];

        v.write_all(&self.qname.to_vec()).await?;
        v.write_u16(self.qtype.into()).await?;
        v.write_u16(self.qclass).await?;

        return Ok(v);
    }
}

#[cfg(test)]
mod tests {
    use super::QType;
    use super::Query;
    use crate::domain::Domain;

    #[tokio::test]
    async fn parse_query() {
        let data: Vec<u8> = vec![
            6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1,
        ];
        let (_, q) = Query::read(&data).unwrap();
        assert_eq!(
            q,
            Query {
                qname: Domain::from(b"google.com.".to_vec()),
                qclass: 1,
                qtype: QType::A,
            }
        );
    }

    #[tokio::test]
    async fn write_query() {
        let q = Query {
            qname: Domain::from(b"google.com.".to_vec()),
            qclass: 1,
            qtype: QType::A.into(),
        };

        let result = q.to_vec().await.unwrap();
        assert_eq!(
            result,
            vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1]
        );
    }
}
