use std::collections::HashMap;
use std::net::*;
pub struct Request {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

pub struct Response {
    version: String,
    status: u32,
    reason: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    pub fn new(host: &str) -> Request {
        let mut req = Request {
            method: "GET".to_string(),
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };
        req.headers.insert("Host".to_string(), host.to_string());
        req
    }
    pub fn build(&self) -> Vec<u8> {
        let mut req:Vec<u8> = Vec::new();
        let mut header = self.method.clone();
        header = header + " " + &self.path + " " + &self.version + "\r\n";
        header = header + "Host: " + self.headers.get("Host").unwrap() + "\r\n";
        for (key, value) in self.headers.iter() {
            if key == "Host" {
                continue;
            }
            header = header + key + ": " + value + "\r\n";
        }
        header += "\r\n";
        let mut t = header.as_bytes().to_vec();
        let mut b = self.body.clone();
        req.append(&mut t);
        req.append(&mut b);
        req
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    //pub fn perform<A:ToSocketAddrs>(&self, addr: A) -> Result<Response> {
    //    let mut stream = TcpStream::connect(addr)?;
    //}
}
