use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read,Write};
use std::str;



fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9527").unwrap();
    println!("Connection 9527 sucessed!");

    stream.write("helloworld".as_bytes()).unwrap();

    let mut buffer = [0;10];

    stream.read(&mut buffer).unwrap();
    println!("Response from server {:?}",str::from_utf8(&buffer).unwrap());
}
