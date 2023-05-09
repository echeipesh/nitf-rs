//! Crate for reading and manipulating NITF files
//! 

// TODO: 
// Need to implement reading optional segments 
// Meaning: after the main header, each sub-segment starts with 2 characters. 
// Need to implement reading these in some arbitrary order, storing starting bytes, offsets, etc.



use std::fs::File;
use std::path::Path;
use std::string::FromUtf8Error;

pub mod nitf_2_1;

use nitf_2_1::Nitf;


pub fn read_nitf(path: Option<&Path>) -> Result<Nitf, FromUtf8Error> {
    match path {
        Some(path) => {
            let mut reader = File::open(path).unwrap();
            return Nitf::from_file(&mut reader)
        }
        None => return Ok(Nitf::default())
    }
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
       
    #[test]
    fn test_read_nitf() {
        let res = read_nitf(None);
        let pass: bool;
        match res {
            Ok(_) => pass = true,
            Err(_) => pass = false
        }
        assert!(pass == true)
    }
}
