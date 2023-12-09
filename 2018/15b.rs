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

fn render(m: &Vec<Vec<Tile>>) {
	for l in m.iter() {
		let scanline = l.iter().map(
			|x| match *x {
				Open => '.',
				Wall => '#',
				Elf(_) => 'E',
				Goblin(_) => 'G',
			}).collect::<String>();
		let hitpoints = l.iter().filter_map(
			|x| match *x {
				Elf(_) | Goblin(_) => Some(format!("{:?}", *x)),
				_ => None
			}).collect::<Vec<String>>().join(",");
		println!("{} {}", scanline, hitpoints);
	}
}

fn neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
	[ (x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1) ]
}

fn findtarget(m: &Vec<Vec<Tile>>, x: usize, y: usize) -> (usize, usize) {
	let unit = m[y][x];
	let mut q = VecDeque::<(usize, usize, usize)>::new();
	let mut vis = HashMap::<(usize, usize), (usize, usize)>::new();
	
	q.push_back((x, y, 0));
	vis.insert((x, y), (x, y));
	
	let mut target = None;
	let mut target_dist = usize::max_value();
	while let Some((cx, cy, cd)) = q.pop_front() {
		if target_dist < cd {
			break;
		}
		for &(nx, ny) in neighbors(cx, cy).iter() {
			match vis.entry((nx, ny)) {
				Entry::Occupied(_) => continue,
				Entry::Vacant(e) => { e.insert((cx, cy)); () },
			}
			match (m[ny][nx], unit) {
				(Open, _) => q.push_back((nx, ny, cd + 1)),
				(Elf(_), Goblin(_)) | (Goblin(_), Elf(_)) => {
					target_dist = cd;
					target = Some(match target {
						None => (cx, cy),
						Some((tx, ty)) if (cy, cx) < (ty, tx) => (cx, cy),
						Some(t) => t,
					});
				},
				_ => (),
			}
		}
	}
	
	match target {
		Some((tx, ty)) => {
			// have a target, trace back path...
			//println!("target is {} {}", nx, ny);
			let mut pos = (tx, ty);
			loop {
				let prev = vis[&pos];
				if prev == (x, y) {
					break;
				}
				pos = prev;
			}
			pos
		},
		// no target - stay in current position.
		None => (x, y),
	}
}

fn battle(m: &mut Vec<Vec<Tile>>, elf_attack_power: usize)
   -> (usize, usize, usize) {
	let (nes, ngs) = m.iter().flat_map(|r| r.iter()).fold((0, 0),
			|(ne, ng), u| match *u {
				Elf(hp) => (ne + 1, ng),
				Goblin(hp) => (ne, ng + 1),
				_ => (ne, ng),
			});
	let (mut ne, mut ng) = (nes, ngs);
   
   let mut round = 0;
'o:
	loop {
		let seq = m.iter().enumerate()
		   .flat_map(|(y, r)| iter::repeat(y).zip(r.iter().enumerate()))
		   .filter_map(|(y, (x, t))| match *t {
		   	Elf(_) | Goblin(_) => Some((x, y)),
		   	_ => None,
		   }).collect::<Vec<(usize, usize)>>();
		   
		let mut dead = HashSet::<(usize, usize)>::new();
		
		for (x, y) in seq {
			let unit = m[y][x];
			
			if dead.contains(&(x, y)) {
				//println!("skipping {:?}", unit);
				continue;
			}
			
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
					Goblin(hp) if hp > elf_attack_power => Goblin(hp - elf_attack_power),
					Elf(_) => { ne -= 1; dead.insert((tx, ty)); Open },
					Goblin(_) => { ng -= 1; dead.insert((tx, ty)); Open },
					_ => panic!("target not unit"),
				};
			}
			
		}
		
		round += 1;
	}
	
	let hp: usize = m.iter().flat_map(|r| r.iter()).map(
		|u| match *u {
			Elf(hp) | Goblin(hp) => hp,
			_ => 0,
		}).sum();

	return (round * hp, nes - ne, ngs - ng);
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
	
	for i in 3.. {
		let mut mb = m.clone();
   	let (v, ed, gd) = battle(&mut mb, i);
   	println!("{} {:?} {}", i, v, ed);
   	if ed == 0 {
   		render(&mb);
   		break;
   	}
   }
}
    