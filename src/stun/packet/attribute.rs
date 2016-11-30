
use std::str::FromStr;
use std::string::ToString;
use std::net::SocketAddr;

use super::ErrorCode;

/**
Range:
    Required: 0x0000-0x7FFF, Options: 0x8000-0xFFFF
    0x0000-0x3FFF   IETF Review comprehension-required range
    0x4000-0x7FFF   Designated Expert   comprehension-required range
    0x8000-0xBFFF   IETF Review comprehension-optional range
    0xC000-0xFFFF   Designated Expert   comprehension-optional range

0x0000  Reserved    [RFC5389]
0x0001  MAPPED-ADDRESS  [RFC5389]
0x0002  Reserved; was RESPONSE-ADDRESS  [RFC5389]
0x0003  CHANGE-REQUEST  [RFC5780]
0x0004  Reserved; was SOURCE-ADDRESS    [RFC5389]
0x0005  Reserved; was CHANGED-ADDRESS   [RFC5389]
0x0006  USERNAME    [RFC5389]
0x0007  Reserved; was PASSWORD  [RFC5389]
0x0008  MESSAGE-INTEGRITY   [RFC5389]
0x0009  ERROR-CODE  [RFC5389]
0x000A  UNKNOWN-ATTRIBUTES  [RFC5389]
0x000B  Reserved; was REFLECTED-FROM    [RFC5389]
0x000C  CHANNEL-NUMBER  [RFC5766]
0x000D  LIFETIME    [RFC5766]
0x000E-0x000F   Reserved    
0x0010  Reserved (was BANDWIDTH)    [RFC5766]
0x0011  Reserved    
0x0012  XOR-PEER-ADDRESS    [RFC5766]
0x0013  DATA    [RFC5766]
0x0014  REALM   [RFC5389]
0x0015  NONCE   [RFC5389]
0x0016  XOR-RELAYED-ADDRESS [RFC5766]
0x0017  REQUESTED-ADDRESS-FAMILY    [RFC6156]
0x0018  EVEN-PORT   [RFC5766]
0x0019  REQUESTED-TRANSPORT [RFC5766]
0x001A  DONT-FRAGMENT   [RFC5766]
0x001B  ACCESS-TOKEN    [RFC7635]
0x001C-0x001F   Unassigned  
0x0020  XOR-MAPPED-ADDRESS  [RFC5389]
0x0021  Reserved (was TIMER-VAL)    [RFC5766]
0x0022  RESERVATION-TOKEN   [RFC5766]
0x0023  Reserved    
0x0024  PRIORITY    [RFC5245]
0x0025  USE-CANDIDATE   [RFC5245]
0x0026  PADDING [RFC5780]
0x0027  RESPONSE-PORT   [RFC5780]
0x0028-0x0029   Reserved    
0x002A  CONNECTION-ID   [RFC6062]
0x002B-0x002F   Unassigned  
0x0030  Reserved    
0x0031-0x7FFF   Unassigned  
0x8000-0x8021   Unassigned  
0x8022  SOFTWARE    [RFC5389]
0x8023  ALTERNATE-SERVER    [RFC5389]
0x8024  Reserved    
0x8025  TRANSACTION_TRANSMIT_COUNTER    [RFC7982]
0x8026  Reserved    
0x8027  CACHE-TIMEOUT   [RFC5780]
0x8028  FINGERPRINT [RFC5389]
0x8029  ICE-CONTROLLED  [RFC5245]
0x802A  ICE-CONTROLLING [RFC5245]
0x802B  RESPONSE-ORIGIN [RFC5780]
0x802C  OTHER-ADDRESS   [RFC5780]
0x802D  ECN-CHECK STUN  [RFC6679]
0x802E  THIRD-PARTY-AUTHORIZATION   [RFC7635]
0x802F  Unassigned  
0x8030  MOBILITY-TICKET [RFC8016]
0x8031-0xBFFF   Unassigned  
0xC000  CISCO-STUN-FLOWDATA [Dan_Wing]
0xC001  ENF-FLOW-DESCRIPTION    [Pål_Erik_Martinsen]
0xC002  ENF-NETWORK-STATUS  [Pål_Erik_Martinsen]
0xC003-0xFFFF   Unassigned  


0x0000  Reserved    [RFC5389]
0x0002  Reserved; was RESPONSE-ADDRESS  [RFC5389]
0x0004  Reserved; was SOURCE-ADDRESS    [RFC5389]
0x0005  Reserved; was CHANGED-ADDRESS   [RFC5389]
0x0007  Reserved; was PASSWORD  [RFC5389]
0x000B  Reserved; was REFLECTED-FROM    [RFC5389]
0x000E-0x000F   Reserved   
0x0010  Reserved (was BANDWIDTH)    [RFC5766]
0x0011  Reserved    
0x0021  Reserved (was TIMER-VAL)    [RFC5766]
0x0023  Reserved   
0x0028-0x0029   Reserved  
0x0030  Reserved      
0x8024  Reserved   
0x8026  Reserved  

0x001C-0x001F   Unassigned 
0x002B-0x002F   Unassigned 
0x0031-0x7FFF   Unassigned  
0x8000-0x8021   Unassigned   
0x802F  Unassigned   
0x8031-0xBFFF   Unassigned  
0xC003-0xFFFF   Unassigned  
**/

