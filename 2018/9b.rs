//rust 1.17.0 
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

fn clockwise(ring: &mut VecDeque<u32>) {
	let e = ring.pop_front().unwrap();
	ring.push_back(e);
}

fn counterclockwise(ring: &mut VecDeque<u32>) {
	let e = ring.pop_back().unwrap();
	ring.push_front(e);
}

const highest_valued_marble: u32 = 7151000;
const number_of_players: usize = 447;

fn main() {
	let mut ring: VecDeque<u32> = VecDeque::new();
	
	ring.push_front(0);

   let mut scores = [0u32; number_of_players];
   let mut player_iter = (0..scores.len()).cycle();
	
	for mut m in 1..(highest_valued_marble + 1) {
		let player = player_iter.next().unwrap();
		if m % 23 == 0 {
			for i in 0..7 { counterclockwise(&mut ring) }
			scores[player] += m + ring.pop_front().unwrap();
	   } else {
	      for i in 0..2 { clockwise(&mut ring) }
	      ring.push_front(m);
	   }
	   //println!("{:?}", ring);
	}
	
	let mut sb : Vec<(usize, u32)> = scores.into_iter().map(|x| *x).enumerate().collect();
	sb.sort_by_key(|&(_, s)| s);
	sb.reverse();
	println!("{:?}", sb);
}
    