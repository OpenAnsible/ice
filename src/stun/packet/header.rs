
use std::str::FromStr;
use std::string::ToString;

use super::super::constant::STUN_MAGIC_COOKIE;

/// Message Class
#[derive(Debug, Clone)]
pub enum Class {
    Request,
    Indication,
    SuccessResponse,
    FailureResponse
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match *self {
            Class::Request            => "Request".to_owned(),
            Class::Indication         => "Indication".to_owned(),
            Class::SuccessResponse    => "Success Response".to_owned(),
            Class::FailureResponse    => "Failure Response".to_owned()
        }
    }
}

impl Class {
    pub fn from_u32(n: u32) -> Result<Self, &'static str> {
        match n {
            0b00 => Ok(Class::Request),
            0b01 => Ok(Class::Indication),
            0b10 => Ok(Class::SuccessResponse),
            0b11 => Ok(Class::FailureResponse),
            _    => Err("range(0b00 ... 0b11)")
        }
    }
    pub fn to_u32(&self) -> u32 {
        match *self {
            Class::Request            => 0b00,
            Class::Indication         => 0b01,
            Class::SuccessResponse    => 0b10,
            Class::FailureResponse    => 0b11
        }
    }
}

/**
Range: 
    0x000-0x07F IETF Review
    0x080-0x0FF Designated Expert

0x000       Reserved    [RFC5389]
0x001       Binding [RFC5389]
0x002       Reserved; was SharedSecret  [RFC5389]
0x003       Allocate    [RFC5766]
0x004       Refresh [RFC5766]
0x005       Unassigned  
0x006       Send    [RFC5766]
0x007       Data    [RFC5766]
0x008       CreatePermission    [RFC5766]
0x009       ChannelBind [RFC5766]
0x00A       Connect [RFC6062]
0x00B       ConnectionBind  [RFC6062]
0x00C       ConnectionAttempt   [RFC6062]
0x00D-0x0FF Unassigned  
0x100-0xFFF Reserved (For DTLS-SRTP multiplexing collision avoidance, see [RFC7983]. Cannot be made available for assignment without IETF Review.)  [RFC7983]

**/
/// Message Method
#[derive(Debug, Clone)]
pub enum Method {
    Binding,
    SharedSecret,
    Allocate,
    Refresh,
    Send,
    Data,
    CreatePermission,
    ChannelBind,
    Connect,
    ConnectionBind,
    ConnectionAttempt
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match *self {
            Method::Binding       => "Binding".to_owned(),
            Method::SharedSecret  => "SharedSecret".to_owned(),
            Method::Allocate      => "Allocate".to_owned(),
            Method::Refresh       => "Refresh".to_owned(),
            Method::Send          => "Send".to_owned(),
            Method::Data          => "Data".to_owned(),
            Method::CreatePermission  => "CreatePermission".to_owned(),
            Method::ChannelBind       => "ChannelBind".to_owned(),
            Method::Connect           => "Connect".to_owned(),
            Method::ConnectionBind    => "ConnectionBind".to_owned(),
            Method::ConnectionAttempt => "ConnectionAttempt".to_owned()
        }
    }
}

impl Method {
    pub fn from_u32(n: u32) -> Result<Self, &'static str> {
        match n {
            0x000                   => Err("Reserved"),
            0x100 ... 0xFFF         => Err("Reserved(For DTLS-SRTP multiplexing collision avoidance, \
                                            see [RFC7983]. Cannot be made available for assignment \
                                            without IETF Review.)"),
            0x005 | 0x00D ... 0x0FF => Err("Unassigned"),
            
            0x001 => Ok(Method::Binding),
            0x002 => Ok(Method::SharedSecret),
            0x003 => Ok(Method::Allocate),
            0x004 => Ok(Method::Refresh),
            0x006 => Ok(Method::Send),
            0x007 => Ok(Method::Data),
            0x008 => Ok(Method::CreatePermission),
            0x009 => Ok(Method::ChannelBind),
            0x00A => Ok(Method::Connect),
            0x00B => Ok(Method::ConnectionBind),
            0x00C => Ok(Method::ConnectionAttempt),
            _     => Err("Range(0x000 ... 0xFFF)")
        }
    }
    pub fn to_u32(&self) -> u32 {
        match *self {
            Method::Binding      => 0x001,
            Method::SharedSecret => 0x002,
            Method::Allocate     => 0x003,
            Method::Refresh      => 0x004,
            Method::Send         => 0x006,
            Method::Data         => 0x007,
            Method::CreatePermission  => 0x008,
            Method::ChannelBind       => 0x009,
            Method::Connect           => 0x00A,
            Method::ConnectionBind    => 0x00B,
            Method::ConnectionAttempt => 0x00C
        }
    }
    pub fn to_hex(&self) -> String {
        format!("{:#05X}", self.to_u32())   
    }
}

