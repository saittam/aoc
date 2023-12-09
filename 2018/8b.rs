//rust 1.17.0
use std::collections::{HashMap};
use std::io::{self, BufRead};

fn parse<'a, 'b, I>(mut iter: &'a mut I) -> u32
where I: std::iter::Iterator<Item = &'b u32> {
	let node_count = *(iter.next().unwrap());
	let md_count = *(iter.next().unwrap());
	
	if node_count == 0 {
		return iter.take(md_count as usize).sum();
	}
	
	let nval: Vec<u32> = (0..node_count)
	   .map(|_| parse(iter))
	   .collect();
	//println!("{:?}", nval);
	let zero: u32 = 0;
   return iter.take(md_count as usize)
      .map(|i| nval.get(*i as usize - 1).unwrap_or(&zero))
      .sum();
}

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();

	let mut buf = String::new();
	handle.read_line(&mut buf);
	let seq = buf.split(" ")
	   .map(|s| s.parse::<u32>().unwrap())
	   .collect::<Vec<u32>>();

   println!("{}", parse(&mut seq.iter()));
}


    
    
