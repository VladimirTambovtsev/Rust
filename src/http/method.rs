use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET, POST, DELETE, PUT, PATCH, HEAD, OPTIONS, TRACE, CONNECT
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "CONNECT" => Ok(Self::CONNECT),
            _ => Err(MethodError)
        }
    }
}


pub struct MethodError;