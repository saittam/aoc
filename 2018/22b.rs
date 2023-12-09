use std:: collections::{BinaryHeap, HashSet, HashMap};

const DEPTH: usize = 3198;
const T: (isize, isize) = (12, 757);
const W: usize = 2000;
const H: usize = 3000;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Type { Rocky, Wet, Narrow }
use Type::*;
const TM: [Type; 3] = [ Rocky, Wet, Narrow ];

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Gear { Hands, Torch, Climb }
use Gear::*;

fn compat(t: Type, g: Gear) -> bool {
	match (t, g) {
		(Rocky, Hands) => false,
		(Wet, Torch) => false,
		(Narrow, Climb) => false,
		_ => true,
	}
}

fn neighbors((x, y): (isize, isize)) -> [(isize, isize); 4] {
	[ (x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1) ]
}

fn mtype(m: &[[usize; W]; H], (x, y): (isize, isize)) -> Option<Type> {
	m.get(y as usize).and_then(|r| r.get(x as usize)).map(|e| TM[e % 3])
}

fn bound((x, y): (isize, isize), g: Gear) -> isize {
	(T.0 - x).abs() + (T.1 - y).abs() + if g == Torch { 0 } else { 7 }
}

fn main() {
	let mut m = [[0; W]; H];
	
	m[0][0] = (0 + DEPTH) % 20183;
   m[T.1 as usize][T.0 as usize] = (0 + DEPTH) % 20183;
	for x in 1..W {
		m[0][x] = ((x * 16807) + DEPTH) % 20183;
	}
	for y in 1..H {
		m[y][0] = ((y * 48271) + DEPTH) % 20183;
		for x in 1..W {
			if (x as isize, y as isize) != T {
				m[y][x] = ((m[y - 1][x] * m[y][x - 1]) + DEPTH) % 20183;
			}
		}
	}
	
	let chrs = [ '.', '=', '|' ];
	let mut r = 0;
	for y in 0..(T.1 + 1) {
		let mut line = String::new();
		for x in 0..(T.0 + 1) {
			let t = m[y as usize][x as usize] % 3;
			r += t;
			line.push(chrs[t]);
		}
		//println!("{}", line);
	}
	println!("{}", r);
	
	let mut bt = HashMap::new();
	let mut vis = HashSet::new();
	let mut q = BinaryHeap::new();
	q.push((bound((0, 0), Torch), 0, (0isize, 0isize), Torch, ((0,0), Torch)));
	while let Some((b, d, p, g, prev)) = q.pop() {
		if !vis.insert((p, g)) {
			continue;
		}
		bt.insert((p, g), prev);
		//println!("considering {:?} {:?} at distance {} bound {}", p, g, d, b);
		if p == T && g == Torch {
			println!("{}", d);
			break;
		}
		for &np in neighbors(p).iter() {
			if let Some(nt) = mtype(&m, np) {
				if compat(nt, g) {
					q.push((-bound(np, g) - d - 1, d + 1, np, g, (p, g)))
				}
			}
		}
		for &sg in [ Hands, Torch, Climb ].iter() {
			if sg != g && compat(mtype(&m, p).unwrap(), sg) {
				q.push((-bound(p, sg) - d - 7, d + 7, p, sg, (p, g)));
			}
		}
	}

   let mut c = 0;
	let mut cur = (T, Torch);
	while cur != ((0, 0), Torch) {
		//println!("{:?}", cur);
		let mut prev = bt[&cur];
		match (mtype(&m, cur.0).unwrap(), cur.1) {
			(Rocky, Hands) | (Wet, Torch) | (Narrow, Climb) => panic!("gear"),
			_ => (),
		}
		if prev.0 == cur.0 && prev.1 != cur.1 {
			c += 7;
		} else if prev.0 != cur.0 && prev.1 == cur.1 {
			let (px, py) = prev.0;
			let (cx, cy) = cur.0;
			if (px - cx).abs() + (py - cy).abs() != 1 {
				panic!("move");
			}
			c += 1;
		} else {
			panic!("discon");
		}
		cur = bt[&cur];
	}
	
	println!("done {}", c);
}
