//rust 1.17.0 
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::io::{self, BufRead};
use std::ops::{Add, Mul};
use std::hash::Hash;
use std::cmp;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Tile {
   Tree,
	Open,
   Lumber,
   Void,
}

use Tile::*;

fn neighbors(m: &Vec<Vec<Tile>>, ux: usize, uy: usize) -> [usize; 4] {
	let mut c = [0usize; 4];
	let x = ux as isize;
	let y = uy as isize;
	let nb = [
	   (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
	   (x - 1, y),                 (x + 1, y),
	   (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
	];
	let empty = Vec::<Tile>::new();
	let void = Void;
	for &(nx, ny) in nb.iter() {
		let t = m.get(ny as usize).unwrap_or(&empty).get(nx as usize).unwrap_or(&void);
		c[*t as usize] += 1;
	}
	return c;
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
		let l = buf.trim().chars().map(
			|c| match c {
				'#' => Lumber,
				'.' => Open,
				'|' => Tree,
				c => panic!("input '{}'", c),
			}).collect();
		m.push(l);
	}

	for tick in 0..10 {
		
		println!("{}", tick);
		for l in m.iter() {
			println!("{}", l.iter().map(
				|x| match *x {
					Open => '.',
					Tree => '|',
					Lumber => '#',
					Void => ' ',
				}).collect::<String>());
		}
		
		let mut mn = m.clone();
		for y in 0..m.len() {
			for x in 0..m[y].len() {
				//println!("updating {},{}", x, y);
				let mut c = neighbors(&m, x, y);
				//println!("matching");
				mn[y][x] = match m[y][x] {
					Open if c[Tree as usize] >= 3 => Tree,
					Tree if c[Lumber as usize] >= 3 => Lumber,
					Lumber if c[Lumber as usize] >= 1 && c[Tree as usize] >= 1 => Lumber,
					Lumber => Open,
					t => t 
				};
			}
		}
		
		m = mn;

		
	}
	
	let mut c = [0; 4];
	for r in m {
		for t in r {
			c[t as usize] += 1;
		}
	}
	
	println!("{}", c[Lumber as usize] * c[Tree as usize]);
}
    
    
    