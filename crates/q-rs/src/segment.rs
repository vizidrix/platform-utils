use crate::bit_buffer::BitBuffer;
use crate::segment_mode::SegmentMode;
use crate::version::Version;
use crate::ALPHANUMERIC_CHARSET;

/// A segment of character/binary/control data in a QR Code symbol.
///
/// Instances of this struct are immutable.
///
/// The mid-level way to create a segment is to take the payload data
/// and call a static factory function such as `Segment::make_numeric()`.
/// The low-level way to create a segment is to custom-make the bit buffer
/// and call the `Segment::new()` constructor with appropriate values.
///
/// This segment struct imposes no length restrictions, but QR Codes have restrictions.
/// Even in the most favorable conditions, a QR Code can only hold 7089 characters of data.
/// Any segment longer than this is meaningless for the purpose of generating QR Codes.
#[derive(Clone, PartialEq, Eq)]
pub struct Segment {
    // The mode indicator of this segment. Accessed through mode().
    pub mode: SegmentMode,

    // The length of this segment's unencoded data. Measured in characters for
    // numeric/alphanumeric/kanji mode, bytes for byte mode, and 0 for ECI mode.
    // Not the same as the data's bit length. Accessed through num_chars().
    pub numchars: usize,

    // The data bits of this segment. Accessed through data().
    pub data: Vec<bool>,
}

impl Segment {
    /*---- Static factory functions (mid level) ----*/

    /// Returns a segment representing the given binary data encoded in byte mode.
    ///
    /// All input byte slices are acceptable.
    ///
    /// Any text string can be converted to UTF-8 bytes and encoded as a byte mode segment.
    pub fn make_bytes(data: &[u8]) -> Self {
        let mut bb = BitBuffer(Vec::with_capacity(data.len() * 8));
        for &b in data {
            bb.append_bits(u32::from(b), 8);
        }
        Segment::new(SegmentMode::Byte, data.len(), bb.0)
    }

    /// Returns a segment representing the given string of decimal digits encoded in numeric mode.
    ///
    /// Panics if the string contains non-digit characters.
    pub fn make_numeric(text: &str) -> Self {
        let mut bb = BitBuffer(Vec::with_capacity(text.len() * 3 + (text.len() + 2) / 3));
        let mut accumdata: u32 = 0;
        let mut accumcount: u8 = 0;
        for b in text.bytes() {
            assert!(
                (b'0'..=b'9').contains(&b),
                "String contains non-numeric characters"
            );
            accumdata = accumdata * 10 + u32::from(b - b'0');
            accumcount += 1;
            if accumcount == 3 {
                bb.append_bits(accumdata, 10);
                accumdata = 0;
                accumcount = 0;
            }
        }
        if accumcount > 0 {
            // 1 or 2 digits remaining
            bb.append_bits(accumdata, accumcount * 3 + 1);
        }
        Segment::new(SegmentMode::Numeric, text.len(), bb.0)
    }

    /// Returns a segment representing the given text string encoded in alphanumeric mode.
    ///
    /// The characters allowed are: 0 to 9, A to Z (uppercase only), space,
    /// dollar, percent, asterisk, plus, hyphen, period, slash, colon.
    ///
    /// Panics if the string contains non-encodable characters.
    pub fn make_alphanumeric(text: &str) -> Self {
        let mut bb = BitBuffer(Vec::with_capacity(text.len() * 5 + (text.len() + 1) / 2));
        let mut accumdata: u32 = 0;
        let mut accumcount: u32 = 0;
        for c in text.chars() {
            let i: usize = ALPHANUMERIC_CHARSET
                .find(c)
                .expect("String contains unencodable characters in alphanumeric mode");
            accumdata = accumdata * 45 + u32::try_from(i).unwrap();
            accumcount += 1;
            if accumcount == 2 {
                bb.append_bits(accumdata, 11);
                accumdata = 0;
                accumcount = 0;
            }
        }
        if accumcount > 0 {
            // 1 character remaining
            bb.append_bits(accumdata, 6);
        }
        Segment::new(SegmentMode::Alphanumeric, text.len(), bb.0)
    }

    /// Returns a list of zero or more segments to represent the given Unicode text string.
    ///
    /// The result may use various segment modes and switch
    /// modes to optimize the length of the bit stream.
    pub fn make_segments(text: &str) -> Vec<Self> {
        if text.is_empty() {
            vec![]
        } else {
            vec![if Segment::is_numeric(text) {
                Segment::make_numeric(text)
            } else if Segment::is_alphanumeric(text) {
                Segment::make_alphanumeric(text)
            } else {
                Segment::make_bytes(text.as_bytes())
            }]
        }
    }

    /// Returns a segment representing an Extended Channel Interpretation
    /// (ECI) designator with the given assignment value.
    pub fn make_eci(assignval: u32) -> Self {
        let mut bb = BitBuffer(Vec::with_capacity(24));
        if assignval < (1 << 7) {
            bb.append_bits(assignval, 8);
        } else if assignval < (1 << 14) {
            bb.append_bits(0b10, 2);
            bb.append_bits(assignval, 14);
        } else if assignval < 1_000_000 {
            bb.append_bits(0b110, 3);
            bb.append_bits(assignval, 21);
        } else {
            panic!("ECI assignment value out of range");
        }
        Segment::new(SegmentMode::Eci, 0, bb.0)
    }

    /*---- Constructor (low level) ----*/

    /// Creates a new QR Code segment with the given attributes and data.
    ///
    /// The character count (numchars) must agree with the mode and
    /// the bit buffer length, but the constraint isn't checked.
    pub fn new(mode: SegmentMode, numchars: usize, data: Vec<bool>) -> Self {
        Self {
            mode,
            numchars,
            data,
        }
    }

    /*---- Instance field getters ----*/

    /// Returns the mode indicator of this segment.
    pub fn mode(&self) -> SegmentMode {
        self.mode
    }

    /// Returns the character count field of this segment.
    pub fn num_chars(&self) -> usize {
        self.numchars
    }

    /// Returns the data bits of this segment.
    pub fn data(&self) -> &Vec<bool> {
        &self.data
    }

    /*---- Other static functions ----*/

    // Calculates and returns the number of bits needed to encode the given
    // segments at the given version. The result is None if a segment has too many
    // characters to fit its length field, or the total bits exceeds usize::MAX.
    pub fn get_total_bits(segs: &[Self], version: Version) -> Option<usize> {
        let mut result: usize = 0;
        for seg in segs {
            let ccbits: u8 = seg.mode.num_char_count_bits(version);
            // ccbits can be as large as 16, but usize can be as small as 16
            if let Some(limit) = 1usize.checked_shl(ccbits.into()) {
                if seg.numchars >= limit {
                    return None; // The segment's length doesn't fit the field's bit width
                }
            }
            result = result.checked_add(4 + usize::from(ccbits))?;
            result = result.checked_add(seg.data.len())?;
        }
        Some(result)
    }

    /// Tests whether the given string can be encoded as a segment in numeric mode.
    ///
    /// A string is encodable iff each character is in the range 0 to 9.
    pub fn is_numeric(text: &str) -> bool {
        text.chars().all(|c| ('0'..='9').contains(&c))
    }

    /// Tests whether the given string can be encoded as a segment in alphanumeric mode.
    ///
    /// A string is encodable iff each character is in the following set: 0 to 9, A to Z
    /// (uppercase only), space, dollar, percent, asterisk, plus, hyphen, period, slash, colon.
    pub fn is_alphanumeric(text: &str) -> bool {
        text.chars().all(|c| ALPHANUMERIC_CHARSET.contains(c))
    }
}
