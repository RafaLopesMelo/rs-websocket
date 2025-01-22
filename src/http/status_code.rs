pub struct StatusCode(u16);

impl StatusCode {
    pub const SWITCHING_PROTOCOLS: StatusCode = StatusCode(101);

    pub fn code(&self) -> u16 {
        return self.0;
    }

    pub fn text(&self) -> String {
        match self.0 {
            101 => "Switching Protocols".to_string(),
            _ => panic!("status code not configured"),
        }
    }
}
