#![allow(unused_imports)]
extern crate ice;

use std::str::FromStr;
use std::string::ToString;

use ice::stun;


fn main() {
	let stun_client = "127.0.0.1:9000";
    let stun_server = "127.0.0.1:3478";
    let head = [
    	0, 1, 0, 0, 33, 18, 164, 66, 70, 149, 250, 
    	122, 253, 177, 191, 174, 164, 118, 181, 61];

    let mut client = stun::client::Client::new(Some(stun_client)).unwrap();
    client.set_server_uri(stun_server);
    
    let res = client.send(&head);
    println!("{:?}", res);
}