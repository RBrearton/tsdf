/// The HasName pub(crate) trait is implemented by any object that has a name.
pub(crate) trait HasName {
    fn name(&self) -> String;
}
