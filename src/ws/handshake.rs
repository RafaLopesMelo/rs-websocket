use crate::http;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use sha1::Digest;

const WS_SERVER_KEY: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub fn handshake(req: &http::Request) -> Vec<u8> {
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

    let ws_key = STANDARD
        .decode(base64_ws_key)
        .expect("`sec-websocket-key` header must be a valid base64 string for websocket handshake");

    if ws_key.len() != 16 {
        panic!("`sec-websocket-key` header must be a 16 bytes long encoded in base64 for websocket handshake");
    }

    let ws_version = req
        .header("sec-websocket-version")
        .expect("`sec-websocket-version` header is required for websocket handshake");

    if ws_version != "13" {
        panic!("`sec-websocket-version` header must be `13` for websocket handshake");
    }

    let str_ws_key = std::str::from_utf8(&ws_key).unwrap();

    let mut str_ws_accept = String::from(str_ws_key);
    str_ws_accept.push_str(WS_SERVER_KEY);

    let mut hasher = sha1::Sha1::new();
    hasher.update(str_ws_accept);
    let ws_accept = &hasher.finalize() as &[u8];

    let builder = http::Builder::new();
    let response = builder
        .status(http::StatusCode::SWITCHING_PROTOCOLS)
        .header("upgrade", "websocket")
        .header("connection", "upgrade")
        .header(
            "sec-websocket-accept",
            std::str::from_utf8(ws_accept).unwrap(),
        )
        .build();

    return response;
}
