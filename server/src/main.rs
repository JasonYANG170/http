mod server;
mod parse;
mod router;


pub use http::response::*;
use std::collections::HashMap;
use std::net::TcpListener;
use parse::*;
use server::*;
fn main(){
    let addr: String = config();
    let server: Server  = Server::new(addr);
    server.run();
}
