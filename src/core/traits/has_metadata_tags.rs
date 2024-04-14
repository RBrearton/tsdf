use crate::core::structs::DistDict;

/// The high level HasMetadataTags trait. This is implemented by any object in a tsdf file that can be
/// associated with metadata, which is both Dir and Array.
/// The metadata itself is a dictionary mapping string keys to string values.
pub(crate) trait HasMetadataTags {
    fn get_metadata(&self) -> DistDict<String, String>;
}
