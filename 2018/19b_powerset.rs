//rust 1.17.0 
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::io::{self, BufRead};
use std::ops::{Add, Mul};
use std::hash::Hash;
use std::cmp;

fn main() {
	let mut factors = HashSet::<usize>::new();
	factors.insert(1);
	
	for i in [2, 2, 2, 2, 2, 3, 131, 839].iter() {
		let fs = factors.iter().cloned().collect::<Vec<usize>>();
		for f in fs {
			factors.insert(f * i);
		}
	}
	
	println!("{:?}", factors);
	println!("{:?}", factors.iter().cloned().sum::<usize>());
}
    
    
    
    