use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use pretty_dns_message::{qtype::QType, resource::Resource};
use std::{collections::HashMap, sync::Mutex};
use tracing::info;

static CACHE: Lazy<Mutex<HashMap<(String, QType), Record>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    cached_at: DateTime<Utc>,
    pub data: CacheData,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CacheData {
    pub answer: Option<Vec<Resource>>,
    pub authority: Option<Vec<Resource>>,
    pub additional: Option<Vec<Resource>>,
}

pub fn resolve(domain: String, qtype: QType) -> Option<Record> {
    info!("resolve: {:?}", domain);
    let mut c = CACHE.lock().unwrap();
    let mut r = c.get(&(domain.clone(), qtype))?.clone();

    if r.expired() {
        c.remove(&(domain, qtype));
        return None;
    }

    r.update_ttl();

    return Some(r);
}

impl Record {
    fn expired(&self) -> bool {
        let now = Utc::now();
        let diff = (now - self.cached_at).num_seconds() as u32;

        if let Some(answer) = &self.data.answer {
            return answer.iter().any(|v| v.ttl <= diff);
        } else {
            return false;
        }
    }

    fn update_ttl(&mut self) {
        let now = Utc::now();
        let diff = (now - self.cached_at).num_seconds() as u32;

        if let Some(v) = self.data.answer.as_mut() {
            for i in 0..v.len() {
                v[i].ttl = v[i].ttl - diff;
            }
        }

        if let Some(v) = self.data.authority.as_mut() {
            for i in 0..v.len() {
                v[i].ttl = v[i].ttl - diff;
            }
        }

        if let Some(v) = self.data.additional.as_mut() {
            for i in 0..v.len() {
                v[i].ttl = v[i].ttl - diff;
            }
        }
    }
}

pub fn cache(
    domain: String,
    qtype: QType,
    answer: &Option<Vec<Resource>>,
    authority: &Option<Vec<Resource>>,
    additional: &Option<Vec<Resource>>,
) -> Result<(), ()> {
    info!("cache: {:?}", domain);

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
    use pretty_dns_message::{qtype::QType, resource::Resource};

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
            name: vec![103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 46],
            _type: QType::A,
            class: 1,
            ttl: 299,
            rdlength: 4,
            rdata: vec![172, 217, 25, 238],
        };
        cache(
            domain.clone(),
            QType::A,
            &Some(vec![resource]),
            &None,
            &None,
        )
        .unwrap();

        let list = resolve(domain, QType::A);
        assert!(list.is_some());
    }
}
