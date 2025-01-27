use std::collections::HashMap;
use std::io::{self, BufRead};
use std::net;

pub struct Request {
    method: String,
    path: String,
    protocol_name: String,
    protocol_version: String,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn is_get(&self) -> bool {
        return self.method == "GET";
    }

    pub fn is_http(&self) -> bool {
        return self.protocol_name == "HTTP";
    }

    pub fn meets_version(&self, version: &str) -> bool {
        return self.protocol_version == version;
    }

    pub fn header(&self, key: &str) -> Option<&String> {
        let k = key.to_lowercase();
        return self.headers.get(&k);
    }

    pub fn from_raw(s: &net::TcpStream) -> Request {
        let buf = io::BufReader::new(s);
        let req: Vec<String> = buf
            .lines()
            .map(|l| l.unwrap())
            .take_while(|l| !l.is_empty())
            .collect();

        let status_line = req[0]
            .split_whitespace()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let protocol = status_line[2].split("/").collect::<Vec<&str>>();

        let mut headers = HashMap::new();

        for header in req[1..].iter() {
            let parts = header.split(":").collect::<Vec<&str>>();
            let key = parts[0].to_lowercase().to_string();
            let value = parts[1].trim().to_string();
            headers.insert(key, value);
        }

        return Request {
            method: status_line[0].clone(),
            path: status_line[1].clone(),
            protocol_name: protocol[0].to_string(),
            protocol_version: protocol[1].to_string(),
            headers,
        };
    }
}

pub struct Builder {
    method: String,
    path: String,
    protocol_name: String,
    protocol_version: String,
    headers: HashMap<String, String>,
}

impl Builder {
    pub fn new() -> Builder {
        return Builder {
            method: String::new(),
            path: String::new(),
            protocol_name: String::new(),
            protocol_version: String::new(),
            headers: HashMap::new(),
        };
    }

    pub fn method(mut self, method: &str) -> Builder {
        self.method = method.to_string();
        return self;
    }

    pub fn path(mut self, path: &str) -> Builder {
        self.path = path.to_string();
        return self;
    }

    pub fn protocol_name(mut self, protocol_name: &str) -> Builder {
        self.protocol_name = protocol_name.to_string();
        return self;
    }

    pub fn protocol_version(mut self, protocol_version: &str) -> Builder {
        self.protocol_version = protocol_version.to_string();
        return self;
    }

    pub fn header(mut self, key: &str, value: &str) -> Builder {
        self.headers.insert(key.to_string(), value.to_string());
        return self;
    }

    pub fn build(self) -> Request {
        return Request {
            method: self.method,
            path: self.path,
            protocol_name: self.protocol_name,
            protocol_version: self.protocol_version,
            headers: self.headers,
        };
    }
}
