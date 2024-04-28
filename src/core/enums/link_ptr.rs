use crate::core::{
    structs::Addr,
    traits::{FileSerializable, FixedSizeOnDisk},
};

/// The main link pointer enum, which can either be an address or a null
/// pointer.
/// The integer address isn't a memory address, but an offset within the file.
/// The value 0 would be the first byte in the file, 1 would be the second byte,
/// and so on. We can use this to uniquely locate any byte in the file.
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum LinkPtr {
    Addr(Addr),
    Null(Addr),
}

impl LinkPtr {
    /// A private method that represents a LinkPtr as an Addr, making an Addr
    /// from 0 if the LinkPtr is Null.
    fn to_addr(&self) -> &Addr {
        match self {
            LinkPtr::Addr(addr) => &addr,
            LinkPtr::Null(addr) => &addr,
        }
    }
}

impl FixedSizeOnDisk for LinkPtr {
    fn get_bin_size_on_disk() -> u64 {
        Addr::get_bin_size_on_disk()
    }

    fn get_json_size_on_disk() -> u64 {
        Addr::get_json_size_on_disk()
    }
}

impl FileSerializable for LinkPtr {
    fn null() -> Self {
        LinkPtr::Null(Addr::null())
    }

    fn to_bin(&self) -> Vec<u8> {
        self.to_addr().to_bin()
    }

    fn to_json(&self) -> String {
        self.to_addr().to_json()
    }

    fn from_bin(bytes: &[u8]) -> Self {
        let addr = Addr::from_bin(bytes);

        // Now return the appropriate LinkPtr depending on what we found.
        if addr.get_loc() == 0 {
            LinkPtr::Null(addr)
        } else {
            LinkPtr::Addr(addr)
        }
    }

    fn from_json(json: String) -> Self {
        let addr = Addr::from_json(json);

        // Now return the appropriate LinkPtr depending on what we found.
        if addr.get_loc() == 0 {
            LinkPtr::Null(addr)
        } else {
            LinkPtr::Addr(addr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that we can convert a LinkPtr to bytes and back.
    #[test]
    fn test_link_ptr_to_bin() {
        let addr = Addr::new(123);
        let link_ptr = LinkPtr::Addr(addr);
        let bytes = link_ptr.to_bin();
        let link_ptr2 = LinkPtr::from_bin(&bytes);
        assert_eq!(link_ptr.to_addr().get_loc(), link_ptr2.to_addr().get_loc());

        let null_link_ptr = LinkPtr::Null(Addr::new(0));
        let bytes = null_link_ptr.to_bin();
        let null_link_ptr2 = LinkPtr::from_bin(&bytes);
        assert_eq!(
            null_link_ptr.to_addr().get_loc(),
            null_link_ptr2.to_addr().get_loc()
        );
    }

    /// Test that we can convert a LinkPtr to json and back.
    #[test]
    fn test_link_ptr_to_json() {
        let addr = Addr::new(123);
        let link_ptr = LinkPtr::Addr(addr);
        let json = link_ptr.to_json();
        let link_ptr2 = LinkPtr::from_json(json);
        assert_eq!(link_ptr.to_addr().get_loc(), link_ptr2.to_addr().get_loc());

        let null_link_ptr = LinkPtr::Null(Addr::new(0));
        let json = null_link_ptr.to_json();
        let null_link_ptr2 = LinkPtr::from_json(json);
        assert_eq!(
            null_link_ptr.to_addr().get_loc(),
            null_link_ptr2.to_addr().get_loc()
        );
    }

    /// Make sure that the size of a LinkPtr is the same as the size of an Addr.
    #[test]
    fn test_link_ptr_size() {
        assert_eq!(
            LinkPtr::get_bin_size_on_disk(),
            Addr::get_bin_size_on_disk()
        );
    }

    /// Make sure that the json string of a LinkPtr is exactly what we expect.
    #[test]
    fn test_link_ptr_json() {
        let addr = Addr::new(123);
        let link_ptr = LinkPtr::Addr(addr);
        let json = link_ptr.to_json();

        // Note that we need to get the padding right because the json string
        // is fixed width, and the maximum value of the address is 2^64 - 1,
        // which is 20 characters long.
        assert_eq!(json, "{\"loc\":123}                 ");

        let null_link_ptr = LinkPtr::Null(Addr::new(0));
        let json = null_link_ptr.to_json();
        assert_eq!(json, "{\"loc\":0}                   ");
    }
}
