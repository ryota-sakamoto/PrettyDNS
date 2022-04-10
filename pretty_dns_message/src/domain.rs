use nom::{
    bytes::complete::take, combinator::flat_map, combinator::peek, multi::fold_many0,
    number::complete::be_u8, IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Domain(Vec<u8>);

impl<T> From<T> for Domain
where
    T: Into<Vec<u8>>,
{
    fn from(data: T) -> Domain {
        return Domain(data.into());
    }
}

impl ToString for Domain {
    fn to_string(&self) -> String {
        String::from_utf8(self.0.clone()).unwrap()
    }
}

impl Domain {
    pub fn read(data: &[u8]) -> IResult<&[u8], Domain> {
        let (data, m1) = peek(be_u8)(data)?;

        // check message compaction
        if (m1 >> 6) == 3 {
            let (data, m1) = be_u8(data)?;
            let (data, m2) = be_u8(data)?;

            return Ok((data, Domain::from(vec![m1, m2])));
        } else {
            let (data, domain) = Domain::read_domain(data)?;
            return Ok((data, Domain::from(domain)));
        }
    }

    pub fn read_domain(data: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (data, qname) = fold_many0(
            Domain::_read_domain,
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

    pub fn split(&self, c: char) -> Vec<Vec<u8>> {
        let c = c as u8;
        let mut result = vec![];
        let mut data = vec![];
        for v in &self.0 {
            let v = *v;
            if v == c {
                result.push(data);
                data = vec![];
                continue;
            }
            data.push(v);
        }

        result.push(data);

        return result;
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut qname = vec![];
        if self.is_compression() {
            qname.extend_from_slice(self.0.as_ref());
            return qname;
        }

        for v in self.split('.') {
            qname.push(v.len() as u8);
            qname.extend_from_slice(v.as_ref());
        }

        return qname;
    }

    pub fn is_compression(&self) -> bool {
        return (self.0[0] >> 6) == 3;
    }
}

#[cfg(test)]
mod tests {
    use super::Domain;

    #[tokio::test]
    async fn test_read() {
        let (_, domain) =
            Domain::read(&vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]).unwrap();
        assert_eq!(
            domain,
            Domain(vec![103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 46]),
        );
    }

    #[tokio::test]
    async fn test_compression_read() {
        let (_, domain) = Domain::read(&vec![192, 12]).unwrap();
        assert_eq!(domain, Domain(vec![192, 12]));
    }

    #[tokio::test]
    async fn test_split() {
        let domain = Domain(b"google.com.".to_vec());
        let result = domain.split('.');
        assert_eq!(
            result,
            [
                vec![103, 111, 111, 103, 108, 101],
                vec![99, 111, 109],
                vec![]
            ]
        );
    }

    #[tokio::test]
    async fn test_to_vec() {
        let domain = Domain(b"google.com.".to_vec());
        let result = domain.to_vec();
        assert_eq!(
            result,
            [6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        );
    }

    #[tokio::test]
    async fn test_compression_to_vec() {
        let domain = Domain(vec![192, 12]);
        let result = domain.to_vec();
        assert_eq!(result, [192, 12]);
    }
}
