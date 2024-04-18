use std::fs::File;

use super::Addr;
use super::IoMetadata;

use crate::core::enums::LinkPtr;
use crate::core::traits::Link;
use crate::core::traits::Locatable;

pub(crate) struct DistListShard<'a, 'b> {
    next: LinkPtr,
    vals: Vec<Addr>,
    link_number: i32,
    loc: Addr,
    file: &'a File,
    metadata: &'b IoMetadata,
}

impl Locatable for DistListShard<'_, '_> {
    fn get_loc(&self) -> &Addr {
        &self.loc
    }
}

impl Link for DistListShard<'_, '_> {
    fn get_next(&self) -> &LinkPtr {
        &self.next
    }

    fn get_link_number(&self) -> i32 {
        self.link_number
    }

    fn get_file(&self) -> &File {
        self.file
    }

    fn get_io_metadata(&self) -> &IoMetadata {
        self.metadata
    }
}
