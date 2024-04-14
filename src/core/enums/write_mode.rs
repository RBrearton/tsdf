/// Enum to specify the write mode of a file. Please note that, for all production use cases, the
/// write mode should be set to `Binary`. The `Text` mode is only for debugging and development.
pub enum WriteMode {
    Binary,
    Text,
}
