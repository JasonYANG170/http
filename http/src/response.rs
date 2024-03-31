use std::collections::HashMap;
use std::io::{Result,Write};

#[derive(Debug,PartialEq)]
pub struct Response {
    version: String,
    status_code: usize,
    status_text: String,
    headers: Option<HashMap<String,String>>,
    body: Option<String>,
}

impl Default for Response {
    fn default() ->Self {
	Self {
	    version: "HTTP/1.1".to_string(),
	    status_code: 200,
	    status_text: "OK".to_string(),
	    headers: None,
	    body: None,
	}
    }
}

impl Response{
    pub fn new(status_code: usize,
	       headers: Option<HashMap<String,String>>,
	       body: Option<String>
    ) -> Response
    {
	Response{
	    version: "HTTP/1.1".to_string(),
	    status_code: status_code,
	    status_text: match status_code {
		200 => "OK".to_string(),
		400 => "Bad Request".to_string(),
		404 => "Not Found".to_string(),
		500 => "Internal Server Error".to_string(),
		502 => "Bad Gateway".to_string(),
		_ => "Unkonwn Error".to_string(),
	    },
	    headers: match headers {
		Some(_) => headers,
		None => {
		    let mut h: HashMap<String,String> = HashMap::new();
		    h.insert("Content-Type".to_string(),"text/html".to_string());
		    Some(h)
		}
	    },
	    body,
	}
    }
}


impl Response {
    pub fn send(self,stream: &mut impl Write) -> Result<()> {
	let response = String::from(self);
	write!(stream,"{}",response);
	Ok(())
    }
}

impl Response {
    fn version(&self) -> &str {
	&self.version
    }
    fn status_code(&self) ->usize {
	self.status_code
    }
    fn status_text(&self) ->&str {
	&self.status_text
    }
    fn headers(&self) ->String {
	let mut header_string = String::new();
	for (k,v) in self.headers.as_ref().unwrap().into_iter() {
	    header_string = format!("{}{}:{}\r\n",header_string,&k,&v);
	}
	header_string
    }
    fn body(&self) ->&str {
	match self.body.as_ref() {
	    Some(a) => &a,
	    None => "",
	}
    }
}

impl From<Response> for String {
    fn from(response: Response) -> String {
	format!("{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
		response.version(),
		response.status_code(),
		response.status_text(),
		response.headers(),
		response.body().len(),
		response.body()
	)
    }
}
