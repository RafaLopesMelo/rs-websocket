mod http;
mod ws;

use http::request::Request;

use std::io::Write;
use std::net;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let req = Request::from_raw(&stream);

        let response = ws::handshake(&req);

        let _ = stream.write(&response.unwrap());
        let _ = stream.flush();
    }
}
