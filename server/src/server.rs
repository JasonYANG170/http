//use std::net::TcpListener;
use tokio::net::TcpListener;
//use std::io::{Read,Write};
use tokio::io::{AsyncRead,AsyncWrite};
use http::request::*;
use http::response::*;
use std::time::*;
use super::router::*;
pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
	Server {addr}
    }
    pub async fn run(&self) ->() {
	let listener = TcpListener::bind(&self.addr).await.unwrap();
	println!("Server Binding On {}",self.addr);
	loop{
	    let  (mut stream,peer_addr) = listener.accept().await.unwrap();
	    tokio::task::spawn(async move {
		stream.readable().await.unwrap();
		let mut buffer: [u8;8000] = [0;8000];
		stream.try_read(&mut buffer).unwrap();
		let mut buffer_vec = buffer.to_vec();
		buffer_vec.retain(|&x| x != 0);
		let request: Request = String::from_utf8(buffer_vec).unwrap().into();
		let response: String = Response::new(200,None,Some(format!("hello {}", peer_addr).to_string())).into();
		stream.try_write(&response.to_string().into_bytes());
		println!("[Reponse] {} {:?}",peer_addr,response);
//		println!("[Connection] {} {:?}",peer_addr,request);
		
	    });

	}
    }
}
