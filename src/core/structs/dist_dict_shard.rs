use crate::core::enums::LinkPtr;

pub(crate) struct DistDictShard<TKey, TVal> {
    next: LinkPtr,
    link_number: Option<i32>,
    keys: Vec<TKey>,
    vals: Vec<TVal>,
}
