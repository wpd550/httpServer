use  std::net::TcpListener;
use std::io::{Read,Write};

fn main() {
    
    let listener  = TcpListener::bind("127.0.0.1:9527").unwrap();

    println!("Runing on port 9527");
    for strean in listener.incoming() {
       match strean {
           Ok(mut strean) => {
                println!("Connection established!");
                let mut buffer = [0;1024];
                strean.read(&mut buffer).unwrap();

                strean.write(&mut buffer).unwrap();


           }
           Err(e) => {

           }
       }
    }
}
