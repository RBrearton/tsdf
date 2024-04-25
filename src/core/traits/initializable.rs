pub(crate) trait Initializable {
    /// Returns whether the struct has been initialized.
    fn is_initialized(&self) -> bool;

    /// Initializes the struct.
    fn init(&mut self);

    /// Sets the initialized flag to the given value.
    fn set_initialization_state(&mut self, initialized: bool);
}