/**
0                   1                   2                   3
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|0 0|     STUN Message Type     |         Message Length        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                         Magic Cookie                          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|                     Transaction ID (96 bits)                  |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

          Figure 2: Format of STUN Message Header

Head Struct
    [0, 1, 0, 0, 33, 18, 164, 66, 70, 149, 250, 122, 253, 177, 191, 174, 164, 118, 181, 61]
    Magic Code     : 2  bits
    Message Type   : 14 bits ( class: 2 bits; method: 12 bits; )
    Message Length : 16 bits (  2 Bytes )
    Magic Cookie   : 32 bits (  4 Bytes ) 
    Transaction ID : 96 bits ( 12 Bytes )

**/
#[derive(Debug, Clone)]
pub struct Header {
    magic_code    : u8,     //  2 bits
    class         : Class,  //  2 bits
    method        : Method, // 12 bits
    length        : u16,    // 16 bits message attribute length
    magic_cookie  : u32,    // 32 bits (Must Be 0x2112A442 (554869826))
    transaction_id: String  // 96 bits unique
}

pub fn bytes_to_hex_str(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("")
}

impl Header {
    pub fn from_bytes (bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 20 {
            return Err("header size must be 20 Bytes.");
        }
        let bytes = &bytes[..20];
        // https://tools.ietf.org/html/rfc5389#appendix-A
        // 0b 00 01 000000000000
        let bits = format!("{:08b}", bytes[0]) + format!("{:08b}", bytes[1]).as_ref();
        let magic_code     = match u8::from_str_radix(&bits[0..2], 2) {
            Ok(magic_code) => magic_code,
            Err(_)         => return Err("magic code parse error")
        };
        let message_class  = match u8::from_str_radix(&bits[2..4], 2) {
            Ok(message_class) => match Class::from_u32(message_class as u32) {
                Ok(class) => class,
                Err(e)    => return Err(e)
            },
            Err(_)        => return Err("message class error.")
        };
        let message_method  = match u16::from_str_radix(&bits[4..16], 2) {
            Ok(message_method) => match Method::from_u32(message_method as u32) {
                Ok(method) => method,
                Err(e)     => return Err(e)
            },
            Err(_)         => return Err("message class error.")
        };

        let message_length = match u16::from_str_radix(bytes_to_hex_str(&bytes[2..4]).as_ref(), 16) {
            Ok(message_length) => message_length,
            Err(_)             => return Err("message length error")
        };
        let magic_cookie   = match u32::from_str_radix(bytes_to_hex_str(&bytes[4..8]).as_ref(), 16) {
            Ok(magic_cookie) => magic_cookie,
            Err(_)           => return Err("magic cookie error")
        };

        let transaction_id = if magic_cookie != STUN_MAGIC_COOKIE {
            bytes_to_hex_str(&bytes[4..20])
        } else {
            bytes_to_hex_str(&bytes[8..20])
        };
        
        Ok(Header{
            magic_code    : magic_code,
            class         : message_class,
            method        : message_method,
            length        : message_length,
            magic_cookie  : magic_cookie,
            transaction_id: transaction_id
        })
    }
    pub fn set_length(&mut self, length: u16) {
        self.length = length;
    }
    pub fn set_class(&mut self, class: Class){
        self.class = class;
    }
    pub fn set_method(&mut self, method: Method){
        self.method = method;
    }
    pub fn set_transaction_id(&mut self, transaction_id: String) {
        self.transaction_id = transaction_id;
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        assert_eq!(self.magic_code, 0u8);

        let bits = format!("00{:02b}", self.class.to_u32() as u8 )
                 + format!("{:012b}", self.method.to_u32() as u8).as_ref();

        bytes.push(u8::from_str_radix(&bits[0.. 8], 2).unwrap());
        bytes.push(u8::from_str_radix(&bits[8..16], 2).unwrap());
        
        let length_bits = format!("{:016b}", self.length);
        bytes.push(u8::from_str_radix(&length_bits[0..8], 2).unwrap());
        bytes.push(u8::from_str_radix(&length_bits[8..16], 2).unwrap());
        if self.magic_cookie == STUN_MAGIC_COOKIE {
            let mc_bits = format!("{:032b}", self.magic_code);
            bytes.push(u8::from_str_radix(&mc_bits[ 0.. 8], 2).unwrap());
            bytes.push(u8::from_str_radix(&mc_bits[ 8..16], 2).unwrap());
            bytes.push(u8::from_str_radix(&mc_bits[16..24], 2).unwrap());
            bytes.push(u8::from_str_radix(&mc_bits[24..32], 2).unwrap());
        }
        bytes.extend(self.transaction_id.clone().into_bytes());
        bytes
    }
}
