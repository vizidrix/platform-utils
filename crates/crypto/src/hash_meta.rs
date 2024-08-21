use crate::Algorithm;

#[derive(Clone, Debug)]
pub struct HashMeta {
    pub algorithm: Algorithm,
    pub hash: Vec<u8>,
}

impl HashMeta {
    pub fn new(algorithm: Algorithm, hash: Vec<u8>) -> Self {
        HashMeta { algorithm, hash }
    }
}

impl std::fmt::Display for HashMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "HashMeta ( alg: {}, hashlen: {}b )",
            self.algorithm,
            self.hash.len()
        )
    }
}
