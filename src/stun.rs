#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_must_use, unreachable_code, non_snake_case, non_camel_case_types)]

use std::{thread, time, mem};

use std::str::FromStr;
use std::string::ToString;
use std::convert::AsRef;
use std::io::{Read, Write};

use std::net::{ SocketAddr, IpAddr, TcpListener, TcpStream, UdpSocket, Shutdown };
// use std::collections::btree_map::BTreeMap;

pub const PUBLIC_STUN_SERVERS: [&'static str; 11] = [
    "stun:stun.xten.net:3478",
    "stun:sip.iptel.org:3478",
    "stun:tesla.divmod.net:3478",
    "stun:erlang.divmod.net:3478",
    "stun:stun.wirlab.net:3478",
    "stun:stun2.wirlab.net:3478",
    "stun:stun1.vovida.org:3478",
    "stun:stun1.l.google.com:19302",
    "stun:stun2.l.google.com:19302",
    "stun:stun3.l.google.com:19302",
    "stun:stun4.l.google.com:19302"
];

// https://tools.ietf.org/html/rfc5389#section-18.4
//      stun   3478/(tcp, udp)
//      stuns  5349/(tcp, udp)
pub const STUN_PORT : u16 = 3478;
pub const STUNS_PORT: u16 = 5349;
pub const STUN_FINGERPRINT_XOR_VALUE: u32 = 0x5354554E; // STUN FINGERPRINT XOR Value



/**

Refs:
    https://tools.ietf.org/html/rfc5389#page-20
    https://tools.ietf.org/html/rfc5389#section-18.3
    http://www.iana.org/assignments/stun-parameters/stun-parameters.xhtml#stun-parameters-6

Error Code:
    range 0 - 699

0-299   Reserved
301-399 Unassigned
402     Unassigned
406-419 Unassigned  
404     Unassigned
421-430 Unassigned
432-436 Unassigned
439     Unassigned
444-445 Unassigned 
448-485 Unassigned 
488-499 Unassigned  
501-507 Unassigned  
509-599 Unassigned 
601-699 Unassigned

300     Try Alternate   [RFC5389]  
400     Bad Request [RFC5389]
401     Unauthorized    [RFC5389]
403     Forbidden   [RFC5766]
405     Mobility Forbidden  [RFC8016]
420     Unknown Attribute   [RFC5389]
437     Allocation Mismatch [RFC5766]
438     Stale Nonce [RFC5389]
440     Address Family not Supported    [RFC6156]
441     Wrong Credentials   [RFC5766]
442     Unsupported Transport Protocol  [RFC5766]
443     Peer Address Family Mismatch    [RFC6156]
446     Connection Already Exists   [RFC6062]
447     Connection Timeout or Failure   [RFC6062]
486     Allocation Quota Reached    [RFC5766]
487     Role Conflict   [RFC5245]
500     Server Error    [RFC5389]
508     Insufficient Capacity   [RFC5766]
**/
#[derive(Debug)]
pub enum Error {
    TryAlternate,  // 300
    BadRequest,    // 400
    Unauthorized,  // 401
    Forbidden,     // 403
    MobilityForbidden,     // 405
    UnknownAttribute,      // 420
    IntegrityCheckFailure, // 431 (IANA 遗漏定义: https://www.ietf.org/rfc/rfc3489.txt)
    AllocationMismatch,    // 437
    StaleNonce,            // 438
    AddressFamilyNotSupported,    // 440
    WrongCredentials,             // 441
    UnsupportedTransportProtocol, // 442
    PeerAddressFamilyMismatch,    // 443
    ConnectionAlreadyExists,      // 446
    ConnectionTimeoutOrFailure,   // 447
    AllocationQuotaReached,       // 486
    RoleConflict,                 // 487
    ServerError,                  // 500
    InsufficientCapacity,         // 508
    GlobalFailure                 // 600 (IANA 遗漏定义: https://www.ietf.org/rfc/rfc3489.txt)
}

