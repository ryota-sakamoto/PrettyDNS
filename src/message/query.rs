use nom::{
    bytes::complete::take,
    combinator::{cond, flat_map},
    multi::fold_many0,
    number::complete::{be_u16, be_u8},
    IResult,
};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct Query {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl Query {
    pub fn read(data: &[u8]) -> IResult<&[u8], Query> {
        let (data, qname) = fold_many0(
            Query::read_domain,
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

        let qname: Vec<u8> = qname.into_iter().flatten().collect();
        let q = String::from_utf8(qname).unwrap();
        let (data, qtype) = be_u16(data)?;
        let (data, qclass) = be_u16(data)?;

        return Ok((
            data,
            Query {
                qname: q,
                qtype: qtype,
                qclass: qclass,
            },
        ));
    }

    fn read_domain(data: &[u8]) -> IResult<&[u8], &[u8]> {
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
        for v in self.qname.split(|v| v == '.') {
            qname.push(v.len() as u8);
            qname.extend_from_slice(v.as_bytes());
        }

        v.write_all(&qname).await?;

        v.write_u16(self.qtype).await?;
        v.write_u16(self.qclass).await?;

        return Ok(v);
    }
}

#[cfg(test)]
mod tests {
    use super::Query;

    #[tokio::test]
    async fn parse_query() {
        let data: Vec<u8> = vec![
            6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1,
        ];
        let (_, q) = Query::read(&data).unwrap();
        assert_eq!(q.qname, "google.com.");
        assert_eq!(q.qclass, 1);
        assert_eq!(q.qtype, 1);
    }

    #[tokio::test]
    async fn write_query() {
        let q = Query {
            qname: "google.com.".to_owned(),
            qclass: 1,
            qtype: 1,
        };

        let result = q.to_vec().await.unwrap();
        assert_eq!(
            result,
            vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1]
        );
    }
}
