use crate::core::enums::LinkPtr;

use super::Locatable;

pub(crate) trait Link: Locatable {
    fn get_next(&self) -> &LinkPtr;
    fn get_link_number(&self) -> i32;
}