impl ToString for Error {
    fn to_string (&self) -> String {
        match *self {
            Error::TryAlternate                 => "Try Alternate".to_owned(),
            Error::BadRequest                   => "Bad request".to_owned(),
            Error::Unauthorized                 => "Unauthorized".to_owned(),
            Error::Forbidden                    => "Forbidden".to_owned(),
            Error::MobilityForbidden            => "Mobility forbidden".to_owned(),
            Error::UnknownAttribute             => "Unknown attribute(s)".to_owned(),
            Error::IntegrityCheckFailure        => "Integrity Check Failure".to_owned(),
            Error::AllocationMismatch           => "Allocation mismatch".to_owned(),
            Error::StaleNonce                   => "Stale nonce".to_owned(),
            Error::AddressFamilyNotSupported    => "Address family not supported".to_owned(),
            Error::WrongCredentials             => "Wrong credentials".to_owned(),
            Error::UnsupportedTransportProtocol => "Unsupported transport protocol".to_owned(),
            Error::PeerAddressFamilyMismatch    => "Peer address family mismatch".to_owned(),
            Error::ConnectionAlreadyExists      => "Connection Already Exists".to_owned(),
            Error::ConnectionTimeoutOrFailure   => "Connection Timeout or Failure".to_owned(),
            Error::AllocationQuotaReached       => "Allocation quota reached".to_owned(),
            Error::RoleConflict                 => "Role conflict".to_owned(),
            Error::ServerError                  => "Server error".to_owned(),
            Error::InsufficientCapacity         => "Insufficient capacity".to_owned(),
            Error::GlobalFailure                => "Global Failure".to_owned(),
        }
    }
}

