use crate::{domain::Domain, qtype::QType};
use nom::{
    bytes::complete::take,
    combinator::{flat_map, map},
    multi::fold_many0,
    number::complete::{be_u16, be_u8},
    IResult,
};
use tokio::io::AsyncWriteExt;

#[derive(Debug, PartialEq)]
pub struct Query {
    pub qname: Domain,
    pub qtype: QType,
    pub qclass: u16,
}

impl Query {
    pub fn read(data: &[u8]) -> IResult<&[u8], Query> {
        let (data, qname) = Query::read_domain(data)?;
        let (data, qtype) = map(be_u16, |q| q.into())(data)?;
        let (data, qclass) = be_u16(data)?;

        return Ok((
            data,
            Query {
                qname: Domain::from(qname),
                qtype: qtype,
                qclass: qclass,
            },
        ));
    }

    pub fn read_domain(data: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (data, qname) = fold_many0(
            Query::_read_domain,
            Vec::new,
            |mut v: Vec<_>, item: &[u8]| {
                let mut item = item.to_vec();
                item.push(46);
                v.push(item);
                v
            },
        )(data)?;

        // read 0 of the end of the qname
        let (data, z) = be_u8(data)?;
        if z != 0 {
            return Err(nom::Err::Incomplete(nom::Needed::new(0)));
        }

        return Ok((data, qname.into_iter().flatten().collect()));
    }

    fn _read_domain(data: &[u8]) -> IResult<&[u8], &[u8]> {
        let (data, a) = flat_map(be_u8, take)(data)?;
        if a.len() == 0 {
            return Err(nom::Err::Error(nom::error::make_error(
                data,
                nom::error::ErrorKind::Eof,
            )));
        }

        return Ok((data, a));
    }

    pub async fn to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut v = vec![];

        let mut qname = vec![];
        for v in self.qname.split('.') {
            qname.push(v.len() as u8);
            qname.extend_from_slice(v.as_ref());
        }

        v.write_all(&qname).await?;

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
