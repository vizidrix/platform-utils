//! Generates QR Codes from text strings and byte arrays.
//!
//! This project aims to be the best, clearest QR Code generator library.
//! The primary goals are flexible options and absolute correctness.
//! Secondary goals are compact implementation size and good documentation comments.
//!
//! Home page with live JavaScript demo, extensive descriptions, and competitor comparisons:
//! [https://www.nayuki.io/page/qr-code-generator-library](https://www.nayuki.io/page/qr-code-generator-library)
//!
//! # Features
//!
//! Core features:
//!
//! - Significantly shorter code but more documentation comments compared to competing libraries
//! - Supports encoding all 40 versions (sizes) and all 4 error correction levels, as per the QR Code Model 2 standard
//! - Output format: Raw modules/pixels of the QR symbol
//! - Detects finder-like penalty patterns more accurately than other implementations
//! - Encodes numeric and special-alphanumeric text in less space than general text
//! - Open-source code under the permissive MIT License
//!
//! Manual parameters:
//!
//! - User can specify minimum and maximum version numbers allowed, then library will automatically choose smallest version in the range that fits the data
//! - User can specify mask pattern manually, otherwise library will automatically evaluate all 8 masks and select the optimal one
//! - User can specify absolute error correction level, or allow the library to boost it if it doesn't increase the version number
//! - User can create a list of data segments manually and add ECI segments
//!
//! More information about QR Code technology and this library's design can be found on the project home page.
//!
//! # Examples
//!
//! ```
//! use qr::Mask;
//! use qr::Code;
//! use qr::CodeEcc;
//! use qr::Segment;
//! use qr::Version;
//! ```
//!
//! Simple operation:
//!
//! ```
//! let qr = Code::encode_text("Hello, world!",
//!     CodeEcc::Medium).unwrap();
//! let svg = to_svg_string(&qr, 4);  // See qrcodegen-demo
//! ```
//!
//! Manual operation:
//!
//! ```
//! let text: &str = "3141592653589793238462643383";
//! let segs = Segment::make_segments(text);
//! let qr = Code::encode_segments_advanced(&segs, CodeEcc::High,
//!     Version::new(5), Version::new(5), Some(Mask::new(2)), false).unwrap();
//! for y in 0 .. qr.size() {
//!     for x in 0 .. qr.size() {
//!         (... paint qr.get_module(x, y) ...)
//!     }
//! }
//! ```

// #![forbid(unsafe_code)]
// use std::convert::TryFrom;

// The set of all legal characters in alphanumeric mode,
// where each character value maps to the index in the string.
pub static ALPHANUMERIC_CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";

mod bit_buffer;
mod code_ecc;
mod error;
mod finder_penalty;
mod mask;
mod qr_code;
mod segment;
mod segment_mode;
mod version;

pub use bit_buffer::*;
pub use code_ecc::*;
pub use error::*;
pub use finder_penalty::*;
pub use mask::*;
pub use qr_code::*;
pub use segment::*;
pub use segment_mode::*;
pub use version::*;

/*---- Constants and tables ----*/

// For use in get_penalty_score(), when evaluating which mask is best.
pub const PENALTY_N1: i32 = 3;
pub const PENALTY_N2: i32 = 3;
pub const PENALTY_N3: i32 = 40;
pub const PENALTY_N4: i32 = 10;

pub static ECC_CODEWORDS_PER_BLOCK: [[i8; 41]; 4] = [
    // Version: (note that index 0 is for padding, and is set to an illegal value)
    //0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40    Error correction level
    [
        -1, 7, 10, 15, 20, 26, 18, 20, 24, 30, 18, 20, 24, 26, 30, 22, 24, 28, 30, 28, 28, 28, 28,
        30, 30, 26, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    ], // Low
    [
        -1, 10, 16, 26, 18, 24, 16, 18, 22, 22, 26, 30, 22, 22, 24, 24, 28, 28, 26, 26, 26, 26, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    ], // Medium
    [
        -1, 13, 22, 18, 26, 18, 24, 18, 22, 20, 24, 28, 26, 24, 20, 30, 24, 28, 28, 26, 30, 28, 30,
        30, 30, 30, 28, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    ], // Quartile
    [
        -1, 17, 28, 22, 16, 22, 28, 26, 26, 24, 28, 24, 28, 22, 24, 24, 30, 28, 28, 26, 28, 30, 24,
        30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    ], // High
];

pub static NUM_ERROR_CORRECTION_BLOCKS: [[i8; 41]; 4] = [
    // Version: (note that index 0 is for padding, and is set to an illegal value)
    //0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40    Error correction level
    [
        -1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 4, 4, 4, 4, 4, 6, 6, 6, 6, 7, 8, 8, 9, 9, 10, 12, 12, 12,
        13, 14, 15, 16, 17, 18, 19, 19, 20, 21, 22, 24, 25,
    ], // Low
    [
        -1, 1, 1, 1, 2, 2, 4, 4, 4, 5, 5, 5, 8, 9, 9, 10, 10, 11, 13, 14, 16, 17, 17, 18, 20, 21,
        23, 25, 26, 28, 29, 31, 33, 35, 37, 38, 40, 43, 45, 47, 49,
    ], // Medium
    [
        -1, 1, 1, 2, 2, 4, 4, 6, 6, 8, 8, 8, 10, 12, 16, 12, 17, 16, 18, 21, 20, 23, 23, 25, 27,
        29, 34, 34, 35, 38, 40, 43, 45, 48, 51, 53, 56, 59, 62, 65, 68,
    ], // Quartile
    [
        -1, 1, 1, 2, 4, 4, 4, 5, 6, 8, 8, 11, 11, 16, 16, 18, 16, 19, 21, 25, 25, 25, 34, 30, 32,
        35, 37, 40, 42, 45, 48, 51, 54, 57, 60, 63, 66, 70, 74, 77, 81,
    ], // High
];
