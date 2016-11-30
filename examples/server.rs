#![allow(unused_imports)]
extern crate ice;

use std::str::FromStr;
use std::string::ToString;

use ice::stun;


fn main() {
    let host = "127.0.0.1:3478";
    stun::server::run(host, "udp");
}