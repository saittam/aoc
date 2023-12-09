//rust 1.17.0 
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::io::{self, BufRead};
use std::ops::{Add, Mul};
use std::hash::Hash;
use std::cmp;
use std::iter;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Tile {
   Wall,
	Open,
   Elf(usize),
   Goblin(usize),
}

use Tile::*;

fn neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
	[ (x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1) ]
}

fn findtarget(m: &Vec<Vec<Tile>>, x: usize, y: usize) -> (usize, usize) {
	let unit = m[y][x];
	let mut q = VecDeque::<(usize, usize)>::new();
	let mut vis = HashMap::<(usize, usize), (usize, usize)>::new();
	
	q.push_back((x, y));
	vis.insert((x, y), (x, y));
	
	while !q.is_empty() {
		let (cx, cy) = q.pop_front().unwrap();
		for &(nx, ny) in neighbors(cx, cy).iter() {
			match vis.entry((nx, ny)) {
				Entry::Occupied(_) => continue,
				Entry::Vacant(e) => { e.insert((cx, cy)); () },
			}
			match (m[ny][nx], unit) {
				(Open, _) => q.push_back((nx, ny)),
				(Elf(_), Goblin(_)) | (Goblin(_), Elf(_)) => {
					// have a target, trace back...
					//println!("target is {} {}", nx, ny);
					let mut pos = (cx, cy);
					loop {
						let prev = vis[&pos];
						if prev == (x, y) {
							return pos;
						}
						pos = prev;
					}
				},
				_ => (),
			}
		}
	}
	
	// no target - stay in current position.
	//println!("no target");
	return (x, y);
}

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut m = Vec::<Vec<Tile>>::new();
   loop {
   	let mut buf = String::new();
	   handle.read_line(&mut buf);
		if buf.is_empty() {
			break;
		}
		let l = buf.trim().chars().take_while(|x| *x != ' ').map(
			|c| match c {
				'#' => Wall,
				'.' => Open,
				'E' => Elf(200),
				'G' => Goblin(200),
				c => panic!("input '{}'", c),
			}).collect();
		m.push(l);
	}
	
	let (mut ne, mut ng) = m.iter().flat_map(|r| r.iter()).fold((0, 0),
			|(ne, ng), u| match *u {
				Elf(hp) => (ne + 1, ng),
				Goblin(hp) => (ne, ng + 1),
				_ => (ne, ng),
			});

   let mut round = 0;
'o:
	loop {
		let seq = m.iter().enumerate()
		   .flat_map(|(y, r)| iter::repeat(y).zip(r.iter().enumerate()))
		   .filter_map(|(y, (x, t))| match *t {
		   	Elf(_) | Goblin(_) => Some((x, y)),
		   	_ => None,
		   }).collect::<Vec<(usize, usize)>>();
		
		for (x, y) in seq {
			let unit = m[y][x];
			
			if ne == 0 || ng == 0 {
				break 'o;
			}
			
			//println!("move {:?} {},{}", unit, x, y);
			
			// move
			let (mx, my) = findtarget(&m, x, y);
			m[y][x] = Open;
			m[my][mx] = unit;
			
			//println!("attack {:?} {},{}", unit, mx, my);
			
			// attack
			let target = neighbors(mx, my).iter().filter_map(
				|&(nx, ny)| {
					match (unit, m[ny][nx]) {
						(Elf(_), Goblin(hp)) |
						(Goblin(_), Elf(hp)) => Some((hp, ny, nx)),
						_ => None,
					}
				}).min();
			if let Some((_, ty, tx)) = target {
				m[ty][tx] = match m[ty][tx] {
					Elf(hp) if hp > 3 => Elf(hp - 3),
					Goblin(hp) if hp > 3 => Goblin(hp - 3),
					Elf(_) => { ne -= 1; Open },
					Goblin(_) => { ng -= 1; Open },
					_ => panic!("target not unit"),
				};
			}
			
		}
		
		round += 1;
		
		println!("{}", round);
		for l in m.iter() {
			println!("{}", l.iter().map(
				|x| match *x {
					Open => '.',
					Wall => '#',
					Elf(_) => 'E',
					Goblin(_) => 'G',
				}).collect::<String>());
		}
	}
	
	let hp: usize = m.iter().flat_map(|r| r.iter()).map(
		|u| match *u {
			Elf(hp) | Goblin(hp) => hp,
			_ => 0,
		}).sum();

	println!("{}", round * hp);
}
    
    
    
    