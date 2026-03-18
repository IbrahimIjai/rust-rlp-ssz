#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RlpItem {
    Bytes(Vec<u8>),
    List(Vec<RlpItem>),
}
   
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RlpError {
    InputTooShort,
    LengthOutOfBounds,
    NonCanonicalLength,
    NonCanonicalSingleByte,
    TrailingBytes,
    ZeroLenLen,
}

impl std::fmt::Display for RlpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RlpError::InputTooShort => write!(f, "input too short"),
            RlpError::LengthOutOfBounds => write!(f, "length out of bounds"),
            RlpError::NonCanonicalLength => write!(f, "non-canonical length encoding (leading zero)"),
            RlpError::NonCanonicalSingleByte => write!(f, "non-canonical single byte encoding"),
            RlpError::TrailingBytes => write!(f, "trailing bytes after top-level item"),
            RlpError::ZeroLenLen => write!(f, "length-of-length field is zero"),
        }
    }
}

impl std::error::Error for RlpError {}