impl Error {
    pub fn from_u32(n: u32) -> Result<Self, &'static str> {
        match n {
            0 ... 299    => Err("Reserved"),
            301 ... 399
            | 402
            | 406 ... 419
            | 404
            | 439
            | 444 ... 445
            | 448 ... 485
            | 488 ... 499
            | 501 ... 507
            | 421 ... 430
            | 432 ... 436
            | 509 ... 599
            | 601 ... 699 => Err("Unassigned"),
            300 => Ok(Error::TryAlternate),
            400 => Ok(Error::BadRequest),
            401 => Ok(Error::Unauthorized),
            403 => Ok(Error::Forbidden),
            405 => Ok(Error::MobilityForbidden),
            420 => Ok(Error::UnknownAttribute),
            431 => Ok(Error::IntegrityCheckFailure),
            437 => Ok(Error::AllocationMismatch),
            438 => Ok(Error::StaleNonce),
            440 => Ok(Error::AddressFamilyNotSupported),
            441 => Ok(Error::WrongCredentials),
            442 => Ok(Error::UnsupportedTransportProtocol),
            443 => Ok(Error::PeerAddressFamilyMismatch),
            446 => Ok(Error::ConnectionAlreadyExists),
            447 => Ok(Error::ConnectionTimeoutOrFailure),
            486 => Ok(Error::AllocationQuotaReached),
            487 => Ok(Error::RoleConflict),
            500 => Ok(Error::ServerError),
            508 => Ok(Error::InsufficientCapacity),
            600 => Ok(Error::GlobalFailure),
            _   => Err("Code Range(0 ... 699)")
        }
    }
    pub fn to_u32 (&self) -> u32 {
        match *self {
            Error::TryAlternate                 => 300,
            Error::BadRequest                   => 400,
            Error::Unauthorized                 => 401,
            Error::Forbidden                    => 403,
            Error::MobilityForbidden            => 405,
            Error::UnknownAttribute             => 420,
            Error::IntegrityCheckFailure        => 431,
            Error::AllocationMismatch           => 437,
            Error::StaleNonce                   => 438,
            Error::AddressFamilyNotSupported    => 440,
            Error::WrongCredentials             => 441,
            Error::UnsupportedTransportProtocol => 442,
            Error::PeerAddressFamilyMismatch    => 443,
            Error::ConnectionAlreadyExists      => 446,
            Error::ConnectionTimeoutOrFailure   => 447,
            Error::AllocationQuotaReached       => 486,
            Error::RoleConflict                 => 487,
            Error::ServerError                  => 500,
            Error::InsufficientCapacity         => 508,
            Error::GlobalFailure                => 600
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
#[derive(Debug)]
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
The address family can take on the following values:
    
    0x01:IPv4
    0x02:IPv6
**/
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

#[derive(Debug)]
pub enum Attribute {
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

impl ToString for Attribute {
    fn to_string (&self) -> String {
        match *self {
                Attribute::MappedAddress => "MAPPED-ADDRESS".to_owned(),
                Attribute::ResponseAddress => "RESPONSE-ADDRESS".to_owned(),
                Attribute::ChangeRequest => "CHANGE-REQUEST".to_owned(),
                Attribute::SourceAddress => "SOURCE-ADDRESS".to_owned(),
                Attribute::ChangedAddress => "CHANGED-ADDRESS".to_owned(),
                Attribute::UserName => "USERNAME".to_owned(),
                Attribute::Password => "PASSWORD".to_owned(),
                Attribute::MessageIntegrity => "MESSAGE-INTEGRITY".to_owned(),
                Attribute::ErrorCode => "ERROR-CODE".to_owned(),
                Attribute::UnknownAttribute => "UNKNOWN-ATTRIBUTES".to_owned(),
                Attribute::ReflectedFrom => "REFLECTED-FROM".to_owned(),
                Attribute::ChannelNumber => "CHANNEL-NUMBER".to_owned(),
                Attribute::LifeTime => "LIFETIME".to_owned(),
                Attribute::BandWidth => "BANDWIDTH".to_owned(),
                Attribute::XorPeerAddress => "XOR-PEER-ADDRESS".to_owned(),
                Attribute::Data => "DATA".to_owned(),
                Attribute::Realm => "REALM".to_owned(),
                Attribute::Nonce => "NONCE".to_owned(),
                Attribute::XorRelayedAddress => "XOR-RELAYED-ADDRESS".to_owned(),
                Attribute::RequestAddressFamily => "REQUESTED-ADDRESS-FAMILY".to_owned(),
                Attribute::EvenPort => "EVEN-PORT".to_owned(),
                Attribute::RequestedTransport => "REQUESTED-TRANSPORT".to_owned(),
                Attribute::DontFragment => "DONT-FRAGMENT".to_owned(),
                Attribute::AccessToken => "ACCESS-TOKEN".to_owned(),
                Attribute::XorMappedAddress => "XOR-MAPPED-ADDRESS".to_owned(),
                Attribute::TimerVal => "TIMER-VAL".to_owned(),
                Attribute::ReservationToken => "RESERVATION-TOKEN".to_owned(),
                Attribute::Priority => "PRIORITY".to_owned(),
                Attribute::UseCandidate => "USE-CANDIDATE".to_owned(),
                Attribute::Padding => "PADDING".to_owned(),
                Attribute::ResponsePort => "RESPONSE-PORT".to_owned(),
                Attribute::ConnectionID => "CONNECTION-ID".to_owned(),
                Attribute::Software => "SOFTWARE".to_owned(),
                Attribute::AlternateServer => "ALTERNATE-SERVER".to_owned(),
                Attribute::TransactionTransmitCounter => "TRANSACTION_TRANSMIT_COUNTER".to_owned(),
                Attribute::CacheTimeout => "CACHE-TIMEOUT".to_owned(),
                Attribute::FingerPrint => "FINGERPRINT".to_owned(),
                Attribute::ICEControlled => "ICE-CONTROLLED".to_owned(),
                Attribute::ICEControlling => "ICE-CONTROLLING".to_owned(),
                Attribute::ResponseOrigin => "RESPONSE-ORIGIN".to_owned(),
                Attribute::OtherAddress => "OTHER-ADDRESS".to_owned(),
                Attribute::ECNCheckStun => "ECN-CHECK STUN".to_owned(),
                Attribute::ThirdPartyAuthorization => "THIRD-PARTY-AUTHORIZATION".to_owned(),
                Attribute::MobilityTicket => "MOBILITY-TICKET".to_owned(),
                Attribute::CiscoStunFlowData => "CISCO-STUN-FLOWDATA".to_owned(),
                Attribute::ENFFlowDescription => "ENF-FLOW-DESCRIPTION".to_owned(),
                Attribute::ENFNetworkStatus => "ENF-NETWORK-STATUS".to_owned()
        }
    }
}

impl Attribute {
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
            0x0001 => Ok(Attribute::MappedAddress),
            0x0002 => Ok(Attribute::ResponseAddress),
            0x0003 => Ok(Attribute::ChangeRequest),
            0x0004 => Ok(Attribute::SourceAddress),
            0x0005 => Ok(Attribute::ChangedAddress),
            0x0006 => Ok(Attribute::UserName),
            0x0007 => Ok(Attribute::Password),
            0x0008 => Ok(Attribute::MessageIntegrity),
            0x0009 => Ok(Attribute::ErrorCode),
            0x000A => Ok(Attribute::UnknownAttribute),
            0x000B => Ok(Attribute::ReflectedFrom),
            0x000C => Ok(Attribute::ChannelNumber),
            0x000D => Ok(Attribute::LifeTime),
            0x0010 => Ok(Attribute::BandWidth),
            0x0012 => Ok(Attribute::XorPeerAddress),
            0x0013 => Ok(Attribute::Data),
            0x0014 => Ok(Attribute::Realm),
            0x0015 => Ok(Attribute::Nonce),
            0x0016 => Ok(Attribute::XorRelayedAddress),
            0x0017 => Ok(Attribute::RequestAddressFamily),
            0x0018 => Ok(Attribute::EvenPort),
            0x0019 => Ok(Attribute::RequestedTransport),
            0x001A => Ok(Attribute::DontFragment),
            0x001B => Ok(Attribute::AccessToken),
            0x0020 => Ok(Attribute::XorMappedAddress),
            0x0021 => Ok(Attribute::TimerVal),
            0x0022 => Ok(Attribute::ReservationToken),
            0x0024 => Ok(Attribute::Priority),
            0x0025 => Ok(Attribute::UseCandidate),
            0x0026 => Ok(Attribute::Padding),
            0x0027 => Ok(Attribute::ResponsePort),
            0x002A => Ok(Attribute::ConnectionID),
            0x8022 => Ok(Attribute::Software),
            0x8023 => Ok(Attribute::AlternateServer),
            0x8025 => Ok(Attribute::TransactionTransmitCounter),
            0x8027 => Ok(Attribute::CacheTimeout),
            0x8028 => Ok(Attribute::FingerPrint),
            0x8029 => Ok(Attribute::ICEControlled),
            0x802A => Ok(Attribute::ICEControlling),
            0x802B => Ok(Attribute::ResponseOrigin),
            0x802C => Ok(Attribute::OtherAddress),
            0x802D => Ok(Attribute::ECNCheckStun),
            0x802E => Ok(Attribute::ThirdPartyAuthorization),
            0x8030 => Ok(Attribute::MobilityTicket),
            0xC000 => Ok(Attribute::CiscoStunFlowData),
            0xC001 => Ok(Attribute::ENFFlowDescription),
            0xC002 => Ok(Attribute::ENFNetworkStatus),
            _      => Err("Range(0x0000 ... 0xFFFF and ).")
        }
    }
    pub fn to_u32(&self) -> u32 {
        match *self {
                Attribute::MappedAddress    => 0x0001,
                Attribute::ResponseAddress  => 0x0002,
                Attribute::ChangeRequest    => 0x0003,
                Attribute::SourceAddress    => 0x0004,
                Attribute::ChangedAddress   => 0x0005,
                Attribute::UserName         => 0x0006,
                Attribute::Password         => 0x0007,
                Attribute::MessageIntegrity => 0x0008,
                Attribute::ErrorCode        => 0x0009,
                Attribute::UnknownAttribute => 0x000A,
                Attribute::ReflectedFrom    => 0x000B,
                Attribute::ChannelNumber    => 0x000C,
                Attribute::LifeTime         => 0x000D,
                Attribute::BandWidth        => 0x0010,
                Attribute::XorPeerAddress   => 0x0012,
                Attribute::Data             => 0x0013,
                Attribute::Realm            => 0x0014,
                Attribute::Nonce            => 0x0015,
                Attribute::XorRelayedAddress    => 0x0016,
                Attribute::RequestAddressFamily => 0x0017,
                Attribute::EvenPort             => 0x0018,
                Attribute::RequestedTransport   => 0x0019,
                Attribute::DontFragment         => 0x001A,
                Attribute::AccessToken          => 0x001B,
                Attribute::XorMappedAddress     => 0x0020,
                Attribute::TimerVal             => 0x0021,
                Attribute::ReservationToken     => 0x0022,
                Attribute::Priority             => 0x0024,
                Attribute::UseCandidate         => 0x0025,
                Attribute::Padding              => 0x0026,
                Attribute::ResponsePort         => 0x0027,
                Attribute::ConnectionID         => 0x002A,
                Attribute::Software             => 0x8022,
                Attribute::AlternateServer      => 0x8023,
                Attribute::TransactionTransmitCounter => 0x8025,
                Attribute::CacheTimeout         => 0x8027,
                Attribute::FingerPrint          => 0x8028,
                Attribute::ICEControlled        => 0x8029,
                Attribute::ICEControlling       => 0x802A,
                Attribute::ResponseOrigin       => 0x802B,
                Attribute::OtherAddress         => 0x802C,
                Attribute::ECNCheckStun         => 0x802D,
                Attribute::ThirdPartyAuthorization => 0x802E,
                Attribute::MobilityTicket          => 0x8030,
                Attribute::CiscoStunFlowData       => 0xC000,
                Attribute::ENFFlowDescription      => 0xC001,
                Attribute::ENFNetworkStatus        => 0xC002
        }
    }
}

#[derive(Debug)]
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

Message Struct
    [0, 1, 0, 0, 33, 18, 164, 66, 70, 149, 250, 122, 253, 177, 191, 174, 164, 118, 181, 61]
    Magic Code     : 2  bits
    Message Type   : 14 bits ( class: 2 bits; method: 12 bits; )
    Message Length : 16 bits (  2 Bytes )
    Magic Cookie   : 32 bits (  4 Bytes ) 
    Transaction ID : 96 bits ( 12 Bytes )

**/

#[derive(Debug)]
pub struct Message {
    magic_code    : u32,
    // Message Type ( 2 bits class, 12 bits method )
    class : Class,
    method: Method,
    length: u32,
    magic_cookie  : u32,   // 0x2112A442 (554869826)
    transaction_id: String
}

impl Message {
    pub fn bytes_to_hex_str(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("")
    }
    pub fn from_bytes (bytes: &[u8]) -> Result<Message, &'static str> {
        match bytes.len() {
            20 => {
                // 0b 00 01 000000000000
                let bits = format!("{:08b}", bytes[0]) + format!("{:08b}", bytes[1]).as_ref();
                let magic_code     = match u32::from_str_radix(&bits[0..2], 2) {
                    Ok(magic_code) => magic_code,
                    Err(_)         => return Err("magic code parse error")
                };
                // ((msg_type.clone() as u32) & 0x3EEF) as usize
                let message_class  = match u32::from_str_radix(&bits[2..4], 2) {
                    Ok(message_class) => match Class::from_u32(message_class) {
                        Ok(class) => class,
                        Err(e)    => return Err(e)
                    },
                    Err(_)        => return Err("message class error.")
                };
                // ((msg_type.clone() as u32) & 0x0110) as usize
                let message_method  = match u32::from_str_radix(&bits[4..16], 2) {
                    Ok(message_method) => match Method::from_u32(message_method) {
                        Ok(method) => method,
                        Err(e)     => return Err(e)
                    },
                    Err(_)         => return Err("message class error.")
                };

                let message_length = match u32::from_str_radix(Message::bytes_to_hex_str(&bytes[2..4]).as_ref(), 16) {
                    Ok(message_length) => message_length,
                    Err(_)             => return Err("message length error")
                };
                let magic_cookie   = match u32::from_str_radix(Message::bytes_to_hex_str(&bytes[4..8]).as_ref(), 16) {
                    Ok(magic_cookie) => {
                        if magic_cookie != 0x2112A442 {
                            return Err("Magic Cookie Must Be `0x2112A442`");
                        }
                        magic_cookie
                    },
                    Err(_)  => return Err("magic cookie error")
                };
                let transaction_id = Message::bytes_to_hex_str(&bytes[8..20]);
                Ok(Message {
                    magic_code    : magic_code,
                    class         : message_class,
                    method        : message_method,
                    length        : message_length,
                    magic_cookie  : magic_cookie,
                    transaction_id: transaction_id
                })
            },
            _  => Err("header size must be 20 Bytes.")
        }
    }

}

