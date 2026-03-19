
pub const BYTES_PER_OFFSET: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SszError {
    /// Input buffer was shorter than expected for the type being decoded.
    InputTooShort,
    /// A boolean byte had a value other than 0x00 or 0x01.
    InvalidBoolean(u8),
    /// An offset pointed outside the bounds of the input buffer.
    OffsetOutOfBounds { offset: usize, len: usize },
    /// Offsets were not in strictly ascending order (malformed container).
    OffsetsNotAscending,
    /// The byte length of a fixed-element sequence was not a multiple of the element size.
    InvalidLength { got: usize, element_size: usize },
    /// A variable-length list exceeded its declared maximum length.
    ListTooLong { len: usize, max: usize },
    /// The first offset in a container did not equal the size of the fixed part.
    InvalidFirstOffset { got: usize, expected: usize },
    /// A BitList was missing its sentinel bit.
    MissingSentinelBit,
    /// Extra bits were set beyond the declared length in a Bitvector.
    ExtraBitsSet,
}

impl std::fmt::Display for SszError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SszError::InputTooShort =>
                write!(f, "input too short"),
            SszError::InvalidBoolean(b) =>
                write!(f, "invalid boolean byte: 0x{b:02x} (must be 0x00 or 0x01)"),
            SszError::OffsetOutOfBounds { offset, len } =>
                write!(f, "offset {offset} is out of bounds for input of length {len}"),
            SszError::OffsetsNotAscending =>
                write!(f, "offsets are not in ascending order"),
            SszError::InvalidLength { got, element_size } =>
                write!(f, "byte length {got} is not a multiple of element size {element_size}"),
            SszError::ListTooLong { len, max } =>
                write!(f, "list length {len} exceeds maximum {max}"),
            SszError::InvalidFirstOffset { got, expected } =>
                write!(f, "first offset {got} does not equal fixed-part size {expected}"),
            SszError::MissingSentinelBit =>
                write!(f, "bitlist is missing its sentinel bit"),
            SszError::ExtraBitsSet =>
                write!(f, "bitvector has extra bits set beyond declared length"),
        }
    }
}

impl std::error::Error for SszError {}
