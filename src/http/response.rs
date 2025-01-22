use std::collections::HashMap;

use super::status_code::StatusCode;

pub struct Builder {
    status: Option<StatusCode>,
    headers: HashMap<String, String>,
}

impl Builder {
    pub fn status(mut self, code: StatusCode) -> Builder {
        self.status = Some(code);
        return self;
    }

    pub fn header(mut self, key: String, value: String) -> Builder {
        self.headers.insert(key, value);
        return self;
    }

    pub fn build(self) -> String {
        if self.status.is_none() {
            panic!("`status` must be specified before build");
        }

        let mut response = String::new();

        return response;
    }

    pub fn new() -> Builder {
        return Builder {
            status: None,
            headers: HashMap::new(),
        };
    }
}
