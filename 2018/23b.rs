//rust 1.17.0 
//use std::collections::{BitVec};
use std::io::{self, BufRead};
//use std::hash::Hash;
//use std::cmp;

type Point = (isize, isize, isize);

fn dist(a: &Point, b: &Point) -> isize {
	(a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

fn corners(((x, y, z), r): (Point, isize)) -> Vec<Point> {
	vec![ (x + r, y, z), (x - r, y, z), (x, y + r, z), (x, y - r, z), (x, y, z + r), (x, y, z - r) ]
}

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut seq = Vec::<(Point, isize)>::new();
   loop {
		let mut buf = String::new();
		handle.read_line(&mut buf);
		if buf.len() == 0 {
			break
		}
		let c: Vec<isize> = buf
		   .split(|c:char| !c.is_digit(10) && c != '-')
  		 .filter(|c:&&str| !c.is_empty())
  		 .map(|s| s.parse::<isize>().unwrap())
  		 .collect();
  	 seq.push(((c[0], c[1], c[2]), c[3]));
   }
   //println!("{}", seq.len());
   //seq.truncate(10);

   let mut m = seq.iter().flat_map(|e| corners(*e)).map(|c| {
   	(seq.iter().filter(|(p, r)| dist(&c, p) <= *r).count(), c)
   }).collect::<Vec<(usize, Point)>>();
   
   m.sort();
   m.reverse();
   
   println!("{:?}", &m[0..20]);
   
   for c in corners((m[0].1, 1)) {
   	println!("{:?} {}", c, seq.iter().filter(|(p, r)| dist(&c, p) <= *r).count());
   }
   
   //println!("{}", m.0 + m.1 + m.2);

/*
   let ((maxx, _, _), _) = *seq.iter().max_by_key(|((x, _, _), _)| x).unwrap();
   let ((minx, _, _), _) = *seq.iter().min_by_key(|((x, _, _), _)| x).unwrap();
   let ((_, maxy, _), _) = *seq.iter().max_by_key(|((_, y, _), _)| y).unwrap();
   let ((_, miny, _), _) = *seq.iter().min_by_key(|((_, y, _), _)| y).unwrap();
   let ((_, _, maxz), _) = *seq.iter().max_by_key(|((_, _, z), _)| z).unwrap();
   let ((_, _, minz), _) = *seq.iter().min_by_key(|((_, _, z), _)| z).unwrap();

	   
   println!("{} {} {} {} {} {}", minx, maxx, miny, maxy, minz, maxz);
   
   println!("{}", nr);
*/
}
    
    