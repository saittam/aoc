//rust 1.17.0 
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn p0<a, b>(p: &(a, b)) -> &a { &p.0 }
fn p1<a, b>(p: &(a, b)) -> &b { &p.1 }

fn duration(s: &str) -> usize {
	s.chars().next().unwrap() as usize - 'A' as usize + 61
}

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
   let mut underway = HashSet::<String>::new();
   let mut res = String::new();
   let mut workers : [Option<(usize, String)>; 5] = [None, None, None, None, None];
   let mut t = 0;
   
   loop {
   	// Mark completed steps done.
   	for w in workers.iter_mut().filter(|s| s.is_some()) {
   		let (tc, s) = w.clone().unwrap();
   		if t == tc {
   			println!("{}: {} complete", t, s);
   			done.insert(s);
   			*w = None;
   	   }
   	}
   	
   	// Recompute ready set.
   	let mut ready = seq.iter().map(p0)
   	   .chain(seq.iter().map(p1))
   	   .map(String::clone).collect::<HashSet<String>>();
   	for s in done.iter() {
   		ready.remove(s);
   	}
   	for s in underway.iter() {
   		ready.remove(s);
   	}  		
   	for &(ref a, ref b) in seq.iter() {
   		if !done.contains(a) {
   			ready.remove(b);
   		}
   	}
   	
   	// Assign steps to workers.
   	let mut rq = ready.iter().map(String::clone).collect::<Vec<String>>();
   	rq.sort();
   	let mut  rqi = rq.iter();
   	for w in workers.iter_mut().filter(|w| w.is_none()) {
   		match rqi.next() {
   			Some(s) => {
   				*w = Some((t + duration(s), s.clone()));
   				underway.insert(s.clone());
   				println!("{}: {} starting", t, s);
   			},
   			None => break, // no further steps ready
   	   }
   	}
   	
   	// Determine when the next step completes.
   	match workers.iter().filter(|w| w.is_some())
   	   .map(|w| w.clone().unwrap().0).min() {
   	   Some(tc) => t = tc,
   	   None => break,  // all done!
   	}
   		
   	//println!("{:?}", done);
   }
   
   println!("{}", t);
}
