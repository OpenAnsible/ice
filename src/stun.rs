use std::io::{ Read, Write, Result };
use std::{ str };
use std::string::String;

// Shutdown{ Read, Write, Both }
use std::net::{ UdpSocket, Shutdown, SocketAddr };
use std::thread;
use std::collections::btree_map::BTreeMap;

// Public STUN Servers
//      stun.xten.net:3478
//      sip.iptel.org:3478
//      tesla.divmod.net:3478
//      erlang.divmod.net:3478
//      stun.wirlab.net:3478
//      stun2.wirlab.net:3478
//      stun1.vovida.org:3478


// Methods
pub static STUN_METHOD_BINDING: usize  = 0x0001; // STUN Protocol

pub static TURN_METHOD_ALLOCATE: usize = 0x0003; // TURN Protocol
pub static TURN_METHOD_REFRESH: usize  = 0x0004; // TURN Protocol
pub static TURN_METHOD_SEND: usize             = 0x0006; // TURN Protocol
pub static TURN_METHOD_DATA: usize             = 0x0007; // TURN Protocol
pub static TURN_METHOD_CREATEPERMISSION: usize = 0x0008; // TURN Protocol
pub static TURN_METHOD_CHANNELBIND: usize      = 0x0009; // TURN Protocol
pub static TURN_METHOD_CONNECT: usize          = 0x000A; // TURN Protocol ( RFC 6062 )
pub static TURN_METHOD_CONNECTIONBIND: usize   = 0x000B; // TURN Protocol ( RFC 6062 )
pub static TURN_METHOD_CONNECTIONATTEMPT: usize= 0x000C; // TURN Protocol ( RFC 6062 )
// Indications

// Classes
pub static STUN_CLASS_REQUEST: usize         = 0x0000; // STUN Protocol
pub static STUN_CLASS_INDICATION: usize      = 0x0010; // STUN Protocol
pub static STUN_CLASS_SUCCESSRESPONSE: usize = 0x0100; // STUN Protocol
pub static STUN_CLASS_ERRORRESPONSE: usize   = 0x0110; // STUN Protocol

// STUN Attr Message Type
//    Required: 0x0000-0x7FFF, Options: 0x8000-0xFFFF
pub static STUN_ATTR_MAPPED_ADDRESS: usize   = 0x0001;
pub static STUN_ATTR_RESPONSE_ADDRESS: usize = 0x0002;
pub static STUN_ATTR_CHANGE_REQUEST: usize   = 0x0003;
pub static STUN_ATTR_SOURCE_ADDRESS: usize   = 0x0004;
pub static STUN_ATTR_CHANGED_ADDRESS: usize  = 0x0005;
pub static STUN_ATTR_USERNAME: usize         = 0x0006;
pub static STUN_ATTR_PASSWORD: usize         = 0x0007;
pub static STUN_ATTR_MESSAGE_INTEGRITY: usize= 0x0008;
pub static STUN_ATTR_ERROR_CODE: usize       = 0x0009;
pub static STUN_ATTR_UNKNOWN_ATTRIBUTES: usize = 0x000A;
pub static STUN_ATTR_REFLECTED_FROM: usize     = 0x000B;
pub static TURN_ATTR_CHANNEL_NUMBER: usize     = 0x000C; // TURN Attr
pub static TURN_ATTR_LIFETIME: usize           = 0x000D; // TURN Attr

pub static TURN_ATTR_XOR_PEER_ADDRESS: usize   = 0x0012; // TURN
pub static TURN_ATTR_DATA: usize               = 0x0013; // TURN
pub static STUN_ATTR_REALM: usize              = 0x0014;
pub static STUN_ATTR_NONCE: usize              = 0x0015;
pub static TURN_ATTR_XOR_RELAYED_ADDRESS: usize= 0x0016; // TURN
pub static TURN_ATTR_REQUESTED_ADDRESS_FAMILY: usize = 0x0017; // TURN ( RFC 6156 )
pub static TURN_ATTR_EVEN_PORT: usize          = 0x0018; // TURN
pub static TURN_ATTR_REQUESTED_TRANSPORT: usize= 0x0019; // TURN
pub static TURN_ATTR_DONT_FRAGMENT: usize      = 0x001A; // TURN
pub static STUN_ATTR_XOR_MAPPED_ADDRESS: usize = 0x0020;
pub static TURN_ATTR_RESERVATION_TOKEN: usize  = 0x0022; // TURN
pub static TURN_ATTR_CONNECTION_ID: usize      = 0x002A; // TURN ( RFC 6062 )

