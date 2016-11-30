#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_must_use, unreachable_code, non_snake_case, non_camel_case_types)]


use std::{thread, time, mem};

use std::str::FromStr;
use std::string::ToString;
use std::convert::AsRef;
use std::io::{Read, Write};

pub mod packet;
pub mod client;
pub mod server;
pub mod port;
pub mod urlparse;

pub use self::port::{STUN_PORT, STUNS_PORT};
pub use self::urlparse::url_parse;
// pub use self::client::Client;

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


pub const STUN_FINGERPRINT_XOR_VALUE: u32 = 0x5354554E; // STUN FINGERPRINT XOR Value

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
