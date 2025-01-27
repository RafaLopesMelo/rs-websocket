use super::error::HandshakeError;
use crate::http;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use sha1::Digest;

const WS_SERVER_KEY: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub fn handshake(req: &http::request::Request) -> Result<Vec<u8>, HandshakeError> {
    if !req.is_get() {
        return Err(HandshakeError::InvalidMethod);
    }

    if !req.is_http() {
        return Err(HandshakeError::InvalidProtocol);
    }

    if !req.meets_version("1.1") {
        return Err(HandshakeError::InvalidProtocolVersion);
    }

    let upgrade = req
        .header("upgrade")
        .ok_or(HandshakeError::InvalidUpgrade)?;

    if !upgrade.to_lowercase().contains("websocket") {
        return Err(HandshakeError::InvalidUpgrade);
    }

    let connection = req
        .header("connection")
        .ok_or(HandshakeError::InvalidConnection)?;

    if !connection.to_lowercase().contains("upgrade") {
        return Err(HandshakeError::InvalidConnection);
    }

    let base64_ws_key = req
        .header("sec-websocket-key")
        .ok_or(HandshakeError::InvalidWSKey)?;

    let ws_key = STANDARD
        .decode(base64_ws_key)
        .map_err(|_| HandshakeError::InvalidWSKey)?;

    if ws_key.len() != 16 {
        return Err(HandshakeError::InvalidWSKey);
    }

    let ws_version = req
        .header("sec-websocket-version")
        .ok_or(HandshakeError::InvalidWSVersion)?;

    if ws_version != "13" {
        return Err(HandshakeError::InvalidWSVersion);
    }

    let str_ws_key = std::str::from_utf8(&ws_key).map_err(|_| HandshakeError::InvalidWSKey)?;

    let mut str_ws_accept = String::from(str_ws_key);
    str_ws_accept.push_str(WS_SERVER_KEY);

    let mut hasher = sha1::Sha1::new();
    hasher.update(str_ws_accept);
    let ws_accept_bytes = &hasher.finalize() as &[u8];

    let ws_accept = STANDARD.encode(ws_accept_bytes);

    let builder = http::response::Builder::new();
    let response = builder
        .status(http::response::StatusCode::SWITCHING_PROTOCOLS)
        .header("upgrade", "websocket")
        .header("connection", "upgrade")
        .header("sec-websocket-accept", &ws_accept)
        .build();

    return Ok(response);
}

#[cfg(test)]
mod tests {
    use super::*;

    const FAKE_WS_KEY: &str = "dGhlIHNhbXBsZSBub25jZQ==";
    const FAKE_WS_ACCEPT: &str = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";

    fn base_req_builder() -> http::request::Builder {
        return http::request::Builder::new()
            .method("GET")
            .path("/test")
            .protocol_name("HTTP")
            .protocol_version("1.1")
            .header("upgrade", "websocket")
            .header("connection", "upgrade")
            .header("sec-websocket-key", FAKE_WS_KEY)
            .header("sec-websocket-version", "13");
    }

    #[test]
    fn test_valid_handshake_succeeds() {
        let builder = http::response::Builder::new();

        let _response = builder
            .status(http::response::StatusCode::SWITCHING_PROTOCOLS)
            .header("upgrade", "websocket")
            .header("connection", "upgrade")
            .header("sec-websocket-accept", FAKE_WS_ACCEPT)
            .build();

        let req = base_req_builder().build();

        assert!(matches!(handshake(&req), Ok(_response)));
    }

    #[test]
    fn test_non_get_returns_error() {
        let req = base_req_builder().method("POST").build();

        assert!(matches!(
            handshake(&req),
            Err(HandshakeError::InvalidMethod)
        ));
    }

    #[test]
    fn test_invalid_protocol_returns_error() {
        let req = base_req_builder().protocol_name("TEST").build();

        assert!(matches!(
            handshake(&req),
            Err(HandshakeError::InvalidProtocol)
        ));
    }

