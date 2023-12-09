//rust 1.17.0 
use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead};
//use std::hash::Hash;
//use std::cmp;
use std::cmp::Ordering;

type Point = (isize, isize, isize);

#[derive(PartialEq, Eq, Debug)]
enum Boundary {
	Lower(isize),
	Upper(isize),
}
use Boundary::*;

impl Boundary {
	fn value(&self) -> isize {
		match self {
			Lower(v) | Upper(v) => *v
		}
	}
}

impl Ord for Boundary {
	fn cmp(&self, other: &Boundary) -> Ordering {
		match self.value().cmp(&other.value()) {
			Ordering::Equal => match (self, other) {
				(Lower(_), Upper(_)) => Ordering::Less,
				(Upper(_), Lower(_)) => Ordering::Greater,
				_ => Ordering::Equal,
			},
			o => o,
		}
	}
}

impl PartialOrd for Boundary {
	fn partial_cmp(&self, other: &Boundary) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

const DIR: [Point; 4] = [
   (1, -1, -1),
   (1, -1, 1),
   (1, 1, -1),
   (1, 1, 1),
];

const CD: [(usize, usize, usize); 4] = [
   (0, 1, 2),
   (1, 0, 3),
   (2, 3, 0),
   (3, 2, 1),
];

fn neg(p: &Point) -> Point {
	(-p.0, -p.1, -p.2)
}

fn add(a: &Point, b: &Point) -> Point {
	(a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn scl(a: &Point, b: &Point) -> isize {
	a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

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
   
   //println!("{:?}", seq);
   
   let bs = seq.iter().map(|((x, y, z), r)| {
   	   let mut bp = [(0, 0); 4];
   	   for (i, d) in DIR.iter().enumerate() {
   	   	bp[i] = (scl(&(x - r, *y, *z), d),
   	   	         scl(&(x + r, *y, *z), d));
   	   }
   	   return bp;
   	}).collect::<Vec<[(isize, isize); 4]>>();
   	
   //println!("{:?}", bs);
   
   let mut boundaries = bs.iter().enumerate().fold(
   	[vec![], vec![], vec![], vec![]],
   	|mut v, (i, a)| {
   		for (di, (l, u)) in a.iter().enumerate() {
   			v[di].push((Lower(*l), i));
   			v[di].push((Upper(*u), i));
   		}
   		v
   	});
   for v in boundaries.iter_mut() {
   	v.sort();
   }
 
   let mut md = Vec::<((isize, isize), HashSet<usize>)>::new();
   for b in boundaries.iter() {
   	let (lh, s, _, _) = b.iter().fold(
   		((0, 0), HashSet::<usize>::new(), 0, HashSet::<usize>::new()),
   		|(mut lh, mut m, mut cl, mut c), (b, i)| {
   			match b {
   				Lower(v) => {
   					c.insert(*i);
   					cl = *v;
   				},
   				Upper(v) => {
   					//println!("{:?}", c);
   					if c.len() > m.len() {
   						m = c.clone();
   						lh = (cl, *v);
   					}
   					c.remove(i);
   				},
   			}
   			(lh, m, cl, c)
   		});
   		md.push((lh, s));
   }
   	
   let mut is = (0..seq.len()).collect::<HashSet<usize>>();
   for ((l, h), m) in md.iter() {
   	is = is.intersection(m).cloned().collect();
   	println!("{} {} {}", m.len(), l, h);
   }
   println!("{}", is.len());
   
   let bps = md.iter().map(|(p, _)| p).cloned().collect::<Vec<(isize, isize)>>();
   let bp = [
      [ bps[0].0, bps[1].0, bps[2].0, bps[3].0 ],
      [ bps[0].1, bps[1].1, bps[2].1, bps[3].1 ],
      [ bps[0].0, bps[1].0, bps[2].1, bps[3].1 ],
      [ bps[0].1, bps[1].0, bps[2].0, bps[3].1 ],
      [ bps[0].1, bps[1].1, bps[2].0, bps[3].0 ],
      [ bps[0].0, bps[1].1, bps[2].1, bps[3].0 ],
   ];
   let mut tp = Vec::<Point>::new();
   for b in bp.iter() {
   	for (d1, d2, d3) in CD.iter().cloned() {
   		let z = DIR[d1].2 * (b[d1] - b[d2]) / 2;
   		let y = DIR[d1].1 * (b[d1] - b[d3]) / 2;
   		let x = b[d1] - y * DIR[d1].1 - z * DIR[d1].2;
   		tp.push((x, y, z));
   	}
   }
   
   let minx = tp.iter().map(|(x, y, z)| x).min().unwrap();
   let maxx = tp.iter().map(|(x, y, z)| x).max().unwrap();
   let miny = tp.iter().map(|(x, y, z)| y).min().unwrap();
   let maxy = tp.iter().map(|(x, y, z)| y).max().unwrap();
   let minz = tp.iter().map(|(x, y, z)| z).min().unwrap();
   let maxz = tp.iter().map(|(x, y, z)| z).max().unwrap();
   
   let mut cd = HashMap::<Point, (usize, isize)>::new();
   for x in (minx - 1)..(maxx + 2) {
   	for y in (miny - 1)..(maxy + 2) {
   		for z in (minz - 1)..(maxz + 2) {
   			let c = (x, y, z);
   			let nr = seq.iter().filter(|(p, r)| dist(&c, p) <= *r).count();
   			cd.insert(c, (nr, dist(&c, &(0, 0, 0))));
   		}
   	}
   }
   
   let nm = cd.values().map(|(n, _)| n).max().unwrap();
   let mind = cd.values().filter(|(n, _)| n == nm).min().unwrap();
   println!("{:?}", mind);
}
    
    