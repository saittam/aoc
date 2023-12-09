//rust 1.17.0 
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::io::{self, BufRead};
use std::ops::{Add, Mul};
use std::hash::Hash;
use std::cmp;

#[derive(PartialEq, Eq, Debug)]
enum Tile {
	Sand,
	Clay,
	Rest,
	Flow,
}

use Tile::*;

fn horiz(m: &mut Vec<Vec<Tile>>,
	      mut x: usize,
	      y: usize,
	      ymax: usize,
	      inc: isize) -> (usize, bool) {
	let mut flow = false;

   // find the extent of the stretch.
	loop {
		let xn = (x as isize + inc) as usize;
		
		fill(m, x, y + 1, ymax);
		match m[x][y + 1] {
			Flow => { flow = true; break; },
			Clay | Rest => (),
			Sand => panic!("sand drop"),
		}
			
		match m[xn][y] {
			Sand => (),
			Clay => break,
			Flow => {
				flow = true;
				break;
			},
			Rest => panic!("open rest"),
		}
			
		x = xn;
	}
	
	return (x, flow);
}

fn fill(m: &mut Vec<Vec<Tile>>, xs: usize, ys: usize, ymax: usize) {
	let mut y = ys;
	while y <= ymax && m[xs][y] == Sand {
	   m[xs][y] = Flow;
		y += 1;
	}
	
	if y > ymax || m[xs][y] == Flow {
		return;
	}
	
   y -= 1;
   let mut flow = false;
	while y >= ys {
		let (xl, flowl) = horiz(m, xs, y, ymax, -1);
		let (xh, flowh) = horiz(m, xs, y, ymax,  1);
		flow = flowl || flowh;
		
		// update the level
		for x in xl..(xh + 1) {
			m[x][y] = if flow { Flow } else { Rest };
		}

		y -= 1;
   }
}
	
fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
      
   let mut yl = Vec::<(usize, usize, usize)>::new();
   let mut xl = Vec::<(usize, usize, usize)>::new();
   loop {
   	let mut buf = String::new();
	   handle.read_line(&mut buf);
		if buf.is_empty() {
			break;
		}
		let dest = if buf.chars().next().unwrap() == 'x' { &mut xl } else { &mut yl };
		let c = buf
	   	.split(|c: char| !c.is_digit(10))
	   	.filter(|s| !s.is_empty())
	      .map(|s| s.parse::<usize>().unwrap())
   		.collect::<Vec<usize>>();
		dest.push((c[0], c[1], c[2]));
	}
	
	let xmax = 
		xl.iter().map(|&(x, _, _)| x)
		.chain(yl.iter().map(|&(_, _, x2)| x2)).max().unwrap();
	let ymax = 
		xl.iter().map(|&(_, _, y2)| y2)
		.chain(yl.iter().map(|&(y, _, _)| y)).max().unwrap();
	let ymin = 
		xl.iter().map(|&(_, y1, _)| y1)
		.chain(yl.iter().map(|&(y, _, _)| y)).min().unwrap();
		
	let mut m: Vec<Vec<Tile>> = (0..(xmax + 1)).map(|_| (0..(ymax + 1)).map(|_| Sand).collect()).collect();
	for (x, y1, y2) in xl {
		for y in y1..(y2 + 1) {
			m[x][y] = Clay;
		}
	}
	for (y, x1, x2) in yl {
		for x in x1..(x2 + 1) {
			m[x][y] = Clay;
		}
	}
	
	fill(&mut m, 500, ymin, ymax);
	
	let mut nrest = 0;
	let mut nflow = 0;
	for y in 0..(ymax + 1) {
		for x in 0..(xmax + 1) {
			match m[x][y] {
				Flow => nflow += 1,
				Rest => nrest += 1,
				_ => (),
		   }
		}
	}
	
	println!("{} {}", nflow + nrest, nrest);
}
    
    
    