pub static STUN_ATTR_SOFTWARE: usize           = 0x8022;
pub static STUN_ATTR_ALTERNATE_SERVER: usize   = 0x8023;
pub static STUN_ATTR_FINGERPRINT: usize        = 0x8028;

// STUN Error Recommended Reasons
pub static STUN_ERROR_300: &'static str    = "Try Alternate";
pub static STUN_ERROR_400: &'static str    = "Bad request";
pub static STUN_ERROR_401: &'static str    = "Unauthorized";
pub static STUN_ERROR_420: &'static str    = "Unknown attribute(s)";
pub static STUN_ERROR_431: &'static str    = "Integrity Check Failure";
pub static STUN_ERROR_438: &'static str    = "Stale nonce";
pub static STUN_ERROR_500: &'static str    = "Server error";
pub static STUN_ERROR_600: &'static str    = "Global Failure";

// TURN Error Recommended Reasons
pub static TURN_ERROR_403: &'static str    = "Forbidden";
pub static TURN_ERROR_437: &'static str    = "Allocation mismatch";
pub static TURN_ERROR_441: &'static str    = "Wrong credentials";
pub static TURN_ERROR_442: &'static str    = "Unsupported transport protocol";
pub static TURN_ERROR_486: &'static str    = "Allocation quota reached";
pub static TURN_ERROR_508: &'static str    = "Insufficient capacity";
pub static TURN_ERROR_440: &'static str    = "Address family not supported"; // RFC 6156 (TURN-IPV6)
pub static TURN_ERROR_443: &'static str    = "Peer address family mismatch"; // RFC 6156 (TURN-IPV6)
pub static TURN_ERROR_446: &'static str    = "Connection Already Exists";    // RFC6062 (TURN-TCP)
pub static TURN_ERROR_447: &'static str    = "Connection Timeout or Failure";// RFC6062 (TURN-TCP)

pub static STUN_MAGIC_COOKIE: usize          = 0x2112A442; // STUN Magic Cookie
pub static STUN_FINGERPRINT_XOR_VALUE: usize = 0x5354554E; // STUN FINGERPRINT XOR Value

// family address for MAPPED-ADDRESS like attributes 
pub static STUN_ATTR_FAMILY_IPV4: usize      = 0x01;
pub static STUN_ATTR_FAMILY_IPV6: usize      = 0x02;

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

// pub fn hex(number: usize) -> String {

// }
// pub fn bin(number: usize) -> String {

// }
// pub fn ord(_char: &str) -> usize {

// }
// pub fn chr(number: usize) -> char {

// }

pub mod Message {
    pub const HeadLength: usize      = 20;
    // 0b000000000001 -> 0b 000000 000001 -> method, class -> 0x00 0x01 -> Binding, Indication
    pub const MagicCode: usize       = 0x00; // 0, Begin.

    pub const Request: usize         = 0x0001; // Binding Request
    pub const Indication: usize      = 0x0101; // 
    pub const SuccessResponse: usize = 0x0101; // Binding Success Response
    pub const ErrorResponse: usize   = 0x0111; // Binding Error Response
    pub const MagicCookie: usize     = 0x2112A442; //[0x21, 0x12, 0xA4, 0x42], [33, 18, 164, 66]

