use crate::core::enums::LinkPtr;

pub(crate) trait FileSerializable {
    /// Writes the object to the file at the given location.
    fn write(&self, loc: LinkPtr);

    /// Reads the object from the file at the given location.
    fn read(loc: LinkPtr) -> Self;
}
