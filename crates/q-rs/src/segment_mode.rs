use crate::version::Version;

/// Describes how a segment's data bits are interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentMode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
    Eci,
}

impl SegmentMode {
    // Returns an unsigned 4-bit integer value (range 0 to 15)
    // representing the mode indicator bits for this mode object.
    pub fn mode_bits(&self) -> u32 {
        use SegmentMode::*;
        match self {
            Numeric => 0x1,
            Alphanumeric => 0x2,
            Byte => 0x4,
            Kanji => 0x8,
            Eci => 0x7,
        }
    }

    // Returns the bit width of the character count field for a segment in this mode
    // in a QR Code at the given version number. The result is in the range [0, 16].
    pub fn num_char_count_bits(&self, ver: Version) -> u8 {
        use SegmentMode::*;
        (match self {
            Numeric => [10, 12, 14],
            Alphanumeric => [9, 11, 13],
            Byte => [8, 16, 16],
            Kanji => [8, 10, 12],
            Eci => [0, 0, 0],
        })[usize::from((ver.value() + 7) / 17)]
    }
}
