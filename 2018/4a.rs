//rust 1.17.0 
use std::collections::{HashMap};
use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
    let mut handle = stdin.lock();
	
		let mut seq = Vec::<String>::new();
	
		
		loop {
		  let mut buf = String::new();
		  handle.read_line(&mut buf);
		  if buf.len() == 0 {
  			break
  		}
  		seq.push(buf);
  	}
  	
  	seq.sort();
  	
  	let mut total = HashMap::<u32, u32>::new();
  	let mut state = HashMap::<u32, [u32; 60]>::new();
  	let mut id : u32 = 0;
  	let mut start : u32 = 0;
  	
  	/*	
  	let update = |end| {
  			*total.entry(id).or_insert(0) += end - start;
  			for m in start..end {
  				let r = state.entry(id).or_insert([0; 60]);
  				(*r)[m as usize] += 1;
  			}
  		};
  		*/
  		
  	for l in seq {
  		let mut c : Vec<&str> = l
        	.split(|c:char| !c.is_alphanumeric())
  		   .filter(|c:&&str| !c.is_empty())
  		   .collect();
  		println!("{:?}", c);
  		let minute = c[4].parse::<u32>().unwrap();
  		match c[5] {
  			"Guard" => id = c[6].parse::<u32>().unwrap(),
  			"falls" => start = minute,
  			"wakes" => {
  				*total.entry(id).or_insert(0) += minute - start;
  			   for m in start..minute {
  				   let r = state.entry(id).or_insert([0; 60]);
  			   	(*r)[m as usize] += 1;
  			   }
  			},
  			_ => ()
  		}
		}
		
		//println!("{:?}", total);
		let (maxidx, maxtotal) = total.iter().max_by_key(|&(&i, &v)| v).unwrap();
		println!("{} {}", maxidx, maxtotal);
		let (maxmin, maxsleep) = state[maxidx].iter().enumerate().max_by_key(|&(i, &v)| v).unwrap();
 	  println!("{} {}", maxmin, maxsleep);
      println!("{}", maxidx * maxmin as u32);
}


    