use std::hash::{Hasher};

// https://docs.rs/http/0.2.5/src/http/extensions.rs.html#8-28
// With TypeIds as keys, there's no need to hash them. They are already hashes
// themselves, coming from the compiler. The IdHasher just holds the u64 of
// the TypeId, and then returns it, instead of doing any bit fiddling.
#[derive(Default)]
pub struct TypeIdHasher(u64);

impl Hasher for TypeIdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use std::mem;
    use std::any::TypeId;
    use std::hash::{Hash, Hasher};

    #[test]
    fn hash_various_types_correctly() {
        
        fn verify_hashing_with(type_id: TypeId) {
            let mut hasher = TypeIdHasher::default();
            type_id.hash(&mut hasher);
            assert_eq!(hasher.finish(), unsafe {
                mem::transmute::<TypeId, u64>(type_id)
            });
        }
        // Pick a variety of types, just to demonstrate it’s all sane. Normal,
        // zero-sized, unsized, &c.
        verify_hashing_with(TypeId::of::<usize>());
        verify_hashing_with(TypeId::of::<()>());
        verify_hashing_with(TypeId::of::<str>());
        verify_hashing_with(TypeId::of::<&str>());
        verify_hashing_with(TypeId::of::<Vec<u8>>());
    }
}