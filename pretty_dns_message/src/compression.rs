use crate::domain::Domain;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Compression(Vec<u8>);

impl Compression {
    pub fn read<'a>(data: &'a [u8], raw: &'a [u8]) -> IResult<&'a [u8], Compression> {
        let mut result = vec![];

        let mut index = 0;
        while index < data.len() {
            if (data[index] >> 6) == 3 {
                let position = data[index + 1] as usize;
                let (_, domain) = Domain::read_domain(&raw[position..])?;
                result.extend(domain);

                index += 2;
            } else {
                let end = data[index] as usize + 1;
                let mut v: Vec<_> = data[index..end].into();
                v.push(0);

                let (_, domain) = Domain::read_domain(&v).unwrap();
                result.extend(domain);

                index += end;
            }
        }

        Ok((data, Compression(result)))
    }
}

#[cfg(test)]
mod tests {
    use super::Compression;

    #[tokio::test]
    async fn test_extract_first() {
        let raw = vec![
            245, 212, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];

        let (_, result) = Compression::read(&vec![192, 12], &raw).unwrap();
        assert_eq!(
            result,
            Compression(vec![103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 46]),
        );
    }

    #[tokio::test]
    async fn test_extract() {
        let raw = vec![
            245, 212, 1, 32, 0, 1, 0, 0, 0, 0, 0, 0, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
            109, 0, 0, 1, 0, 1,
        ];

        let (_, result) = Compression::read(&vec![1, 98, 192, 12], &raw).unwrap();
        assert_eq!(
            result,
            Compression(vec![
                98, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 46
            ]),
        );
    }
}
