use super::{HasMetadataTags, HasName};
use crate::core::enums::HighLevelObject;

/// The high level dir trait. This is like a directory in a file system, but in
/// a tsdf file directories can contain Arrays, other Dirs and metadata tags.
pub(crate) trait DirTrait: HasMetadataTags + HasName {
    /// Returns a list of all the objects in the directory. This includes all
    /// metadata tags, arrays, and subdirectories.
    fn list_dir(&self) -> Vec<&String>;

    /// Returns the object with the given name in the directory. This can be an
    /// Array, another Dir, or a metadata tag.
    fn get(&self, name: String) -> HighLevelObject;
}
