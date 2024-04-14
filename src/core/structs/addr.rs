/// The Addr struct is a simple struct that holds an integer location. This location is
/// an offset within the file, and can be used to uniquely locate any byte in the file.
pub(crate) struct Addr {
    loc: u64,
}
