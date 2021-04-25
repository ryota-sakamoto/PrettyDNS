pub mod header;
pub mod query;
pub mod resource;

#[derive(Debug)]
pub struct Message {
    header: header::Header,
    query: Option<query::Query>,
    answer: Option<resource::Resource>,
}

pub async fn from_bytes(data: &[u8]) -> std::io::Result<Message> {
    let h = header::Header::from_bytes(data).await.unwrap();
    let q = if h.qd_count > 0 {
        Some(query::Query::from_bytes(data).await?)
    } else {
        None
    };
    let a = if h.an_count > 0 {
        Some(resource::Resource::from_bytes(data).await?)
    } else {
        None
    };

    return Ok(Message {
        header: h,
        query: q,
        answer: a,
    });
}

impl Message {
    pub fn get_fqdn(&self) -> Option<String> {
        return String::from_utf8(self.query.as_ref()?.get_qname().clone()).ok();
    }
}
