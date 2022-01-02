#[derive(Debug, PartialEq, Clone)]
pub enum QType {
    A,
    Unknown,
}

impl From<u16> for QType {
    fn from(v: u16) -> QType {
        match v {
            1 => QType::A,
            _ => QType::Unknown,
        }
    }
}

impl Into<u16> for QType {
    fn into(self) -> u16 {
        return 1;
    }
}
