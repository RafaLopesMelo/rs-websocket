mod http;

use http::request::Request;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::net;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let req = Request::from_raw(stream);

        if !req.is_get() {
            panic!("only GET requests are allowed for websocket handshake");
        }

        if !req.is_http() {
            panic!("only HTTP is allowed for websocket handshake");
        }

        if !req.meets_version("1.1") {
            panic!("only HTTP/1.1 is allowed for websocket handshake");
        }

        let upgrade = req
            .header("upgrade")
            .expect("`upgrade` header is required for websocket handshake");

        if upgrade.to_lowercase().contains("websocket") {
            panic!("`upgrade` header must include `websocket` token for websocket handshake");
        }

        let connection = req
            .header("connection")
            .expect("`connection` header is required for websocket handshake");

        if connection.to_lowercase().contains("upgrade") {
            panic!("`connection` header must include `upgrade` token for websocket handshake");
        }

        let base64_ws_key = req
            .header("sec-websocket-key")
            .expect("`sec-websocket-key` header is required for websocket handshake");

        let ws_key = STANDARD.decode(base64_ws_key).expect(
            "`sec-websocket-key` header must be a valid base64 string for websocket handshake",
        );

        if ws_key.len() != 16 {
            panic!("`sec-websocket-key` header must be a 16 bytes long encoded in base64 for websocket handshake");
        }

        let ws_version = req
            .header("sec-websocket-version")
            .expect("`sec-websocket-version` header is required for websocket handshake");

        if ws_version != "13" {
            panic!("`sec-websocket-version` header must be `13` for websocket handshake");
        }
    }
}
