#[derive(Debug)]
pub enum HandshakeError {
    InvalidMethod,
    InvalidProtocol,
    InvalidProtocolVersion,
    InvalidUpgrade,
    InvalidConnection,
    InvalidWSKey,
    InvalidWSVersion,
}

impl std::fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HandshakeError::InvalidMethod => {
                write!(f, "only GET requests are allowed for websocket handshake")
            }
            HandshakeError::InvalidProtocol => {
                write!(f, "only HTTP/1.1 is allowed for websocket handshake")
            }
            HandshakeError::InvalidProtocolVersion => {
                write!(f, "only HTTP/1.1 is allowed for websocket handshake")
            }
            HandshakeError::InvalidUpgrade => {
                write!(
                    f,
                    "`upgrade` header must include `websocket` token for websocket handshake"
                )
            }
            HandshakeError::InvalidConnection => {
                write!(
                    f,
                    "`connection` header must include `upgrade` token for websocket handshake"
                )
            }
            HandshakeError::InvalidWSKey => {
                write!(f, "`sec-websocket-key` header must be a valid UTF-8 16 bytes long base64 string for websocket handshake")
            }
            HandshakeError::InvalidWSVersion => {
                write!(
                    f,
                    "`sec-websocket-version` header must be `13` for websocket handshake"
                )
            }
        }
    }
}
