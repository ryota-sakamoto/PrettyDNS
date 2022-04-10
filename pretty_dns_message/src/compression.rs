use crate::domain::Domain;
use nom::{bytes::complete::take, combinator::peek, number::complete::be_u8, IResult};

#[derive(Debug, PartialEq, Clone)]
pub struct CompressionData(Vec<DataType>);

#[derive(Debug, PartialEq, Clone)]
enum DataType {
    Compression { position: u8 },
    Raw(Vec<u8>),
}

impl CompressionData {
    pub fn read<'a>(raw: &'a [u8]) -> IResult<&'a [u8], CompressionData> {
        let mut result = vec![];

        let mut index = 0;
        let mut data = raw.clone();
        while index < raw.len() {
            let (_, flag) = peek(be_u8)(data)?;
            if (flag >> 6) == 3 {
                let (_data, _) = be_u8(data)?;
                let (_data, m2) = be_u8(_data)?;
                data = _data;

                result.push(DataType::Compression { position: m2 });

                index += 2;
            } else {
                let (_, end) = be_u8(data)?;
                let (remain, _data) = take(end as usize + 1)(&data[index..])?;
                let (_, domain) = Domain::read_domain(false)(_data)?;

                data = remain;
                result.push(DataType::Raw(domain));

                index += end as usize + 1;
            }
        }

        Ok((data, CompressionData(result)))
    }
}

#[cfg(test)]
mod tests {
    use super::{CompressionData, DataType};

    #[tokio::test]
    async fn test_extract_first() {
        let (_, result) = CompressionData::read(&vec![192, 12]).unwrap();
        assert_eq!(
            result,
            CompressionData(vec![DataType::Compression { position: 12 }])
        );
    }

    #[tokio::test]
    async fn test_extract() {
        let (_, result) = CompressionData::read(&vec![1, 98, 192, 12]).unwrap();
        assert_eq!(
            result,
            CompressionData(vec![
                DataType::Raw(vec![98, 46]),
                DataType::Compression { position: 12 }
            ])
        );
    }
}
