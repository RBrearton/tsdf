// Declare the project structure.
pub(crate) mod core;

use crate::core::enums::LinkPtr;
use crate::core::structs::{addr, Addr};

fn main() {
    // Make a new LinkPtr.
    let addr_loc = 10;
    let test_addr = Addr::new(addr_loc);
    let link_ptr = LinkPtr::Addr(test_addr);

    // Print the LinkPtr.
    match link_ptr {
        LinkPtr::Addr(addr) => println!("Address: {}", addr_loc),
        LinkPtr::Null => println!("Null pointer"),
    }

    println!("Hello, world!");
}
