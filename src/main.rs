#![feature(lookup_host)]
#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
#![allow(unused_must_use, unreachable_code, non_snake_case, unused_assignments)]

extern crate url;
extern crate rand;

use std::string::ToString;
use std::convert::AsRef;
use std::io::{Read, Write};
use std::{thread, time};

use std::net::{ SocketAddr, IpAddr, TcpListener, TcpStream, UdpSocket, Shutdown };

pub mod stun;


fn main() {
    let stun_server = "127.0.0.1:3478";
    
    let data = [0, 1, 0, 0, 33, 18, 164, 66, 70, 149, 250, 
    122, 253, 177, 191, 174, 164, 118, 181, 61];

    let mut client = stun::client::Client::new(None).unwrap();
    client.set_server_uri(stun_server);

    let res = client.send(&data);

    println!("{:?}", res);

}
