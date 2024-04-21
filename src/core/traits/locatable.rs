use crate::core::structs::Addr;

/// A trait for objects that have a location in a file.
pub(crate) trait Locatable {
    fn get_addr(&self) -> &Addr;
}
