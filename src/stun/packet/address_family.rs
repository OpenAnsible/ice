
use std::str::FromStr;
use std::string::ToString;

/**
The address family can take on the following values:
    
    0x01:IPv4
    0x02:IPv6
**/
#[derive(Debug)]
pub enum AddressFamily {
    None,
    Ipv4,
    Ipv6
}

impl ToString for AddressFamily {
    fn to_string(&self) -> String {
        match *self {
            AddressFamily::None => "Unknow".to_owned(),
            AddressFamily::Ipv4 => "IPv4".to_owned(),
            AddressFamily::Ipv6 => "IPv6".to_owned()
        }
    }
}

impl AddressFamily {
    pub fn from_u32(n: u32) -> Result<Self, &'static str> {
        match n {
            0x00 => Ok(AddressFamily::None),
            0x01 => Ok(AddressFamily::Ipv4),
            0x02 => Ok(AddressFamily::Ipv6),
            _    => Err("Address Family Error")
        }
    }
    pub fn to_u32(&self) -> u32 {
        match *self {
            AddressFamily::None => 0x00,
            AddressFamily::Ipv4 => 0x01,
            AddressFamily::Ipv6 => 0x02
        }
    }
}