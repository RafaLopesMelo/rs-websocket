use std::collections::HashMap;

pub struct Builder {
    status: Option<StatusCode>,
    headers: HashMap<String, String>,
}

impl Builder {
    pub fn status(mut self, code: StatusCode) -> Builder {
        self.status = Some(code);
        return self;
    }

    pub fn header(mut self, key: &str, value: &str) -> Builder {
        self.headers.insert(key.to_string(), value.to_string());
        return self;
    }

    pub fn build(self) -> Vec<u8> {
        let status = self
            .status
            .expect("`status` must be specified before build");

        let mut response = String::new();

        let status_line = format!("HTTP/1.1 {} {}\r\n", status.code(), status.text());
        response.push_str(status_line.as_str());

        for (key, value) in self.headers {
            let header = format!("{}: {}\r\n", key, value);
            response.push_str(header.as_str());
        }

        return response.into_bytes();
    }

    pub fn new() -> Builder {
        return Builder {
            status: None,
            headers: HashMap::new(),
        };
    }
}

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