// Message Attribute Type
#[derive(Debug)]
pub enum AttributeType {
    MappedAddress,    // 0x0001  MAPPED-ADDRESS  [RFC5389]
    ResponseAddress,  // 0x0002  Reserved; was RESPONSE-ADDRESS  [RFC5389]
    ChangeRequest,    // 0x0003  CHANGE-REQUEST  [RFC5780]
    SourceAddress,    // 0x0004  Reserved; was SOURCE-ADDRESS    [RFC5389]
    ChangedAddress,   // 0x0005  Reserved; was CHANGED-ADDRESS   [RFC5389]
    UserName,         // 0x0006  USERNAME    [RFC5389]
    Password,         // 0x0007  Reserved; was PASSWORD  [RFC5389]
    MessageIntegrity, // 0x0008  MESSAGE-INTEGRITY   [RFC5389]
    ErrorCode,        // 0x0009  ERROR-CODE  [RFC5389]
    UnknownAttribute, // 0x000A  UNKNOWN-ATTRIBUTES  [RFC5389]
    ReflectedFrom,    // 0x000B  Reserved; was REFLECTED-FROM    [RFC5389]
    ChannelNumber,    // 0x000C  CHANNEL-NUMBER  [RFC5766]
    LifeTime,         // 0x000D  LIFETIME    [RFC5766]
    BandWidth,        // 0x0010  Reserved (was BANDWIDTH)    [RFC5766]
    XorPeerAddress,   // 0x0012  XOR-PEER-ADDRESS    [RFC5766]
    Data,             // 0x0013  DATA    [RFC5766]
    Realm,            // 0x0014  REALM   [RFC5389]
    Nonce,            // 0x0015  NONCE   [RFC5389]
    XorRelayedAddress,// 0x0016  XOR-RELAYED-ADDRESS [RFC5766]
    RequestAddressFamily, // 0x0017  REQUESTED-ADDRESS-FAMILY    [RFC6156]
    EvenPort,             // 0x0018  EVEN-PORT   [RFC5766]
    RequestedTransport,   // 0x0019  REQUESTED-TRANSPORT [RFC5766]
    DontFragment,         // 0x001A  DONT-FRAGMENT   [RFC5766]
    AccessToken,          // 0x001B  ACCESS-TOKEN    [RFC7635]
    XorMappedAddress,     // 0x0020  XOR-MAPPED-ADDRESS  [RFC5389]
    TimerVal,             // 0x0021  Reserved (was TIMER-VAL)    [RFC5766]
    ReservationToken,     // 0x0022  RESERVATION-TOKEN   [RFC5766]
    Priority,             // 0x0024  PRIORITY    [RFC5245]
    UseCandidate,         // 0x0025  USE-CANDIDATE   [RFC5245]
    Padding,              // 0x0026  PADDING [RFC5780]
    ResponsePort,         // 0x0027  RESPONSE-PORT   [RFC5780]
    ConnectionID,         // 0x002A  CONNECTION-ID   [RFC6062]
    Software,             // 0x8022  SOFTWARE    [RFC5389]
    AlternateServer,      // 0x8023  ALTERNATE-SERVER    [RFC5389]
    TransactionTransmitCounter, // 0x8025  TRANSACTION_TRANSMIT_COUNTER    [RFC7982]
    CacheTimeout,               // 0x8027  CACHE-TIMEOUT   [RFC5780]
    FingerPrint,                // 0x8028  FINGERPRINT [RFC5389]
    ICEControlled,              // 0x8029  ICE-CONTROLLED  [RFC5245]
    ICEControlling,             // 0x802A  ICE-CONTROLLING [RFC5245]
    ResponseOrigin,             // 0x802B  RESPONSE-ORIGIN [RFC5780]
    OtherAddress,               // 0x802C  OTHER-ADDRESS   [RFC5780]
    ECNCheckStun,               // 0x802D  ECN-CHECK STUN  [RFC6679]
    ThirdPartyAuthorization,    // 0x802E  THIRD-PARTY-AUTHORIZATION   [RFC7635]
    MobilityTicket,             // 0x8030  MOBILITY-TICKET [RFC8016]
    CiscoStunFlowData,          // 0xC000  CISCO-STUN-FLOWDATA [Dan_Wing]
    ENFFlowDescription,         // 0xC001  ENF-FLOW-DESCRIPTION    [Pål_Erik_Martinsen]
    ENFNetworkStatus            // 0xC002  ENF-NETWORK-STATUS  [Pål_Erik_Martinsen]
}

