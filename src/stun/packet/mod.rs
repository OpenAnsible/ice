
use std::str::FromStr;
use std::string::ToString;
use std::convert::AsRef;

use std::io::{Read, Write};

mod header;
mod attribute;
mod address_family;

pub use self::header::{Header, Method, Class};
pub use self::attribute::{AttributeType, Attribute};
pub use self::address_family::AddressFamily;

#[derive(Debug)]
pub struct Packet {
    header    : Header,
    attributes: Vec<Attribute>
}

impl Packet {
    pub fn to_bytes(&self) -> &[u8] {
        unimplemented!();
    }
    pub fn to_hex_string(&self) -> String {
        unimplemented!();
    }
    pub fn from_bytes(&self) -> Result<Self, &'static str> {
        unimplemented!();
    }
}