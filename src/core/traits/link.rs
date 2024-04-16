use crate::core::enums::LinkPtr;

pub(crate) trait Link {
    fn get_next(&self) -> &LinkPtr;
    fn get_link_number(&self) -> i32;
}