    #[test]
    fn test_invalid_protocol_version_returns_error() {
        let req = base_req_builder().protocol_version("1.0").build();

        assert!(matches!(
            handshake(&req),
            Err(HandshakeError::InvalidProtocolVersion)
        ));
    }

    #[test]
    fn test_without_upgrade_returns_error() {
        let req = http::request::Builder::new()
            .method("GET")
            .path("/test")
            .protocol_name("HTTP")
            .protocol_version("1.1")
            .header("connection", "upgrade")
            .header("sec-websocket-key", FAKE_WS_KEY)
            .header("sec-websocket-version", "13")
            .build();

        assert!(matches!(
            handshake(&req),
            Err(HandshakeError::InvalidUpgrade)
        ));
    }

    #[test]
    fn test_invalid_upgrade_returns_error() {
        let req = base_req_builder().header("upgrade", "test").build();

        assert!(matches!(
            handshake(&req),
            Err(HandshakeError::InvalidUpgrade)
        ));
    }

    #[test]
    fn test_without_connection_returns_error() {
        let req = http::request::Builder::new()
            .method("GET")
            .path("/test")
            .protocol_name("HTTP")
            .protocol_version("1.1")
            .header("upgrade", "websocket")
            .header("sec-websocket-key", FAKE_WS_KEY)
            .header("sec-websocket-version", "13")
            .build();

        assert!(matches!(
            handshake(&req),
            Err(HandshakeError::InvalidConnection)
        ));
    }

    #[test]
    fn test_invalid_connection_returns_error() {
        let req = base_req_builder().header("connection", "test").build();

        assert!(matches!(
            handshake(&req),
            Err(HandshakeError::InvalidConnection)
        ));
    }

    #[test]
    fn test_without_ws_key_returns_error() {
        let req = http::request::Builder::new()
            .method("GET")
            .path("/test")
            .protocol_name("HTTP")
            .protocol_version("1.1")
            .header("upgrade", "websocket")
            .header("connection", "upgrade")
            .header("sec-websocket-version", "13")
            .build();

        assert!(matches!(handshake(&req), Err(HandshakeError::InvalidWSKey)));
    }

    #[test]
    fn test_invalid_base64_ws_key_returns_error() {
        let req = base_req_builder()
            .header("sec-websocket-key", "test")
            .build();

        assert!(matches!(handshake(&req), Err(HandshakeError::InvalidWSKey)));
    }

    #[test]
    fn test_invalid_ws_key_length_returns_error() {
        let req = base_req_builder()
            .header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ")
            .build();

        assert!(matches!(handshake(&req), Err(HandshakeError::InvalidWSKey)));
    }

    #[test]
    fn test_without_ws_version_returns_error() {
        let req = http::request::Builder::new()
            .method("GET")
            .path("/test")
            .protocol_name("HTTP")
            .protocol_version("1.1")
            .header("upgrade", "websocket")
            .header("connection", "upgrade")
            .header("sec-websocket-key", FAKE_WS_KEY)
            .build();

        assert!(matches!(
            handshake(&req),
            Err(HandshakeError::InvalidWSVersion)
        ));
    }

    #[test]
    fn test_invalid_ws_version_returns_error() {
        let req = base_req_builder()
            .header("sec-websocket-version", "1")
            .build();

        assert!(matches!(
            handshake(&req),
            Err(HandshakeError::InvalidWSVersion)
        ));
    }

    #[test]
    fn test_invalid_non_utf8_ws_key_returns_error() {
        // This base64 string decodes to 16 bytes containing invalid UTF-8 sequences
        let invalid_utf8_key = "2+/v7+/v7+/v7+/v7+/v7w=="; // decodes to 16 bytes containing 0xFFs

        let req = base_req_builder()
            .header("sec-websocket-key", invalid_utf8_key)
            .build();

        assert!(matches!(handshake(&req), Err(HandshakeError::InvalidWSKey)));
    }
}
