use super::FileSerializable;

/// A distributed list is a list that is distributed across multiple shards, where the shards may
/// be in completely different locations in a file.
pub(crate) trait DistListTrait<T: FileSerializable> {
    /// Adds an element to the list.
    fn add(&self, elem: T);

    /// Removes an element from the list.
    fn remove(&self, elem: T);

    /// Gets the element at the given index.
    fn get(&self, index: i32) -> T;
}
