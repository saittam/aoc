//rust 1.17.0 
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn p0<a, b>(p: &(a, b)) -> &a { &p.0 }
fn p1<a, b>(p: &(a, b)) -> &b { &p.1 }

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut seq = Vec::<(String, String)>::new();
   loop {
		let mut buf = String::new();
		handle.read_line(&mut buf);
		if buf.len() == 0 {
			break
		}
		let mut c : Vec<&str> = buf
		   .split(|c:char| !c.is_alphanumeric())
  		 .filter(|c:&&str| !c.is_empty())
  		 .collect();
  	 seq.push((c[1].to_string(), c[7].to_string()));
   }
   
   let mut done = HashSet::<String>::new();
   let mut res = String::new();
   
   loop {
   	let mut ready = seq.iter().map(p0)
   	   .chain(seq.iter().map(p1))
   	   .map(String::clone).collect::<HashSet<String>>();
   	for s in done.iter() {
   		ready.remove(s);
   	}   		   		
   	for &(ref a, ref b) in seq.iter() {
   		if !done.contains(a) {
   			ready.remove(b);
   		}
   	}
   	println!("{:?}", done);
   	println!("{:?}", ready);
   	match ready.iter().min() {
   		Some(s) => {
   			res.push_str(s);
   			done.insert(s.clone());
   		},
   		None => break
   	}
   }
   
   println!("{}", res);
}


    
    
    