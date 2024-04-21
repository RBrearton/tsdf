use crate::core::traits::FileSerializable;

/// The Addr struct is a simple struct that holds an integer location. This
/// location is an offset within the file, and can be used to uniquely locate
/// any byte in the file.
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Addr {
    loc: u64,
}

impl Addr {
    pub(crate) fn new(loc: u64) -> Self {
        Self { loc }
    }

    pub(crate) fn get_loc(&self) -> u64 {
        self.loc
    }
}

/// The value that we use to represent a null Addr.
const NULL_LOC: u64 = 0;

impl FileSerializable for Addr {
    fn null() -> Self {
        Self { loc: NULL_LOC }
    }

    fn to_bin(&self) -> Vec<u8> {
        let bytes = self.loc.to_le_bytes().to_vec();
        bytes
    }

    fn from_bin(bytes: &[u8]) -> Self {
        let loc = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        Self::new(loc)
    }

    fn get_bin_size_on_disk() -> u64 {
        std::mem::size_of::<u64>() as u64
    }

    fn get_json_size_on_disk() -> u64 {
        // To get the size of the json string, we make the largest possible
        // Addr, convert it to json, and get the length of the json string. This
        // is far from optimized, but the whole point of the json representation
        // is to be debug-friendly, not performant.
        let addr = Addr { loc: u64::MAX };
        let json = addr.to_json();
        json.len() as u64
    }
}

// A simple test to make sure that we can convert an Addr to bytes/json and
/// back.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test that we can convert an Addr to bytes and back.
    #[test]
    fn test_addr_to_bytes() {
        let addr = Addr::new(123);
        let bytes = addr.to_bin();
        let addr2 = Addr::from_bin(&bytes);
        assert_eq!(addr.get_loc(), addr2.get_loc());
    }

    /// Test that we can convert an Addr to json and back.
    #[test]
    fn test_addr_to_json() {
        let addr = Addr::new(123);
        let json = addr.to_json();
        let addr2 = Addr::from_json(json);
        assert_eq!(addr.get_loc(), addr2.get_loc());
    }
}
