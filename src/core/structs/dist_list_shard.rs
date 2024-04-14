use super::Addr;
use crate::core::enums::LinkPtr;

pub(crate) struct DistListShard {
    next: LinkPtr,
    vals: Vec<Addr>,
}
