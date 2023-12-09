//rust 1.17.0 
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};
use std::cmp;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
enum Orientation { Horz, Vert }

use Orientation::*;

type PList = Vec<(isize, isize)>;
type DoorMap = HashSet<(isize, isize, Orientation)>;

fn update(dir: char, cpos: &mut PList, m: &mut DoorMap) {
	let (dx, dy) = match dir {
		'N' => (0, -1),
		'E' => (1, 0),
		'W' => (-1, 0),
		'S' => (0, 1),
		_ => panic!("bad dir '{}'", dir),
	};
	for (ref mut x, ref mut y) in cpos.iter_mut() {
		m.insert((
			cmp::min(*x, *x + dx),
			cmp::min(*y, *y + dy),
			if dy == 0 { Horz } else { Vert }
		));
				
		*x += dx;
		*y += dy;
	}
}

fn push(res: &mut PList, e: Option<&(isize, isize)>) {
	//println!("{:?} {:?}", res.last(), e);
	match (res.last().map(|&p| p), e) {
		(Some(lep), Some(ep)) if lep != *ep => res.push(*ep),
		(None, Some(ep)) => res.push(*ep),
		_ => (),
	}
}

fn merge(a: &PList, b: &PList) -> PList {
	let mut ai = a.iter().peekable();
	let mut bi = b.iter().peekable();
	
	let mut res = PList::new();
	loop {
		match (ai.peek().map(|&&p| p), bi.peek().map(|&&p| p)) {
			(Some(ap), Some(bp)) if ap <= bp => push(&mut res, ai.next()),
		   (Some(_), None) => push(&mut res, ai.next()),
			(Some(ap), Some(bp)) if bp <= ap => push(&mut res, bi.next()),
			(None, Some(_)) => push(&mut res, bi.next()),
			_ => break,
		}
	}
	
	//println!("merged {:?} {:?} => {:?}", a, b, res);
	return res;
}

fn scan(s: &str, m: &mut DoorMap) {
	let mut stack = Vec::<PList>::new();
	let mut cpos: PList = vec![(0, 0)];
	let mut endpos = PList::new();
	for c in s.chars() {
		match c {
			'^'|'$' => (),
			'N'|'E'|'S'|'W' => update(c, &mut cpos, m),
			'(' => {
				stack.push(cpos.clone());
				endpos = PList::new();
			},
			'|' => {
				endpos = merge(&endpos, &cpos);
				cpos = stack.last().unwrap().clone();
			},
			')' => {
				cpos = merge(&endpos, &cpos);
				stack.pop();
			},
			_ => panic!("bad input '{}'", c),
		}
		//println!("{} {:?}", c, cpos);
	}
}

fn neighbors(x: isize, y: isize) -> [(isize, isize); 4] {
	[ (x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1) ]
}

fn bfs(m: &DoorMap, x: isize, y: isize) -> usize {
	let mut q = VecDeque::<(isize, isize, usize)>::new();
	let mut vis = HashSet::<(isize, isize)>::new();
	
	q.push_back((x, y, 0));
	vis.insert((x, y));
	
	let mut maxdist = 0;
	while !q.is_empty() {
		let (cx, cy, dist) = q.pop_front().unwrap();
		for &(nx, ny) in neighbors(cx, cy).iter() {
			let mx = cmp::min(cx, nx);
			let my = cmp::min(cy, ny);
			let o = if cy == ny { Horz } else { Vert };
			if !m.contains(&(mx, my, o)) || vis.contains(&(nx, ny)) {
				continue;
			}
			vis.insert((nx, ny));
			q.push_back((nx, ny, dist + 1));
			maxdist = cmp::max(maxdist, dist + 1);
		}
	}
	
	// no target - stay in current position.
	//println!("no target");
	return maxdist;
}

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
   
   let mut line = String::new();
   handle.read_line(&mut line);
   
   let mut m = DoorMap::new();
   scan(&line.trim(), &mut m);
   
   let (minx, miny, _) = m.iter().min().unwrap();
   let (maxx, maxy, _) = m.iter().max().unwrap();
   
   println!("{}", std::iter::repeat('#').take(3 + 2 * (maxx - minx) as usize).collect::<String>());
   for y in *miny..(*maxy + 2) {
   	let lineh = (*minx..(*maxx + 1)).into_iter()
   	   .map(|x| if m.contains(&(x, y, Horz)) { "|" } else { "#" })
   	   .map(|x| x.to_string())
   	   .collect::<Vec<String>>()
   	   .join(".");
   	println!("#.{}", lineh);
   	   	
   	let linev = (*minx..(*maxx + 1)).into_iter()
   	   .map(|x| if m.contains(&(x, y, Vert)) { "-" } else { "#" })
   	   .map(|x| x.to_string())
   	   .collect::<Vec<String>>()
   	   .join("#");
   	println!("#{}#", linev);
   }
   
   println!("{:?}", bfs(&m, 0, 0));
}


    
    
    
    