impl ToString for AttributeType {
    fn to_string (&self) -> String {
        match *self {
            AttributeType::MappedAddress => "MAPPED-ADDRESS".to_owned(),
            AttributeType::ResponseAddress => "RESPONSE-ADDRESS".to_owned(),
            AttributeType::ChangeRequest => "CHANGE-REQUEST".to_owned(),
            AttributeType::SourceAddress => "SOURCE-ADDRESS".to_owned(),
            AttributeType::ChangedAddress => "CHANGED-ADDRESS".to_owned(),
            AttributeType::UserName => "USERNAME".to_owned(),
            AttributeType::Password => "PASSWORD".to_owned(),
            AttributeType::MessageIntegrity => "MESSAGE-INTEGRITY".to_owned(),
            AttributeType::ErrorCode => "ERROR-CODE".to_owned(),
            AttributeType::UnknownAttribute => "UNKNOWN-ATTRIBUTES".to_owned(),
            AttributeType::ReflectedFrom => "REFLECTED-FROM".to_owned(),
            AttributeType::ChannelNumber => "CHANNEL-NUMBER".to_owned(),
            AttributeType::LifeTime => "LIFETIME".to_owned(),
            AttributeType::BandWidth => "BANDWIDTH".to_owned(),
            AttributeType::XorPeerAddress => "XOR-PEER-ADDRESS".to_owned(),
            AttributeType::Data => "DATA".to_owned(),
            AttributeType::Realm => "REALM".to_owned(),
            AttributeType::Nonce => "NONCE".to_owned(),
            AttributeType::XorRelayedAddress => "XOR-RELAYED-ADDRESS".to_owned(),
            AttributeType::RequestAddressFamily => "REQUESTED-ADDRESS-FAMILY".to_owned(),
            AttributeType::EvenPort => "EVEN-PORT".to_owned(),
            AttributeType::RequestedTransport => "REQUESTED-TRANSPORT".to_owned(),
            AttributeType::DontFragment => "DONT-FRAGMENT".to_owned(),
            AttributeType::AccessToken => "ACCESS-TOKEN".to_owned(),
            AttributeType::XorMappedAddress => "XOR-MAPPED-ADDRESS".to_owned(),
            AttributeType::TimerVal => "TIMER-VAL".to_owned(),
            AttributeType::ReservationToken => "RESERVATION-TOKEN".to_owned(),
            AttributeType::Priority => "PRIORITY".to_owned(),
            AttributeType::UseCandidate => "USE-CANDIDATE".to_owned(),
            AttributeType::Padding => "PADDING".to_owned(),
            AttributeType::ResponsePort => "RESPONSE-PORT".to_owned(),
            AttributeType::ConnectionID => "CONNECTION-ID".to_owned(),
            AttributeType::Software => "SOFTWARE".to_owned(),
            AttributeType::AlternateServer => "ALTERNATE-SERVER".to_owned(),
            AttributeType::TransactionTransmitCounter => "TRANSACTION_TRANSMIT_COUNTER".to_owned(),
            AttributeType::CacheTimeout => "CACHE-TIMEOUT".to_owned(),
            AttributeType::FingerPrint => "FINGERPRINT".to_owned(),
            AttributeType::ICEControlled => "ICE-CONTROLLED".to_owned(),
            AttributeType::ICEControlling => "ICE-CONTROLLING".to_owned(),
            AttributeType::ResponseOrigin => "RESPONSE-ORIGIN".to_owned(),
            AttributeType::OtherAddress => "OTHER-ADDRESS".to_owned(),
            AttributeType::ECNCheckStun => "ECN-CHECK STUN".to_owned(),
            AttributeType::ThirdPartyAuthorization => "THIRD-PARTY-AUTHORIZATION".to_owned(),
            AttributeType::MobilityTicket => "MOBILITY-TICKET".to_owned(),
            AttributeType::CiscoStunFlowData => "CISCO-STUN-FLOWDATA".to_owned(),
            AttributeType::ENFFlowDescription => "ENF-FLOW-DESCRIPTION".to_owned(),
            AttributeType::ENFNetworkStatus => "ENF-NETWORK-STATUS".to_owned()
        }
    }
}

