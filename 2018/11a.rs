//rust 1.17.0 
use std::collections::{HashSet};
use std::io::{self, BufRead};
use std::ops::{Add, Mul};
use std::hash::Hash;
use std::cmp;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Add for Point<T>
where T: std::marker::Copy, T: Add<Output = T> {
	type Output = Point<T>;
	fn add(self, other: Point<T>) -> Point<T> {
		Point { x: self.x + other.x, y: self.y + other.y }
	}
}

impl Mul<Point<i32>> for i32 {
	type Output = Point<i32>;
	fn mul(self, p: Point<i32>) -> Point<i32> {
		Point { x: self * p.x, y: self * p.y }
	}
}

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut seq = Vec::<(Point<i32>, Point<i32>)>::new();
   loop {
		let mut buf = String::new();
		handle.read_line(&mut buf);
		if buf.len() == 0 {
			break
		}
		let mut c : Vec<i32> = buf
		   .split(|c:char| !c.is_digit(10) && c != '-')
  		 .filter(|c:&&str| !c.is_empty())
  		 .map(|s| s.parse::<i32>().unwrap())
  		 .collect();
  	 seq.push((Point{ x: c[0], y: c[1]},
  	 	       Point{ x: c[2], y: c[3]}));
   }
   
   // compute the approximate center
   let xmin = seq.iter().min_by_key(|p| p.0.x).unwrap();
   let xmax = seq.iter().max_by_key(|p| p.0.x).unwrap();
   let approx = (xmax.0.x - xmin.0.x) / (xmin.1.x - xmax.1.x);
   
   // try steps around approx to find the one that minimizes bounding box
   let mut best = i32::max_value();
   let mut btl = Point{ x: 0, y: 0 };
   let mut bbr = Point{ x: 0, y: 0 };
   let mut step = 0;
   for i in (approx - 100)..(approx + 100) {
   	let mut tl = Point{ x: i32::max_value(), y: i32::max_value() };
   	let mut br = Point{ x: i32::min_value(), y: i32::min_value() };
   	
   	for p in seq.iter().map(|&(p, v)| p + i as i32 * v) {
   		tl.x = cmp::min(tl.x, p.x);
   		tl.y = cmp::min(tl.y, p.y);
   		br.x = cmp::max(br.x, p.x);
   		br.y = cmp::max(br.y, p.y);
   	}
   	
   	let c = br.y - tl.y;
   	if c < best {
   		println!("{} {} in {:?} {:?}", i, c, tl, br);
   		best = c;
   		btl = tl;
   		bbr = br;
   		step = i;
   	}
   }
   
   println!("best {} in {:?} {:?}", step, btl, bbr);
   
   // render
   let points = seq.iter()
      .map(|&(p, v)| p + step as i32 * v)
      .collect::<HashSet<Point<i32>>>();
      
   println!("{:?}", points);
      
   for y in btl.y..(bbr.y + 1) {
   	let mut res = String::new();
   	for x in btl.x..(bbr.x + 1) {
   		res.push(
   			if points.contains(&Point{ x: x, y: y }) {
   				'#'
   			} else {
   				' '
   			});
   	}
   		
   	println!("{}", res);
   }

}
    
