//rust 1.17.0 
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

type Pos = (i32, i32);

fn d(a: &Pos, b: &Pos) -> i32 {
	(b.0 - a.0).abs() + (b.1 - a.1).abs()
}

fn p0 (p: &Pos) -> i32 { p.0 }
fn p1 (p: &Pos) -> i32 { p.1 }

/*
fn cartesian<I1, I2>(i1: I1, i2: I2)
   -> std::iter::Iterator<Item = (I1::Item, I2::Item)>
where
   I1: std::iter::Iterator,
   I2: std::iter::Iterator,
{
	i1.fold(std::iter::empty::<(I1::Item, I2::Item)>(),
		|i, v1| i.chain(i2.cloned().map(|v2| (v1, v2))))
}
*/

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut seq = Vec::<(i32, i32)>::new();
   loop {
		let mut buf = String::new();
		handle.read_line(&mut buf);
		if buf.len() == 0 {
			break
		}
		let mut c : Vec<i32> = buf
		   .split(|c:char| !c.is_digit(10))
  		 .filter(|c:&&str| !c.is_empty())
  		 .map(|c| c.parse::<i32>().unwrap())
  		 .collect();
  	 seq.push((c[0], c[1]));
   }
   
   let min = (seq.iter().map(p0).min().unwrap(),
   	        seq.iter().map(p1).min().unwrap());
   let max = (seq.iter().map(p0).max().unwrap(),
   	        seq.iter().map(p1).max().unwrap());

   let mut count = 0;
   
   for x in min.0..max.0 {
   	for y in min.1..max.1 {
   		let sd : i32 = seq.iter().map(|p| d(&(x, y), p)).sum();
   		if sd < 10000 {
   		   count += 1
   		}
   	}
   }
   
   println!("{}", count);
}


    
    