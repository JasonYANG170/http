[简体中文](README_zh.md) | [English](README.org)

# Http

## 使用

### 将字符串转换为 Request / Response

```rust
use http::response::*;
use http::request::*;
let listener = TcpListener::bind(&self.addr).unwrap();
println!("Server Binding On {}",self.addr);
for stream in listener.incoming() {
    let mut stream = stream.unwrap();
    let mut buffer = [0;8000];
	    stream.read(&mut buffer).unwrap();
	    let mut buffer_vec = buffer.to_vec();
	    buffer_vec.retain(|&x| x != 0);
	    let request: Request = String::from_utf8(buffer_vec).unwrap().into();
	    println!("[Connection] {} {:?}",stream.peer_addr().unwrap(),request);
	    stream.write(&mut buffer);
```
