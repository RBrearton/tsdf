/// Enum for the different IO modes.
pub enum IoMode {
    /// Read the file. This never places a lock on the file.
    Read,

    /// Write to the file without locking it. Using this mode removes some functionality, such as
    /// data deletion. Otherwise, data can be written to the file as normal.
    LocklessWrite,

    /// Write to the file with a lock. This mode allows for all functionality, including data
    /// deletion, at the cost of not allowing readers to concurrently access the file.
    LockingWrite,
}
