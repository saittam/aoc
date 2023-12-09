//rust 1.17.0
use std::collections::{HashMap};
use std::io::{self, BufRead};
use std::ascii::AsciiExt;

fn matchc(a: char, b: char) -> bool {
   a.to_ascii_lowercase() == b && a == b.to_ascii_uppercase()
}

fn reduce<I>(citer: I) -> String
where I: Iterator<Item = char>
{
   let mut res = String::new();
	for c in citer {
		match res.pop() {
			Some(p) => {
	      	if !matchc(p, c) && !matchc(c, p) {
		      	res.push(p);
		      	res.push(c);
		      }
		   },
		   None => res.push(c),
		}
	}
	return res
}

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();

	let mut buf = String::new();
	handle.read_line(&mut buf);
	
   let (l, c, r, f) = "abcdefghijklmnopqrstuvwxyz".chars().map(
   	|c| {
   		let filtered = buf.trim().chars().filter(|d| d.to_ascii_lowercase() != c).collect::<String>();
   		let reduced = reduce(filtered.chars());
   		(reduced.len(), c, reduced, filtered)
   	}).min().unwrap();
 
   println!("{}", l);
   println!("{}", c);
   println!("{}", r);
   println!("{}", f);
   
}


    
    