// default allocation lifetime (in seconds) unless refreshed 
pub static TURN_DEFAULT_ALLOCATION_LIFETIME: usize = 600;
// maximum allocation lifetime (in seconds) unless refreshed 
pub static TURN_MAX_ALLOCATION_LIFETIME: usize     = 3600;
// default permission lifetime (in seconds) unless refreshed 
pub static TURN_DEFAULT_PERMISSION_LIFETIME: usize = 300;
// default channel lifetime (in seconds) unless refreshed 
pub static TURN_DEFAULT_CHANNEL_LIFETIME: usize    = 600;
// lifetime of a nonce (in seconds) 
pub static TURN_DEFAULT_NONCE_LIFETIME: usize      = 3600;
// lifetime of a token (in seconds) 
pub static TURN_DEFAULT_TOKEN_LIFETIME: usize      = 60;

// RFC6062 (TURN-TCP) 
// Timeout of TCP relay when no ConnectionBind is received (in seconds) 
pub static TURN_DEFAULT_TCP_RELAY_TIMEOUT: usize   = 30;

// RFC6062 (TURN-TCP) 
// Timeout of TCP connect (in seconds) 
pub static TURN_DEFAULT_TCP_CONNECT_TIMEOUT: usize = 30;




pub fn tcp_handler(stream: &mut TcpStream) {
    println!("[INFO] Connection: {:?}", stream);

    let mut buff = [0; 2048];
    let size = stream.read(&mut buff[..]);
    println!("[DATA] {:?}", buff.to_vec() );
    println!("[INFO] Connection End.");
}

