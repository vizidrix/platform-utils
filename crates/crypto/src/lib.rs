mod algorithm;
mod hash_meta;

pub use algorithm::*;
pub use hash_meta::*;

use ring::digest;

/// Perform sha-256 hash on provided data
pub fn hash_sha256(data: &[u8]) -> HashMeta {
    let result = digest::digest(&digest::SHA256, data);
    HashMeta::new(algorithm::Algorithm::SHA256, result.as_ref().to_vec())
}
