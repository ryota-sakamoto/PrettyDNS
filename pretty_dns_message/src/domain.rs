#[derive(Debug, PartialEq)]
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
}

#[cfg(test)]
mod tests {
    use super::Domain;

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
}
