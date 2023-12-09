//rust 1.17.0 
use std::collections::{HashSet};
use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
    let mut handle = stdin.lock();
	
		let mut seq = Vec::new();
	
		
		loop {
		  let mut buf = String::new();
		  handle.read_line(&mut buf);
		  if buf.len() == 0 {
  			break
  		}
  		//let mut t = buf.trim().clone();
  		seq.push(buf.trim().to_string());
		}
		
		//let mut set = HashSet::new();

		for v in &seq {
			for w in &seq {
				if v.len() != w.len() {
					continue
				}
				let mut nd = 0;
				for (cv,cw) in v.chars().zip(w.chars()) {
					nd += if cv == cw {0} else {1}
				}
				
				if nd == 1 {
					println!("{} {}", v, w)
				}
			}
			
	
	   }
  
}


    