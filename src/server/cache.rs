use std::{
    collections::HashMap,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use once_cell::sync::Lazy;

static CACHE: Lazy<Mutex<HashMap<String, Record>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug)]
pub struct Record {
    pub ip: String,
    pub expire: u64,
}

pub fn resolve(domain: String, _type: u16) -> Option<Record> {
    let mut c = CACHE.lock().unwrap();
    let record = c.get(&domain).map(|v| v.clone());
    if let Some(r) = &record {
        let t = SystemTime::now();
        let now = t.duration_since(UNIX_EPOCH).unwrap();
        if r.expire <= now.as_secs() {
            c.remove(&domain);
            return None;
        }
    }

    return record;
}

pub fn cache(domain: String, _type: u16, record: Record) -> Result<(), String> {
    let mut c = CACHE.lock().unwrap();
    c.insert(domain, record);
    return Ok(());
}
