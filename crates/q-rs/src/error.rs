use thiserror::Error;

/// The error type when the supplied data does not fit any QR Code version.
///
/// Ways to handle this exception include:
///
/// - Decrease the error correction level if it was greater than `QrCodeEcc::Low`.
/// - If the `encode_segments_advanced()` function was called, then increase the maxversion
///   argument if it was less than `Version::MAX`. (This advice does not apply to the
///   other factory functions because they search all versions up to `Version::MAX`.)
/// - Split the text data into better or optimal segments in order to reduce the number of bits required.
/// - Change the text or binary data to be shorter.
/// - Change the text to fit the character set of a particular segment mode (e.g. alphanumeric).
/// - Propagate the error upward to the caller/user.
#[derive(Debug, Clone, Error)]
#[non_exhaustive]
pub enum QrError {
    /// The input data is too long to fit into any QR Code version at the current error correction level.
    #[error("Segment too long")]
    SegmentTooLong,
    #[error("Data length = {0} bits, Max capacity = {1} bits")]
    DataOverCapacity(usize, usize),
    #[error("Invalid version: {0}")]
    InvalidVersion(u8),
    #[error("Invalid error correction level: {0}")]
    InvalidEcc(u8),
    #[error("Invalid mask: {0}")]
    InvalidMask(u8),
}

// impl std::error::Error for QrError {}

// impl std::fmt::Display for QrError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match *self {
//             Self::SegmentTooLong => write!(f, "Segment too long"),
//             Self::DataOverCapacity(datalen, maxcapacity) => write!(
//                 f,
//                 "Data length = {} bits, Max capacity = {} bits",
//                 datalen, maxcapacity
//             ),
//         }
//     }
// }
