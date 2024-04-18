use crate::core::structs::tsdf_hash::TsdfHash;

/// A trait for objects that can be hashed into a TsdfHash.
pub(crate) trait TsdfHashable {
    fn hash(&self) -> TsdfHash;
}
