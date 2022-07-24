use std::collections::HashMap;


#[derive(Debug,PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s:&str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}
#[derive(Debug,PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match  s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Resource {
    Path(String),
}


pub struct HttpRequest {
    pub method:Method,
    pub veresion: Version,
    pub resource : Resource,
    pub headers: HashMap<String,String>,
    pub msg_body : String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> HttpRequest {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resourece = Resource::Path("index,html".to_string());
        let mut parsed_headers:HashMap<String,String> = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resourece = resource;
            } else if line.contains(":") {
                let (k,v) = parse_header_line(line);
                parsed_headers.insert(k,v);
            } else if line.len() == 0 {
                 
            } else {
                parsed_msg_body = line;
            }


        }

        HttpRequest { method: parsed_method, veresion: parsed_version, resource: parsed_resourece, headers: parsed_headers, msg_body: parsed_msg_body.to_string() }
    }
}


fn process_req_line(req:&str) ->(Method,Resource,Version) {
    let mut words = req.split_whitespace();

    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn parse_header_line(s:&str) -> (String,String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        value = v.to_string();
    }
    (key,value)

}

#[cfg(test)] 
mod tests {

    use super::*;
    #[test]
    fn test_method_into() {
        let m:Method = "GET".into();
        assert_eq!(m,Method::Get);
    }

    #[test]
    fn test_Version_into() {
        let v:Version = "HTTP/1.1".into();
        assert_eq!(v,Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s : String = String::from("GET /greet HTTP/1.1\r\nHost: localhost:9527\r\nAccept: */*\r\nUser-Agent: curl/7.71.1\r\n\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.71.1".into());

        let req: HttpRequest = s.into();

        assert_eq!(Method::Get,req.method);
        assert_eq!(Version::V1_1,req.veresion);
        assert_eq!(Resource::Path("/greet".to_string()),req.resource);
        assert_eq!(headers_expected,req.headers);
    }
}