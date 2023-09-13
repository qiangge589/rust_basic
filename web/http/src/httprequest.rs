// Http解析
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_0,
    V1_1,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.0" => Version::V1_0,
            "HTTP/1.1" => Version::V1_1, // Add this line to correctly match the version
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    // Path
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_headers_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                // Skip empty line
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_owned(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_headers_line(s: &str) -> (String, String) {
    let mut headers_items = s.split(":");
    let mut key = String::from(":");
    let mut value = String::from(":");
    if let Some(k) = headers_items.next() {
        key = k.to_string();
    }
    if let Some(v) = headers_items.next() {
        value = v.to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from() {
        let m: Method = Method::from("GET"); // "GET".into() also works
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_from() {
        let v: Version = Version::from("HTTP/1.0");
        assert_eq!(v, Version::V1_0);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost:localhost:8080\r\nUser-Agent:curl/7.64.1\r\nAccept:*/*\r\n\r\n");
        let mut headers = HashMap::new();
        headers.insert("Host".into(), "localhost".into());
        headers.insert("Accept".into(), "*/*".into());
        headers.insert("User-Agent".into(), "curl/7.64.1".into());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers, req.headers);
        assert_eq!("", req.msg_body); // Check the message body
    }
}