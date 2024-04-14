use crate::core::structs::Addr;

pub(crate) trait FileSerializable {
    /// Writes the object to the file at the given location.
    fn write(&self, addr: Addr);

    /// Reads the object from the file at the given location.
    fn from_addr(addr: Addr) -> Self;
}
