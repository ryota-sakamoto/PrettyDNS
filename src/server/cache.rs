use crate::message::resource::Resource;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};
use tracing::info;

static CACHE: Lazy<Mutex<HashMap<String, Record>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug)]
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
