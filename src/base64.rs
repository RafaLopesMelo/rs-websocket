use base64::engine::general_purpose::STANDARD;
use base64::Engine;

#[derive(Debug)]
pub struct Base64Error {}

impl Base64Error {
    const fn new() -> Self {
        Self {}
    }
}

impl std::fmt::Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid UTF-8 base64 input provided")
    }
}

pub fn decode(input: &String) -> Result<String, Base64Error> {
    let decoded = STANDARD.decode(input).map_err(|_| Base64Error::new())?;
    return String::from_utf8(decoded).map_err(|_| Base64Error::new());
}

pub fn encode(input: &[u8]) -> String {
    return STANDARD.encode(input);
}
