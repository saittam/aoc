//rust 1.17.0 
use std::collections::{HashMap};
use std::io::{self, BufRead};
use std::ops::{Add, Mul};
use std::hash::Hash;
use std::cmp;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn new(x: T, y: T) -> Point<T> {
		Point{ x: x, y: y }
	}
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Rail { S, R, L, X }

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Turn { Right, Straight, Left }

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut carts = Vec::<(Point<i32>, Point<i32>, Turn)>::new();
   let mut map = HashMap::<Point<i32>, Rail>::new();
   for y in 0.. {
		let mut buf = String::new();
		handle.read_line(&mut buf);
		if buf.len() == 0 {
			break
		}
		for (x, c) in buf.chars().enumerate() {
			let p = Point::<i32>::new(x as i32, y as i32);
			match c {
				'/' => { map.insert(p, Rail::L); () },
				'\\' => { map.insert(p, Rail::R); () },
				'+' => { map.insert(p, Rail::X); () },
				'^' => carts.push((p, Point::<i32>::new(0, -1), Turn::Left)),
				'>' => carts.push((p, Point::<i32>::new(1, 0), Turn::Left)),
				'v' => carts.push((p, Point::<i32>::new(0, 1), Turn::Left)),
				'<' => carts.push((p, Point::<i32>::new(-1, 0), Turn::Left)),
				_ => (),
			}
		}
   }
   
   'o: loop {
   	let mut ncarts = Vec::<(Point<i32>, Point<i32>, Turn)>::new();
   	carts.sort();
   	{
   	let mut citer = (&carts).into_iter();
   		
   	loop {
   	   let &(p, v, t) = match citer.next() {
   	   	Some(e) => e,
   	   	None => break,
   	   };
   		let np = p + v;

   		if citer.clone().chain((&ncarts).into_iter())
   		   .filter(|&&(op, _, _)| np == op)
   		   .count() > 0 {
   		   println!("collision at {:?}", np);
   		   break 'o;
   		}
   		
   		let r = *map.get(&np).unwrap_or(&Rail::S);
   		let (nv, nt) = match (r, v, t) {
   			(Rail::S, _, _) =>
   			   (v, t),
   			(Rail::L, Point{ x: vx, y: vy }, _) =>
   			   (Point::new(-vy, -vx), t),
   			(Rail::R, Point{ x: vx, y: vy }, _) =>
   			   (Point::new(vy, vx), t),
   			(Rail::X, Point{ x: vx, y: vy }, Turn::Left) =>
   			   (Point::new(vy, -vx), Turn::Straight),
   			(Rail::X, _, Turn::Straight) =>
   			   (v, Turn::Right),
   			(Rail::X, Point{ x: vx, y: vy }, Turn::Right) =>
   			   (Point::new(-vy, vx), Turn::Left),
   		};
   		ncarts.push((np, nv, nt));
      }
      }
      carts = ncarts;
      
      //println!("{:?}", carts);
   }
   		
   
}
    
    