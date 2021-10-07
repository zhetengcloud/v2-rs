use flexi_logger::Logger;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 8];
    if let Ok(sz) = stream.read(&mut buf) {
        log::debug!("read {}, {:?}", sz, buf);
    }
}

pub fn main() {
    let _lg = Logger::try_with_str("debug")
        .unwrap()
        .log_to_stdout()
        .start()
        .unwrap();

    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    log::debug!("socks listening on 4000");

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