impl AttributeType {
    pub fn from_u32 (n: u32) -> Result<Self, &'static str> {
        match n {
            0x0000
            | 0x000E ... 0x000F
            | 0x0011
            | 0x0023
            | 0x0028 ... 0x0029
            | 0x0030
            | 0x8024
            | 0x8026 => Err("Reserved"),
            0x001C ... 0x001F
            | 0x002B ... 0x002F
            | 0x0031 ... 0x7FFF
            | 0x8000 ... 0x8021
            | 0x802F
            | 0x8031 ... 0xBFFF
            | 0xC003 ... 0xFFFF => Err("Unassigned"),
            0x0001 => Ok(AttributeType::MappedAddress),
            0x0002 => Ok(AttributeType::ResponseAddress),
            0x0003 => Ok(AttributeType::ChangeRequest),
            0x0004 => Ok(AttributeType::SourceAddress),
            0x0005 => Ok(AttributeType::ChangedAddress),
            0x0006 => Ok(AttributeType::UserName),
            0x0007 => Ok(AttributeType::Password),
            0x0008 => Ok(AttributeType::MessageIntegrity),
            0x0009 => Ok(AttributeType::ErrorCode),
            0x000A => Ok(AttributeType::UnknownAttribute),
            0x000B => Ok(AttributeType::ReflectedFrom),
            0x000C => Ok(AttributeType::ChannelNumber),
            0x000D => Ok(AttributeType::LifeTime),
            0x0010 => Ok(AttributeType::BandWidth),
            0x0012 => Ok(AttributeType::XorPeerAddress),
            0x0013 => Ok(AttributeType::Data),
            0x0014 => Ok(AttributeType::Realm),
            0x0015 => Ok(AttributeType::Nonce),
            0x0016 => Ok(AttributeType::XorRelayedAddress),
            0x0017 => Ok(AttributeType::RequestAddressFamily),
            0x0018 => Ok(AttributeType::EvenPort),
            0x0019 => Ok(AttributeType::RequestedTransport),
            0x001A => Ok(AttributeType::DontFragment),
            0x001B => Ok(AttributeType::AccessToken),
            0x0020 => Ok(AttributeType::XorMappedAddress),
            0x0021 => Ok(AttributeType::TimerVal),
            0x0022 => Ok(AttributeType::ReservationToken),
            0x0024 => Ok(AttributeType::Priority),
            0x0025 => Ok(AttributeType::UseCandidate),
            0x0026 => Ok(AttributeType::Padding),
            0x0027 => Ok(AttributeType::ResponsePort),
            0x002A => Ok(AttributeType::ConnectionID),
            0x8022 => Ok(AttributeType::Software),
            0x8023 => Ok(AttributeType::AlternateServer),
            0x8025 => Ok(AttributeType::TransactionTransmitCounter),
            0x8027 => Ok(AttributeType::CacheTimeout),
            0x8028 => Ok(AttributeType::FingerPrint),
            0x8029 => Ok(AttributeType::ICEControlled),
            0x802A => Ok(AttributeType::ICEControlling),
            0x802B => Ok(AttributeType::ResponseOrigin),
            0x802C => Ok(AttributeType::OtherAddress),
            0x802D => Ok(AttributeType::ECNCheckStun),
            0x802E => Ok(AttributeType::ThirdPartyAuthorization),
            0x8030 => Ok(AttributeType::MobilityTicket),
            0xC000 => Ok(AttributeType::CiscoStunFlowData),
            0xC001 => Ok(AttributeType::ENFFlowDescription),
            0xC002 => Ok(AttributeType::ENFNetworkStatus),
            _      => Err("Range(0x0000 ... 0xFFFF and ).")
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let hex_str = bytes[..2].iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
        let number  = match u32::from_str_radix(hex_str.as_str(), 16){
            Ok(number) => number,
            Err(_) => return Err("Attribute Type parse error.")
        };
        AttributeType::from_u32(number)
    }
    pub fn to_u32(&self) -> u32 {
        match *self {
            AttributeType::MappedAddress    => 0x0001,
            AttributeType::ResponseAddress  => 0x0002,
            AttributeType::ChangeRequest    => 0x0003,
            AttributeType::SourceAddress    => 0x0004,
            AttributeType::ChangedAddress   => 0x0005,
            AttributeType::UserName         => 0x0006,
            AttributeType::Password         => 0x0007,
            AttributeType::MessageIntegrity => 0x0008,
            AttributeType::ErrorCode        => 0x0009,
            AttributeType::UnknownAttribute => 0x000A,
            AttributeType::ReflectedFrom    => 0x000B,
            AttributeType::ChannelNumber    => 0x000C,
            AttributeType::LifeTime         => 0x000D,
            AttributeType::BandWidth        => 0x0010,
            AttributeType::XorPeerAddress   => 0x0012,
            AttributeType::Data             => 0x0013,
            AttributeType::Realm            => 0x0014,
            AttributeType::Nonce            => 0x0015,
            AttributeType::XorRelayedAddress    => 0x0016,
            AttributeType::RequestAddressFamily => 0x0017,
            AttributeType::EvenPort             => 0x0018,
            AttributeType::RequestedTransport   => 0x0019,
            AttributeType::DontFragment         => 0x001A,
            AttributeType::AccessToken          => 0x001B,
            AttributeType::XorMappedAddress     => 0x0020,
            AttributeType::TimerVal             => 0x0021,
            AttributeType::ReservationToken     => 0x0022,
            AttributeType::Priority             => 0x0024,
            AttributeType::UseCandidate         => 0x0025,
            AttributeType::Padding              => 0x0026,
            AttributeType::ResponsePort         => 0x0027,
            AttributeType::ConnectionID         => 0x002A,
            AttributeType::Software             => 0x8022,
            AttributeType::AlternateServer      => 0x8023,
            AttributeType::TransactionTransmitCounter => 0x8025,
            AttributeType::CacheTimeout         => 0x8027,
            AttributeType::FingerPrint          => 0x8028,
            AttributeType::ICEControlled        => 0x8029,
            AttributeType::ICEControlling       => 0x802A,
            AttributeType::ResponseOrigin       => 0x802B,
            AttributeType::OtherAddress         => 0x802C,
            AttributeType::ECNCheckStun         => 0x802D,
            AttributeType::ThirdPartyAuthorization => 0x802E,
            AttributeType::MobilityTicket          => 0x8030,
            AttributeType::CiscoStunFlowData       => 0xC000,
            AttributeType::ENFFlowDescription      => 0xC001,
            AttributeType::ENFNetworkStatus        => 0xC002
        }
    }
}

