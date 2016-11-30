
use std::str::FromStr;
use std::string::ToString;

use std::net::{ SocketAddr, IpAddr, TcpStream, UdpSocket };
use super::{url_parse, STUN_PORT, STUNS_PORT};

// https://zh.wikipedia.org/wiki/%E7%BD%91%E7%BB%9C%E5%9C%B0%E5%9D%80%E8%BD%AC%E6%8D%A2
pub enum Nat {
    FullCone,              // 一对一
    AddressRestrictedCone, // 地址受限锥形 NAT
    PortRestrictedCone,    // 端口受限锥形 NAT
    Symmetric              // 对称 NAT
}

#[derive(Debug)]
pub struct Client {
    server: Option<SocketAddr>,
    client: UdpSocket
}

impl Client {
    pub fn new(uri: Option<&str>) -> Result<Self, &'static str> {
        let mut url: String = String::new();
        if uri.is_none() {
            url = format!("stun://127.0.0.1:{}", STUN_PORT);
        } else {
            url = uri.unwrap().to_owned();
        };

        let socket_addr   = url_parse(&url).expect("local uri format error.");
        let client_socket = UdpSocket::bind(socket_addr).expect("Couldn't bind port");

        match socket_addr.ip().is_loopback() {
            true => Ok(Client { server: None, client: client_socket}),
            _    => Err("local_uri ip error.")
        }
    }
    pub fn set_server_uri(&mut self, uri: &str) -> bool {
        let stun_server_socket_addr = url_parse(uri).expect("server uri format error.");
        self.server = Some(stun_server_socket_addr);
        true
    }
    pub fn send(&self, msg: &[u8]) -> Result<usize, &'static str> {
        assert_eq!(self.server.is_some(), true);
        let target = self.server.unwrap();
        match self.client.send_to(msg, target) {
            Ok(size) => Ok(size),
            Err(_)   => Err("send error.")
        }
    }
    pub fn nat (&self) {
        assert_eq!(self.server.is_some(), true);

    }
}