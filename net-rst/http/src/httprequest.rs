use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized
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
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_into() {
        let get: Method = "GET".into();
        assert_eq!(get, Method::Get);
    }

    #[test]
    fn test_post_into() {
        let post: Method = "POST".into();
        assert_eq!(post, Method::Post);
    }

    #[test]
    fn test_method_uninitialized_into() {
        let un: Method = "test".into();
        assert_eq!(un, Method::Uninitialized);
    }

    #[test]
    fn test_http11_into() {
        let http11: Version = "HTTP/1.1".into();
        assert_eq!(http11, Version::V1_1);
    }

    #[test]
    fn test_http20_into() {
        let http20: Version = "HTTP/2.0".into();
        assert_eq!(http20, Version::V2_0);
    }

    #[test]
    fn test_http_uninitialized_into() {
        let un: Version = "test".into();
        assert_eq!(un, Version::Uninitialized);
    }
}