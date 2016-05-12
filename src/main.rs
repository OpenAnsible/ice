
use std::io::Write;
use std::net::{ TcpListener, TcpStream, UdpSocket };
use std::thread;

fn tcp_server() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    println!("listening started, ready to accept");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    let mut stream = stream.unwrap();
                    stream.write(b"Hello World\r\n").unwrap();
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
    // close the socket server
    drop(listener);
}

fn udp_server(){
    let mut socket = UdpSocket::bind("127.0.0.1:34254");
    thread::spawn(move|| {
        let mut buf = [0; 10];
        let (amt, src) = try!(socket.recv_from(&mut buf));

        // Send a reply to the socket we received data from
        let buf = &mut buf[..amt];
        buf.reverse();
        try!(socket.send_to(buf, &src));
    });
    // close the socket
    drop(socket);
}

fn main() {
    tcp_server();
}
