use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use pretty_dns_message::{qtype::QType, resource::Resource};
use std::{collections::HashMap, sync::Mutex};
use tracing::debug;

static CACHE: Lazy<Mutex<HashMap<(String, QType), Record>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    cached_at: DateTime<Utc>,
    pub data: CacheData,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CacheData {
    pub answer: Vec<Resource>,
    pub authority: Vec<Resource>,
    pub additional: Vec<Resource>,
}

pub fn resolve(domain: String, qtype: QType) -> Option<CacheData> {
    debug!("try to resolve cache: {:?} {:?}", domain, qtype);
    let mut c = CACHE.lock().unwrap();
    let mut r = c.get(&(domain.clone(), qtype))?.clone();

    if r.expired() {
        debug!("cache is expired: {:?} {:?}", domain, qtype);
        c.remove(&(domain, qtype));
        return None;
    }

    debug!("found cache: {:?} {:?}", domain, qtype);

    r.update_ttl();

    return Some(r.data);
}

impl Record {
    fn expired(&self) -> bool {
        let now = Utc::now();
        let diff = (now - self.cached_at).num_seconds() as u32;

        return self.data.answer.iter().any(|v| v.ttl <= diff);
    }

    fn update_ttl(&mut self) {
        let now = Utc::now();
        let diff = (now - self.cached_at).num_seconds() as u32;

        for i in 0..self.data.answer.len() {
            self.data.answer[i].ttl = self.data.answer[i].ttl - diff;
        }

        for i in 0..self.data.authority.len() {
            self.data.authority[i].ttl = self.data.authority[i].ttl - diff;
        }

        for i in 0..self.data.additional.len() {
            self.data.additional[i].ttl = self.data.additional[i].ttl - diff;
        }
    }
}

pub fn cache(
    domain: String,
    qtype: QType,
    answer: &Vec<Resource>,
    authority: &Vec<Resource>,
    additional: &Vec<Resource>,
) -> Result<(), ()> {
    debug!("store cache: {:?} {:?}", domain, qtype);

    let mut c = CACHE.lock().unwrap();
    c.insert(
        (domain, qtype),
        Record {
            cached_at: Utc::now(),
            data: CacheData {
                answer: answer.clone(),
                authority: authority.clone(),
                additional: additional.clone(),
            },
        },
    );
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::{cache, resolve};
    use pretty_dns_message::{
        compression_domain::{CompressionDomain, DataType},
        domain::Domain,
        qtype::QType,
        resource::Resource,
    };

    #[test]
    fn test_resolve_none() {
        let domain = "example.com.".to_owned();
        let list = resolve(domain, QType::A);
        assert_eq!(list, None);
    }

    #[test]
    fn test_resolve_some() {
        let domain = "test.example.com.".to_owned();
        let resource = Resource {
            name: CompressionDomain::new(vec![
                DataType::Raw(vec![103, 111, 111, 103, 108, 101]),
                DataType::Raw(vec![99, 111, 109]),
            ]),
            _type: QType::A,
            class: 1,
            ttl: 299,
            rdlength: 4,
            rdata: vec![172, 217, 25, 238],
        };
        cache(domain.clone(), QType::A, &vec![resource], &vec![], &vec![]).unwrap();

        let list = resolve(domain, QType::A);
        assert!(list.is_some());
    }
}
