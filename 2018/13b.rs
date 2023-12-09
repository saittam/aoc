//rust 1.17.0 
use std::collections::{HashMap, VecDeque};
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Cart {
	Active(Point<i32>, Point<i32>, Turn),
	Crashed(Point<i32>),
}

use Cart::{Active, Crashed};

type CartVec = Vec<Cart>;

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut carts: CartVec = Vec::new();
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
				'^' => carts.push(Active(p, Point::<i32>::new(0, -1), Turn::Left)),
				'>' => carts.push(Active(p, Point::<i32>::new(1, 0), Turn::Left)),
				'v' => carts.push(Active(p, Point::<i32>::new(0, 1), Turn::Left)),
				'<' => carts.push(Active(p, Point::<i32>::new(-1, 0), Turn::Left)),
				_ => (),
			}
		}
   }

   let mut remaining = carts.len();
   while remaining > 1 {
   	carts.sort();
   	
   	for i in 0..carts.len() {
   	   let (p, v, t) = match carts[i] {
   	   	Active(p, v, t) => (p, v, t),
   	   	Crashed(p) => continue,
   	   };
   		let np = p + v;

         let mut collision = false;
         for c in carts.iter_mut() {
         	match *c {
         		Active(op, _, _) => {
         			if np == op {
         				collision = true;
         				*c = Crashed(np);
         			}
         		},
         		_ => (),
         	}
         }
         
         if collision {
   		   println!("collision at {:?}", np);
   		   carts[i] = Crashed(np);
   		   remaining -= 2;
   		   continue;
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
   		carts[i] = Active(np, nv, nt);
      }
   }
   
   println!("{:?}", carts);
}
    
    