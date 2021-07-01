#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct Attribute {
    pub name: String,
    pub str_value: String,
}

pub struct Node {
    pub name: String,
    body: String,
    attributes: Vec<Attribute>,
}

impl Node {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
