use std::ops::Add;

use super::Addr;
use crate::core::enums::LinkPtr;
use crate::core::traits::DistListShardTrait;
use crate::core::traits::FileSerializable;
use crate::core::traits::Link;

pub(crate) struct DistListShard {
    next: LinkPtr,
    vals: Vec<Addr>,
    link_number: i32,
}

impl Link for DistListShard {
    fn get_next(&self) -> &LinkPtr {
        &self.next
    }

    fn get_link_number(&self) -> i32 {
        self.link_number
    }
}

// impl DistListShardTrait<Addr> for DistListShard {
//     fn add(&self, elem: Addr) {
//         self.vals.push(elem);
//     }

//     fn remove(&self, elem: Addr) {
//         self.vals.retain(|&x| x != elem);
//     }

//     fn get(&self, index: i32) -> Addr {
//         self.vals[index as usize]
//     }
// }
