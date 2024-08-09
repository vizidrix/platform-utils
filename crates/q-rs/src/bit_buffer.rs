/// An appendable sequence of bits (0s and 1s).
///
/// Mainly used by Segment.
pub struct BitBuffer(pub Vec<bool>);

// Returns true iff the i'th bit of x is set to 1.
pub fn get_bit(x: u32, i: i32) -> bool {
    (x >> i) & 1 != 0
}

impl BitBuffer {
    /// Appends the given number of low-order bits of the given value to this buffer.
    ///
    /// Requires len &#x2264; 31 and val &lt; 2<sup>len</sup>.
    pub fn append_bits(&mut self, val: u32, len: u8) {
        assert!(len <= 31 && val >> len == 0, "Value out of range");
        self.0
            .extend((0..i32::from(len)).rev().map(|i| get_bit(val, i))); // Append bit by bit
    }
}
