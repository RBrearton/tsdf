use crate::core::structs::tsdf_hash::TsdfHash;

/// A trait for objects that can be hashed into a TsdfHash.
pub(crate) trait TsdfHashable {
    fn hash(&self) -> TsdfHash;
}

// Implement this for all types that implement the std library Hash trait.
impl<T: std::hash::Hash> TsdfHashable for T {
    fn hash(&self) -> TsdfHash {
        TsdfHash::new(&self)
    }
}
