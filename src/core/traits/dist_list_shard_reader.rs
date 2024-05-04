use std::os::unix::fs::FileExt;

use crate::core::structs::Addr;

use super::{
    FileSerializable, FixedSizeOnDisk, ShardTrait, VariableSizeOnDisk,
};

/// This trait defines everything we need to be able to do to read from a
/// shard of a distributed list in a file.
/// # Structure on disk
/// The default implementation of this trait gives the shard the following
/// structure on disk:
/// | is_next_written (1 byte) | next (8 bytes) |
/// | is_element_written_0 (1 byte) | ... | is_element_written_n (1 byte) |
/// | element_0 (var bytes) | ... | element_n (var bytes) |
/// The reason for storing all the is_element_written booleans together is
/// to optimize the process of searching for the first unwritten element in
/// the shard.
pub(crate) trait DistListShardReader<T: FileSerializable>:
    VariableSizeOnDisk + ShardTrait<T>
{
    /// Returns the nth element in the shard.
    fn get_element(&self, n: usize) -> T {
        let addr = self.get_element_addr(n);
        T::from_addr(addr, self.get_file(), self.get_io_metadata())
    }

    /// Returns the nth is_element_written boolean in the shard.
    fn is_element_written(&self, n: usize) -> bool {
        let addr = self.get_is_element_written_addr(n);
        let mut buf = [0];
        self.get_file().read_at(&mut buf, addr.get_loc()).unwrap();

        buf[0] == 1
    }

    /// Returns the address of the nth element in the shard.
    fn get_element_addr(&self, n: usize) -> Addr {
        // The location of the nth element is the location of the final
        // is_element_written boolean in the shard plus the size of the
        // is_element_written boolean, plus the size of each element up to
        // the nth element.
        // The way that we get around adding the size of a bool to the location
        // of the last is_written boolean is by requesting the location of the
        // the final element + 1. Note that the final element's number is
        // self.get_capacity() - 1.
        let first_element_loc = self
            .get_is_element_written_addr(self.get_capacity())
            .get_loc();
        let size_of_element = T::get_size_on_disk(self.get_io_metadata());
        let loc = first_element_loc + size_of_element * n as u64;

        Addr::new(loc)
    }

    /// Returns the address of the nth is_element_written boolean in the shard.
    fn get_is_element_written_addr(&self, n: usize) -> Addr {
        // The shard starts with the is_next_written boolean, and then the
        // next pointer. After that, we have the is_element_written booleans.
        let size_of_addr = Addr::get_size_on_disk(self.get_io_metadata());
        let size_of_bool = 1;
        let start_of_is_element_written =
            self.get_addr().get_loc() + size_of_bool + size_of_addr;

        // Then, all the is_element_written booleans come one after another.
        let loc = start_of_is_element_written + size_of_bool * n as u64;

        Addr::new(loc)
    }

    /// Returns whether the shard is full.
    fn is_full(&self) -> bool {
        // To find out if the shard is full, we check the final
        // is_element_written boolean in the shard.
        self.is_element_written(self.get_capacity() - 1)
    }
}
