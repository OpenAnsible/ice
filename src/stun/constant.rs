

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

pub const STUN_MAGIC_COOKIE: u32 = 0x2112A442;

pub const STUN_FINGERPRINT_XOR_VALUE: u32 = 0x5354554E; // STUN FINGERPRINT XOR Value

// default allocation lifetime (in seconds) unless refreshed 
pub const TURN_DEFAULT_ALLOCATION_LIFETIME: u32 = 600;
// maximum allocation lifetime (in seconds) unless refreshed 
pub const TURN_MAX_ALLOCATION_LIFETIME: u32     = 3600;
// default permission lifetime (in seconds) unless refreshed 
pub const TURN_DEFAULT_PERMISSION_LIFETIME: u32 = 300;
// default channel lifetime (in seconds) unless refreshed 
pub const TURN_DEFAULT_CHANNEL_LIFETIME: u32    = 600;
// lifetime of a nonce (in seconds) 
pub const TURN_DEFAULT_NONCE_LIFETIME: u32      = 3600;
// lifetime of a token (in seconds) 
pub const TURN_DEFAULT_TOKEN_LIFETIME: u32      = 60;

// RFC6062 (TURN-TCP) 
// Timeout of TCP relay when no ConnectionBind is received (in seconds) 
pub const TURN_DEFAULT_TCP_RELAY_TIMEOUT: u32   = 30;

// RFC6062 (TURN-TCP) 
// Timeout of TCP connect (in seconds) 
pub const TURN_DEFAULT_TCP_CONNECT_TIMEOUT: u32 = 30;
