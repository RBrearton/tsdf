use crate::core::structs::{array::Array, dir::Dir};

pub(crate) enum HighLevelObject {
    Array(Array),
    Dir(Dir),
    MetadataTag(String),
}
