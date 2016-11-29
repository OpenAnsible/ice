#![feature(lookup_host)]
#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_must_use, unreachable_code, non_snake_case)]

extern crate url;
extern crate rand;

use std::string::ToString;
use std::convert::AsRef;
use std::io::{Read, Write};
use std::{thread, time};

use std::net::{ SocketAddr, IpAddr, TcpListener, TcpStream, UdpSocket, Shutdown };

pub mod stun;

