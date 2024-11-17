use nom::{
    bytes::complete::take,
    combinator::peek,
    combinator::{cond, flat_map},
    multi::fold_many0,
    number::complete::be_u8,
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct CompressionData(Vec<DataType>);

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Compression { position: u8 },
    Raw(Vec<u8>),
}

impl CompressionData {
    pub fn new(inner: Vec<DataType>) -> CompressionData {
        CompressionData(inner)
    }

    pub fn read<'a>(raw: &'a [u8]) -> IResult<&'a [u8], CompressionData> {
        let mut result = vec![];

        let mut index = 0;
        let mut data = raw.clone();
        loop {
            let (_, flag) = peek(be_u8)(data)?;
            if (flag >> 6) == 3 {
                let (_data, _) = be_u8(data)?;
                let (_data, m2) = be_u8(_data)?;
                data = _data;

                result.push(DataType::Compression { position: m2 });
                break;
            } else if flag != 0 {
                let (_, end) = be_u8(data)?;
                let (remain, _data) = take(end as usize + 1)(&data[index..])?;
                let (_, domain) = Self::read_domain(false)(_data)?;

                data = remain;
                result.push(DataType::Raw(domain));

                index += end as usize + 1;
            } else {
                break;
            }
        }

        Ok((data, CompressionData(result)))
    }

    fn read_domain(is_check_last_zero: bool) -> impl FnMut(&[u8]) -> IResult<&[u8], Vec<u8>> {
        move |data: &[u8]| {
            let (data, qname) = fold_many0(
                Self::_read_domain,
                Vec::new,
                |mut v: Vec<_>, item: &[u8]| {
                    let item = item.to_vec();
                    v.push(item);
                    v
                },
            )(data)?;

            let (data, z) = cond(is_check_last_zero, be_u8)(data)?;
            if let Some(z) = z {
                // read 0 which is the end of the qname
                if z != 0 {
                    return Err(nom::Err::Incomplete(nom::Needed::new(0)));
                }
            }

            return Ok((data, qname.into_iter().flatten().collect()));
        }
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

    pub fn into(self) -> Vec<u8> {
        let mut is_append_zero = false;
        if let Some(DataType::Raw(_)) = self.0.last() {
            is_append_zero = true;
        }

        let mut result: Vec<_> = self
            .0
            .into_iter()
            .map::<Vec<u8>, _>(|v| v.into())
            .flatten()
            .collect();

        if is_append_zero {
            result.push(0);
        }

        result
    }
}

impl Into<Vec<u8>> for DataType {
    fn into(self) -> Vec<u8> {
        match self {
            DataType::Compression { position } => {
                vec![192, position]
            }
            DataType::Raw(v) => {
                let mut result = vec![v.len() as u8];
                result.extend(v);

                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CompressionData, DataType};

    #[tokio::test]
    async fn test_read_compression() {
        let data = vec![192, 12, 0, 1, 0, 1, 0, 0, 1, 43, 0, 4, 172, 217, 25, 238];
        let (data, result) = CompressionData::read(&data).unwrap();
        assert_eq!(data, vec![0, 1, 0, 1, 0, 0, 1, 43, 0, 4, 172, 217, 25, 238]);
        assert_eq!(
            result,
            CompressionData(vec![DataType::Compression { position: 12 }])
        );
    }

    #[tokio::test]
    async fn test_read_compression_mix() {
        let data = vec![1, 98, 192, 12];
        let (data, result) = CompressionData::read(&data).unwrap();
        assert_eq!(data, vec![]);
        assert_eq!(
            result,
            CompressionData(vec![
                DataType::Raw(vec![98]),
                DataType::Compression { position: 12 }
            ])
        );
    }

    #[tokio::test]
    async fn test_into() {
        let data = CompressionData(vec![
            DataType::Raw(vec![98]),
            DataType::Compression { position: 12 },
        ]);
        let result: Vec<u8> = data.into();
        assert_eq!(result, vec![1, 98, 192, 12]);
    }

    #[tokio::test]
    async fn test_into_only_raw() {
        let data = CompressionData(vec![
            DataType::Raw(vec![103, 111, 111, 103, 108, 101]),
            DataType::Raw(vec![99, 111, 109]),
        ]);
        let result: Vec<u8> = data.into();
        assert_eq!(
            result,
            vec![6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0]
        );
    }
}