    #[derive(Debug)]
    pub struct Header {
        pub prefix: usize, // must be 0x0000
        /// Message Type: Method(12Bit) + Class(2Bit) = Message Type(14 Bit).
        // STUN: Binding
        // TURN: Allocate, Refresh, CreatePermission, ChannelBind, 
        //       Connect, ConnectionBind, Send, Data, ConnectionAttempt
        pub method: usize,
        // STUN: Request, Indication, SuccessResponse, ErrorResponse
        pub class : usize,
        pub length: usize, // 0xAA 0x00
        pub magic_cookie  : usize, // 0x2112A442
        pub transaction_id: String
    }

    impl Header {
        pub fn parse (header: &[u8]) -> Result<Header, String> {
            if header.len() != 20 {
                return Err("Error: header size must be 20 Bytes.".to_string());
            }
            let msg_prefix  = header[0] as usize;  // 0x00
            if msg_prefix != MagicCode {
                return Err("Error: protocol error.".to_string());
            }
            // Message Type(1 Bytes):
            //     format!("0x{:02X}", header[1]);
            let msg_method = Header::get_method(&header[1]);
            let msg_class  = Header::get_class(&header[1]);
            // Last Bit Must Be 0x00
            let msg_length = Header::bytes_to_usize(&header[2..4]);

            // MagicCookie Must Be 0x2112A442 ( 554869826 )
            //    Bytes: [0x21, 0x12, 0xA4, 0x42], [33, 18, 164, 66]
            let msg_magic_cookie   = Header::bytes_to_usize(&header[4..8]);
            if msg_magic_cookie != 0x2112A442 {
                return Err("Error: STUN Message MagicCookie must be 0x2112A442.".to_string());
            }
            let msg_transaction_id = Header::bytes_to_hex_str(&header[8..20]);

            Ok(Header { prefix: 0x00, method: msg_method, class : msg_class, length: msg_length,
                magic_cookie  : 0x2112A442, transaction_id: msg_transaction_id.to_string()})
        }
        pub fn get_method(msg_type: &u8)->usize {
            ((msg_type) & 0x3EEF) as usize
        }
        pub fn get_class(msg_type: &u8)->usize {
            ((msg_type) & 0x0110) as usize
        }
        pub fn bytes_to_hex_str(bytes: &[u8]) -> String {
            let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
            strs.connect("")
        }
        pub fn bytes_to_usize(bytes: &[u8] ) -> usize {
            let hex_str = Header::bytes_to_hex_str(bytes);
            Header::hex_str_to_usize(&hex_str)
        }
        pub fn hex_str_to_usize(hex_str: &str) -> usize {
            usize::from_str_radix(hex_str, 16).unwrap()
        }
    }

    // #[derive(Debug)]
    // pub struct Attr {
    //     t: usize, // Required: 0x0000-0x7FFF, Options: 0x8000-0xFFFF
    //     l: usize,
    //     v: 
    // }
}

fn onmessage (socket: &UdpSocket) {
    // STUN Header
    let mut header = vec![0; 20];
    match socket.recv_from(&mut header) {
        Ok((length, address)) => {
            let header = self::Message::Header::parse(&header);
            match header {
                Ok(header) => {
                    println!("{:?}", header);
                    
                    let mut body: Vec<u8> = Vec::with_capacity(header.length);
                    let _ = socket.recv_from(&mut body);
                    println!("{:?}", body );

                    // let msg_attrs = BTreeMap::new();
                    // msg_attrs.entry(id).or_insert( " ...." );
                    // let mut total: usize = 0;
                    // while true {
                    //     let mut buff = vec![0;4];
                    //     match socket.recv_from(&mut buff) {
                    //         Ok((length, address)) => {
                    //             println!("length: {:?} , address: {:?}, buff: {:?}", length, address, buff );
                    //             break;
                    //         },
                    //         Err(_) => {
                    //             break;
                    //         }
                    //     }
                    // };
                },
                Err(_) => {

                }
            }

            

        },
        Err(_) => {
            // pass
            ()
        }
    }
}



pub fn run (host: &str){
    let mut socket = UdpSocket::bind(host).unwrap();
    println!("[UDP Server] server running on {} ...", host);
    loop {
        onmessage(&socket)
    };
    // close the socket
    drop(socket);
}