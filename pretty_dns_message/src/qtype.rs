#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum QType {
    A,
    AAAA,
    NS,
    Unknown(u16),
}

impl From<u16> for QType {
    fn from(v: u16) -> QType {
        match v {
            1 => QType::A,
            2 => QType::NS,
            28 => QType::AAAA,
            _ => QType::Unknown(v),
        }
    }
}

impl Into<u16> for QType {
    fn into(self) -> u16 {
        match self {
            QType::A => 1,
            QType::NS => 2,
            QType::AAAA => 28,
            QType::Unknown(v) => v,
        }
    }
}
