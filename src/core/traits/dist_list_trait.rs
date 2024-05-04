use super::FileSerializable;

/// A distributed list is a list that is distributed across multiple shards,
/// where the shards may be in completely different locations in a file.
pub(crate) trait DistListTrait<T: FileSerializable> {
    /// Adds an element to the list.
    fn add(&self, elem: T);

    /// Adds a list of elements to the list.
    fn add_all(&self, elems: Vec<T>);

    /// Updates the element at the given index.
    fn update(&self, index: i32, elem: T);

    /// Gets the element at the given index.
    fn get(&self, index: i32) -> T;
}
