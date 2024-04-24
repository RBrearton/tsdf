use crate::core::enums::LinkPtr;

use super::Locatable;

pub(crate) trait Link: Locatable {
    /// Returns a reference to the next link.
    fn get_next(&self) -> LinkPtr;

    /// Returns the link number of this link.
    fn get_link_number(&self) -> i32;
}
