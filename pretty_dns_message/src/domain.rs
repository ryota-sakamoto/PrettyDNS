use std::fmt;

#[derive(PartialEq)]
pub struct Domain(Vec<u8>);

impl<'a> fmt::Debug for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Domain").field(&self.0).finish()
    }
}

impl From<Vec<u8>> for Domain {
    fn from(data: Vec<u8>) -> Domain {
        return Domain(data);
    }
}

impl From<String> for Domain {
    fn from(data: String) -> Domain {
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
