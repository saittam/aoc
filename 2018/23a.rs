//rust 1.17.0 
//use std::collections::{HashSet};
use std::io::{self, BufRead};
//use std::hash::Hash;
//use std::cmp;

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut seq = Vec::<((isize, isize, isize), isize)>::new();
   loop {
		let mut buf = String::new();
		handle.read_line(&mut buf);
		if buf.len() == 0 {
			break
		}
		let mut c: Vec<isize> = buf
		   .split(|c:char| !c.is_digit(10) && c != '-')
  		 .filter(|c:&&str| !c.is_empty())
  		 .map(|s| s.parse::<isize>().unwrap())
  		 .collect();
  	 seq.push(((c[0], c[1], c[2]), c[3]));
   }
   
   let mut nr = 0;
   let ((mx, my, mz), mr) = *seq.iter().max_by_key(|(_, r)| r).unwrap();
   for ((x, y, z), _) in seq {
   	if (x - mx).abs() + (y - my).abs() + (z - mz).abs() <= mr {
   		nr += 1;
   	}
   }
   
   println!("{}", nr);
}
    
