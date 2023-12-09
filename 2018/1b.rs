//rust 1.17.0 
use std::collections::{HashSet};
use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
    let mut handle = stdin.lock();
	
		let mut seq = Vec::new();
		let mut seen = HashSet::new();
		let mut s = 0;
		
		loop {
		  let mut buf = String::new();
		  handle. read_line(&mut buf);
		  match buf.trim().parse::<i32>() {
  			Ok(val) => seq.push(val),
  			Err(e) => break
  		}
		}
		for v in seq.iter().cycle() {
			if seen.contains(&s) {
				println!("already seen: {}", s)
			}
			
			seen.insert(s);
			s += *v;
			println!("s: {}", s)
	}
  
}


    