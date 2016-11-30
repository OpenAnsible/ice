
use std::str::FromStr;
use std::string::ToString;

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
pub enum ErrorCode {
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

impl ToString for ErrorCode {
    fn to_string (&self) -> String {
        match *self {
            ErrorCode::TryAlternate                 => "Try Alternate".to_owned(),
            ErrorCode::BadRequest                   => "Bad request".to_owned(),
            ErrorCode::Unauthorized                 => "Unauthorized".to_owned(),
            ErrorCode::Forbidden                    => "Forbidden".to_owned(),
            ErrorCode::MobilityForbidden            => "Mobility forbidden".to_owned(),
            ErrorCode::UnknownAttribute             => "Unknown attribute(s)".to_owned(),
            ErrorCode::IntegrityCheckFailure        => "Integrity Check Failure".to_owned(),
            ErrorCode::AllocationMismatch           => "Allocation mismatch".to_owned(),
            ErrorCode::StaleNonce                   => "Stale nonce".to_owned(),
            ErrorCode::AddressFamilyNotSupported    => "Address family not supported".to_owned(),
            ErrorCode::WrongCredentials             => "Wrong credentials".to_owned(),
            ErrorCode::UnsupportedTransportProtocol => "Unsupported transport protocol".to_owned(),
            ErrorCode::PeerAddressFamilyMismatch    => "Peer address family mismatch".to_owned(),
            ErrorCode::ConnectionAlreadyExists      => "Connection Already Exists".to_owned(),
            ErrorCode::ConnectionTimeoutOrFailure   => "Connection Timeout or Failure".to_owned(),
            ErrorCode::AllocationQuotaReached       => "Allocation quota reached".to_owned(),
            ErrorCode::RoleConflict                 => "Role conflict".to_owned(),
            ErrorCode::ServerError                  => "Server error".to_owned(),
            ErrorCode::InsufficientCapacity         => "Insufficient capacity".to_owned(),
            ErrorCode::GlobalFailure                => "Global Failure".to_owned(),
        }
    }
}

impl ErrorCode {
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
            300 => Ok(ErrorCode::TryAlternate),
            400 => Ok(ErrorCode::BadRequest),
            401 => Ok(ErrorCode::Unauthorized),
            403 => Ok(ErrorCode::Forbidden),
            405 => Ok(ErrorCode::MobilityForbidden),
            420 => Ok(ErrorCode::UnknownAttribute),
            431 => Ok(ErrorCode::IntegrityCheckFailure),
            437 => Ok(ErrorCode::AllocationMismatch),
            438 => Ok(ErrorCode::StaleNonce),
            440 => Ok(ErrorCode::AddressFamilyNotSupported),
            441 => Ok(ErrorCode::WrongCredentials),
            442 => Ok(ErrorCode::UnsupportedTransportProtocol),
            443 => Ok(ErrorCode::PeerAddressFamilyMismatch),
            446 => Ok(ErrorCode::ConnectionAlreadyExists),
            447 => Ok(ErrorCode::ConnectionTimeoutOrFailure),
            486 => Ok(ErrorCode::AllocationQuotaReached),
            487 => Ok(ErrorCode::RoleConflict),
            500 => Ok(ErrorCode::ServerError),
            508 => Ok(ErrorCode::InsufficientCapacity),
            600 => Ok(ErrorCode::GlobalFailure),
            _   => Err("Code Range(0 ... 699)")
        }
    }
    pub fn to_u32 (&self) -> u32 {
        match *self {
            ErrorCode::TryAlternate                 => 300,
            ErrorCode::BadRequest                   => 400,
            ErrorCode::Unauthorized                 => 401,
            ErrorCode::Forbidden                    => 403,
            ErrorCode::MobilityForbidden            => 405,
            ErrorCode::UnknownAttribute             => 420,
            ErrorCode::IntegrityCheckFailure        => 431,
            ErrorCode::AllocationMismatch           => 437,
            ErrorCode::StaleNonce                   => 438,
            ErrorCode::AddressFamilyNotSupported    => 440,
            ErrorCode::WrongCredentials             => 441,
            ErrorCode::UnsupportedTransportProtocol => 442,
            ErrorCode::PeerAddressFamilyMismatch    => 443,
            ErrorCode::ConnectionAlreadyExists      => 446,
            ErrorCode::ConnectionTimeoutOrFailure   => 447,
            ErrorCode::AllocationQuotaReached       => 486,
            ErrorCode::RoleConflict                 => 487,
            ErrorCode::ServerError                  => 500,
            ErrorCode::InsufficientCapacity         => 508,
            ErrorCode::GlobalFailure                => 600
        }
    }
}