pub mod header;
pub mod query;

#[derive(Debug)]
pub struct Message {
    header: header::Header,
    query: query::Query,
}

impl Message {
    pub fn from_bytes(data: &[u8]) -> Result<Message, String> {
        return Ok(Message {
            header: header::Header::from_bytes(data)?,
            query: query::Query::from_bytes(data)?,
        });
    }
}
