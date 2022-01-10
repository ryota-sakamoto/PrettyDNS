use crate::message::resource::Resource;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};
use tracing::info;

static CACHE: Lazy<Mutex<HashMap<String, Record>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    cached_at: DateTime<Utc>,
    data: Vec<Resource>,
}

pub fn resolve(domain: String) -> Option<Record> {
    info!("resolve: {:?}", domain);
    let mut c = CACHE.lock().unwrap();
    let mut r = c.get(&domain)?.clone();

    let now = Utc::now();
    let diff = (now - r.cached_at).num_seconds() as u32;

    for v in r.data.iter() {
        if v.ttl <= diff {
            c.remove(&domain);
            return None;
        }
    }

    let mut data = r.data.clone();
    for (i, v) in r.data.iter().enumerate() {
        data[i].ttl = v.ttl - diff;
    }

    // cache(domain, data.clone()).unwrap();
    r.data = data;

    return Some(r);
}

pub fn cache(domain: String, data: Vec<Resource>) -> Result<(), ()> {
    info!("cache: {:?}", domain);

    let mut c = CACHE.lock().unwrap();
    c.insert(
        domain,
        Record {
            cached_at: Utc::now(),
            data: data,
        },
    );
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::{cache, resolve};
    use crate::message::{qtype::QType, resource::Resource};

    #[tokio::test]
    async fn test_resolve_none() {
        let domain = "example.com.".to_owned();
        let list = resolve(domain);
        assert_eq!(list, None);
    }

    #[tokio::test]
    async fn test_resolve_some() {
        let domain = "test.example.com.".to_owned();
        let resource = Resource {
            name: vec![103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 46],
            _type: QType::A,
            class: 1,
            ttl: 299,
            rdlength: 4,
            rdata: vec![172, 217, 25, 238],
        };
        cache(domain.clone(), vec![resource]).unwrap();

        let list = resolve(domain);
        assert!(list.is_some());
    }
}
