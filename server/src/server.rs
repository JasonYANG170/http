use std::net::TcpListener;
use std::io::{Read,Write};
use http::request::*;
use std::time::*;
use super::router::*;
pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
	Server {addr}
    }
    pub fn run(&self) ->() {
	let listener = TcpListener::bind(&self.addr).unwrap();
	println!("Server Binding On {}",self.addr);

	for stream in listener.incoming() {
	    let mut stream = stream.unwrap();


	    let mut buffer = [0;800];
	    stream.read(&mut buffer).unwrap();
	    let mut buffer_vec = buffer.to_vec();
	    buffer_vec.retain(|&x| x != 0);
	    let request: Request = String::from_utf8(buffer_vec).unwrap().into();
	    println!("[Connection] {} {:?}",stream.peer_addr().unwrap(),request);
	    stream.write(&mut buffer);
//	    Router::route(request,&mut stream);
	}
    }
}
