//parse command
pub fn config() -> String{
    use std::env;
    use std::env::Args;
    let args: Args = env::args();
    let mut vec: Vec<String> = vec!();
    for args in args {
	vec.push(args);
    }
    if vec.len() > 1 && vec.contains(&"--help".to_string()) {
	help(&vec);
    }

    if vec.len() < 2 {
	help(&vec);
    }
    let server_address: String = env::args().nth(1).unwrap();
    println!("server_address is {:?}",server_address);
    server_address
}

fn help(vec: &Vec<String>) ->(){
    println!("Usage: {} server_address", vec[0]);
    println!("Example: {} 0.0.0.0:3000", vec[0]);
    std::process::exit(0);
}
