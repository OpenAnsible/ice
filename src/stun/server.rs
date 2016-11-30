
use std::thread;
use std::str::FromStr;
use std::string::ToString;
use std::io::{Read, Write};
use std::net::{ SocketAddr, IpAddr, TcpListener, TcpStream, UdpSocket, Shutdown };

use super::{url_parse, STUN_PORT, STUNS_PORT};
use super::{packet};

pub fn tcp_handler(stream: &mut TcpStream) {
    println!("[INFO] Connection: {:?}", stream);

    let mut buff = [0; 2048];
    let size = stream.read(&mut buff[..]);
    println!("[DATA] {:?}", buff.to_vec() );
    println!("[INFO] Connection End.");
}

pub fn udp_handler(msg: &[u8], response: &mut [u8]) -> Result<usize, ()>{
    if msg.len() >= 20 {
        match packet::Header::from_bytes(msg) {
            Ok(head) => println!("{:?}", head),
            Err(e)   => println!("{:?}", e)
        };
    }
    Ok(0)
}

pub fn tcp_server(host: &str){
    let socket_addr = url_parse(host).expect("local uri format error.");
    let listener = TcpListener::bind(socket_addr).unwrap();
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
    let socket_addr = url_parse(host).expect("local uri format error.");
    let mut socket  = UdpSocket::bind(socket_addr).unwrap();
    println!("[UDP Server] server running on {} ...", socket_addr);
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