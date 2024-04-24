use std::fs::File;

use super::Addr;
use super::IoMetadata;

use crate::core::enums::LinkPtr;
use crate::core::traits::Link;
use crate::core::traits::Locatable;

pub(crate) struct DistListShard<'a, 'b> {
    link_number: i32,
    loc: Addr,
    file: &'a File,
    metadata: &'b IoMetadata,
}

impl Locatable for DistListShard<'_, '_> {
    fn get_addr(&self) -> &Addr {
        &self.loc
    }

    fn get_file(&self) -> &File {
        self.file
    }

    fn get_io_metadata(&self) -> &IoMetadata {
        self.metadata
    }
}
