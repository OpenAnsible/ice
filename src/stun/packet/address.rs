
use std::str::FromStr;
use std::string::ToString;

/**
The address family can take on the following values:
    
    0x01:IPv4
    0x02:IPv6
**/
#[derive(Debug)]
pub enum Family {
    Ipv4,
    Ipv6
}

impl ToString for Family {
    fn to_string(&self) -> String {
        match *self {
            Family::Ipv4 => "IPv4".to_owned(),
            Family::Ipv6 => "IPv6".to_owned()
        }
    }
}

impl Family {
    pub fn from_u32(n: u32) -> Result<Self, &'static str> {
        match n {
            0x01 => Ok(Family::Ipv4),
            0x02 => Ok(Family::Ipv6),
            _    => Err("Address Family Error")
        }
    }
    pub fn to_u32(&self) -> u32 {
        match *self {
            Family::Ipv4 => 0x01,
            Family::Ipv6 => 0x02
        }
    }
}

#[derive(Debug)]
pub struct Address {
    family : Family,
    port   : u16,
    address: String
}

impl Address {
    pub fn new (family: Family, port: u16, address: String) {

    }
}
