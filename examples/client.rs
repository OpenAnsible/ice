
extern crate ice;

use std::str::FromStr;
use std::string::ToString;

use ice::stun;


fn main() {
    let uri      = "127.0.0.1:3478";
    let local_uri = "127.0.0.1:9090";
    let client = stun::Client::new(uri, Some(local_uri)).unwrap();
    let head = [
        0, 1, 0, 0, 33, 18, 164, 66, 70, 149, 250, 
        122, 253, 177, 191, 174, 164, 118, 181, 61];
    let res = client.send(&head);
    println!("{:?}",  res);
}