use crate::core::enums::LinkPtr;

pub(crate) struct DistDictShard<TKey, TVal> {
    next: LinkPtr,
    keys: Vec<TKey>,
    vals: Vec<TVal>,
}
