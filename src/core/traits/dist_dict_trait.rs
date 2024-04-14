use super::FileSerializable;

/// A distributed dictionary is a key-value store that is distributed across multiple shards.
/// Anything that implements this pub(crate) trait can behave as a distributed dictionary.
pub(crate) trait DistDictTrait<TKey: FileSerializable, TVal: FileSerializable> {
    /// Adds a key-value pair to the dictionary.
    fn add(&self, key: TKey, val: TVal);

    /// Removes a key-value pair from the dictionary.
    fn remove(&self, key: TKey);

    /// Gets the value associated with the given key.
    fn get(&self, key: TKey) -> TVal;
}
