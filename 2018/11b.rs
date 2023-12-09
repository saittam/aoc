//rust 1.17.0 
use std::collections::{HashSet};
use std::io::{self, BufRead};

const serial_number: i32 = 6548;

fn powerlevel(x: i32, y: i32) -> i16 {
	let rack_id = x + 10;
	return ((((((rack_id * y) + serial_number) * rack_id) / 100) % 10) - 5) as i16;
}

fn main() {
	let mut mpower = i32::min_value();
	let mut bx: usize = 0;
	let mut by: usize = 0;
	let mut bsz: usize = 0;
	
	let mut levels = [[[0 as i16; 301]; 301]; 301];
	for x in 1..301 {
		for y in 1..301 {
			levels[1][x][y] = powerlevel(x as i32, y as i32);
		}
	}

	for sz in 2..301 {
		let hs = sz / 2;
		let hl = hs + (sz % 2);
		
	   for px in 1..(302 - sz) {
	   	for py in 1..(302 - sz) {
	   		let mut power: i32 = 0;
			
	   		power += levels[hl][px][py] as i32;
	   		power += levels[hl][px + hs][py + hs] as i32;
	   		power += levels[hs][px + hl][py] as i32;
		   	power += levels[hs][px][py + hl] as i32;
			
		   	if hs < hl {
	   			power -= levels[1][px + hs][py + hs] as i32;
	   		}
	   		
	   		levels[sz][px][py] = power as i16;
			
			   if power > mpower {
			   	mpower = power;
		   		bx = px;
		   		by = py;
			   	bsz = sz;
			   	println!("Better: {} {},{},{}", mpower, bx, by, bsz);
		   	}
	   	}			
	   }
	}
	
	println!("Best: {} {},{},{}", mpower, bx, by, bsz);
}
    