#[derive(Debug)]
pub enum AttributeValue {
    MappedAddress(SocketAddr),
    XorMappedAddress(SocketAddr),
    ResponseAddress(SocketAddr),
    ResponseOrigin(SocketAddr),
    XorPeerAddress(SocketAddr),
    UserName,
    ErrorCode(ErrorCode),
    UnknownAttribute,
    ReflectedFrom,
}

impl AttributeValue {
    pub fn from_bytes(attr_type: AttributeType, bytes: &[u8]) -> Result<Self, &'static str>{
        unimplemented!();
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        match *self {
            AttributeValue::MappedAddress(ref socket_addr) => {
                let family = match *socket_addr {
                    SocketAddr::V4(_) => 0x01u8,
                    SocketAddr::V6(_) => 0x02u8
                };
                let port: u16 = socket_addr.port();
                let address = format!("{}", socket_addr.ip());
                
                let mut bytes: Vec<u8> = vec![0, family];
                let port_hex_string = format!("{:016b}", port);
                bytes.push(u8::from_str_radix(&port_hex_string[0.. 8], 10).unwrap());
                bytes.push(u8::from_str_radix(&port_hex_string[8..16], 10).unwrap());
                bytes.extend(address.into_bytes());
                bytes
            },
            AttributeValue::ErrorCode(ref error_code) => {
                let mut bytes:Vec<u8>  = vec![0, 0];
                let code   = error_code.to_u32();
                let class  = (code/100) as u8; // 3 bits
                let number = (code%100) as u8; // 8 bits
                bytes.push(u8::from_str_radix(format!("{:02x}", class).as_str(), 10).unwrap());
                bytes.push(u8::from_str_radix(format!("{:02x}", number).as_str(), 10).unwrap());
                bytes.extend(error_code.to_bytes());
                bytes
            },
            _ => unimplemented!()
        }
    }
}

#[derive(Debug)]
pub struct Attribute {
    type_ : AttributeType,  // 16 bits
    length: u32,            // 16 bits
    value : String          // 32 bits ( Or More. )
}

impl Attribute {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str>{
        unimplemented!();
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        unimplemented!();
    }
}