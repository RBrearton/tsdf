// Declare the project structure.
pub(crate) mod core;

use crate::core::enums::LinkPtr;

fn main() {
    // Make a new LinkPtr.
    let link_ptr = LinkPtr::Addr(20);

    // Print the LinkPtr.
    match link_ptr {
        LinkPtr::Addr(addr) => println!("Address: {}", addr),
        LinkPtr::Null => println!("Null pointer"),
    }

    println!("Hello, world!");
}
