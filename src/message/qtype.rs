#[derive(Debug, PartialEq, Clone)]
pub enum QType {
    A,
    NS,
    Unknown,
}

impl From<u16> for QType {
    fn from(v: u16) -> QType {
        match v {
            1 => QType::A,
            2 => QType::NS,
            _ => QType::Unknown,
        }
    }
}

impl Into<u16> for QType {
    fn into(self) -> u16 {
        match self {
            QType::A => 1,
            QType::NS => 2,
            QType::Unknown => 1,
        }
    }
}
