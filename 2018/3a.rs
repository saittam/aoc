//rust 1.17.0 
use std::collections::{HashSet};
use std::io::{self, BufRead};

struct Claim {
	id: u32,
	left: u32,
	top: u32,
	width: u32,
	height: u32,
}

fn main() {
	let stdin = io::stdin();
    let mut handle = stdin.lock();
	
		let mut seq = Vec::<Claim>::new();
	
		
		loop {
		  let mut buf = String::new();
		  handle.read_line(&mut buf);
		  if buf.len() == 0 {
  			break
  		}
  		let mut comps : Vec<u32> = buf
        	.split(|c:char| !c.is_digit(10))
  		   .filter(|c:&&str| !c.is_empty())
  		   .map(|c:&str| c.parse::<u32>().unwrap())
  		   .collect();
  		seq.push(Claim {
  			id: comps[0],
  			left: comps[1],
  			top: comps[2],
  		   width: comps[3],
  		   height: comps[4],
  		});
		}
		
		let mut claims = [[0; 1000]; 1000];
		let mut nc = 0;

		for c in &seq {
			for x in c.left..c.left+c.width {
				for y in c.top..c.top+c.height {
					nc += claims[x as usize][y as usize];
					claims[x as usize][y as usize] = 1;
				}
			}
	   }
      println!("{}", nc)
}


    