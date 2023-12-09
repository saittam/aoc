//rust 1.17.0 
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn countprefix(s: &str) -> usize {
	s.chars().take_while(|c| *c == '.').count()
}

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
   
   let mut state_line = String::new();
   handle.read_line(&mut state_line);
   let mut state = state_line.trim().chars().skip(15).collect::<String>();
   handle.read_line(&mut String::new());
      
   let mut rules = HashMap::<String, char>::new();
   loop {
		let mut buf = String::new();
		handle.read_line(&mut buf);
		if buf.len() == 0 {
			break
		}
		let c: Vec<char> = buf.chars().collect();
		rules.insert(c[0..5].iter().collect::<String>(), c[9]);
   }
   
   //println!("{:?}", rules);
   //println!("0 {}", state);
   
   let mut start: i32 = 0;
   let mut step: usize = 0;
   let mut shift: usize = 0;
   loop {
   	step += 1;
   	let mut new_state = String::new();
   	let mut slice = ".....".to_string();
   	for c in state.chars().chain("....".chars()) {
   		//println!("{}/{}/", slice, c);
   		slice.push(c);
   		//println!("{}", slice);
   		slice = slice.chars().skip(1).collect::<String>();
   		//println!("{}", slice);
   		new_state.push(*rules.get(&slice).unwrap_or(&'.'));
   	}
   	
   	
   	if state.trim_matches('.') == new_state.trim_matches('.') {
   		shift = countprefix(&new_state) - countprefix(&state) - 2;
   		break;
   	}
   	
   	
   	state = new_state.to_string();
   	//println!("{} {}", step, state);
   }
	
   let s: i32 = state.chars().enumerate()
      .filter(|&(_, c)| c == '#')
      .map(|(i, _)| (step as i32 * -2) + i as i32)
      .sum();
   let nh = state.chars().filter(|c| *c == '#').count();
   
   println!("{}", s as usize + (50000000000 + 3 - step) * shift * nh);
}


    
    
    
