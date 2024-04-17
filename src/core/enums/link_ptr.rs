use crate::core::structs::Addr;

/// The main link pointer enum, which can either be an address or a null
/// pointer.
/// The integer address isn't a memory address, but an offset within the file.
/// The value 0 would be the first byte in the file, 1 would be the second byte,
/// and so on. We can use this to uniquely locate any byte in the file.
pub(crate) enum LinkPtr {
    Addr(Addr),
    Null,
}