pub fn udp_handler(msg: &[u8], response: &mut [u8]) -> Result<usize, ()>{
    if msg.len() >= 20 {
        match Message::from_bytes(msg) {
            Ok(head) => println!("{:?}", head),
            Err(e)   => println!("{:?}", e)
        };
    }
    Ok(0)
}

pub fn tcp_server(host: &str){
    let listener = TcpListener::bind(host).unwrap();
    println!("[TCP Server] server running at : {:?}", listener);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || tcp_handler(&mut stream));
            },
            Err(e) => println!("[Error] {:?}", e)
        };
    }
}

pub fn udp_server(host: &str){
    let mut socket = UdpSocket::bind(host).unwrap();
    println!("[UDP Server] server running on {} ...", host);
    // [0, 1, 0, 0, 33, 18, 164, 66, 70, 149, 250, 122, 253, 177, 191, 174, 164, 118, 181, 61]
    loop {
        let mut buf = [0; 576];
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("[INFO] Connection: {:?}", src);
                let msg = &buf[..size];
                let mut response = [0; 576];
                // thread::spawn(move || udp_handler(&msg, &mut response));
                match udp_handler(&msg, &mut response) {
                    Ok(size) => {
                        if size > 0 {
                            socket.send_to(&response[..size], &src);
                        }
                    },
                    Err(_) => {}
                }

            },
            Err(e) => println!("[Error] {:?}", e)
        };
    }
    drop(socket);
}

pub fn run (host: &str, protocol: &str){
    match protocol.to_lowercase().as_str() {
        "tcp" => tcp_server(host),
        "udp" => udp_server(host),
        _     => panic!("[Error] protocol error {:?}", protocol)
    }
}