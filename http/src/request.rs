use std::collections::HashMap;
#[derive(Debug,PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
    UNINITIALIZED,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
	match s {
	    "GET" => Method::GET,
	    "POST" => Method::POST,
	    "PUT" => Method::PUT,
	    "DELETE" =>Method::DELETE,
	    "HEAD" => Method::HEAD,
	    "OPTIONS" => Method::OPTIONS,
	    "PATCH" => Method::PATCH,
	    "TRACE" => Method::TRACE,
	    "CONNECT" => Method::CONNECT,
	    _ => Method::UNINITIALIZED,
	}
    }
}



#[derive(Debug,PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    V3,
    UNINITIALIZED,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
	match s {
	    "HTTP/1.1" => Version::V1_1,
	    _ => Version::UNINITIALIZED,
	}
    }
}
#[derive(Debug)]
pub enum Resource {
    Path(String),
}
pub struct Url {
    protocol: String,
    address: String,
    port: u16,
    path: String,
}


#[derive(Debug)]
pub struct Request {
    method: Method,
    version: Version,
    resource: Resource,
    headers: HashMap<String,String>,
    body: String,
}



impl From<String> for Request {
    fn from(s: String) ->Self {
	let mut method = Method::UNINITIALIZED;
	let mut version = Version::V1_1;
	let mut resource = Resource::Path("".to_string());
	let mut headers: HashMap<String,String> = HashMap::new();
	let mut body = "";
	let (method,resource,version) = parse_first(&s);
	for line in s.lines().skip(1) {
	    if line.contains(':') {
		parse_headers(line,&mut headers);
	    }
	}

	if s.lines().last() != None {
	    body = s.lines().last().unwrap();
	}
	Request{
	    method: method,
	    version: version,
	    resource: resource,
	    headers: headers,
	    body: body.to_string(),
	}
    }
    
}

fn parse_first(s:&str) ->(Method,Resource,Version){
    let mut  line = s.lines();
    let first = line.next();
    if first == None {return (Method::UNINITIALIZED,Resource::Path("".to_string()),Version::UNINITIALIZED)}
    let mut first = first.unwrap();
    if first.contains("HTTP") {
	let mut split = first.split_whitespace();
	let method = Method::from(split.next().unwrap());
	let resource = Resource::Path(split.next().unwrap().to_string());
	let version = Version::from(split.next().unwrap());
	return  (method,resource,version);
    }
    (Method::UNINITIALIZED,Resource::Path("".to_string()),Version::UNINITIALIZED)
}

fn parse_headers(s:&str,h:&mut HashMap<String,String>) ->() {
    let mut a = s.split(':');
    h.insert(a.next().unwrap().to_string(),a.next().unwrap().to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method() {
	let m : Method = Method::from("POST");
	assert_eq!(m,Method::POST);
    }
    #[test]
    fn test_version() {
	let a: Version = Version::from("HTTP/1.1");
	assert_eq!(a,Version::V1_1);
    }

}
