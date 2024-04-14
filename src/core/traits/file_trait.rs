use super::DirTrait;

pub(crate) trait FileTrait: DirTrait {
    /// Returns the version of the file.
    fn get_version(&self) -> &str;

    /// Returns the path to the file.
    fn get_path(&self) -> &str;
}
