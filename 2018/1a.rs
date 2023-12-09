//rust 1.17.0 
use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
    let mut handle = stdin.lock();
	
		
		let mut s = 0;
		loop {
			let mut buf = String::new();
		
		handle. read_line(&mut buf);
		//println!("{}", buf);
		s += buf.trim().parse::<i32>().unwrap();
		println!("{}", s)
	}
  
}


