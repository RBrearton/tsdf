use super::HighLevelObjectMetadata;
use crate::core::enums::HighLevelObject;

pub struct Dir {
    contents: Vec<HighLevelObject>,
    metadata: HighLevelObjectMetadata,
}
