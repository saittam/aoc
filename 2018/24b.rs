//rust 1.17.0 
use std::collections::{HashSet};
//use std::collections::hash_map::Entry;
use std::io::{self, BufRead};
//use std::ops::{Add, Mul};
//use std::hash::Hash;
//use std::cmp;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Side {
	ImmuneSystem,
	Infection,
}
use Side::*;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Group {
	side: Side,
	nunits: usize,
	hp: usize,
	weak: HashSet<String>,
	immune: HashSet<String>,
	damage_points: usize,
   damage_type: String,
	initiative: usize,
}

impl Group {
	fn power(&self) -> usize {
		self.damage_points * self.nunits
	}
	
	fn alive(&self) -> bool {
		self.nunits > 0
	}
	
	fn attack_damage(&self, target: &Group) -> usize {
		self.power() * 
		if target.immune.contains(&self.damage_type) {
			0
		} else if target.weak.contains(&self.damage_type) {
			2
		} else {
			1
		}
	}
}

fn battle(groups: &mut Vec<Group>) -> bool {
	loop {
		//println!("{}", groups.iter().map(|g| format!("{:?}", g)).collect::<Vec<String>>().join("\n"));
		//println!("\n");
		
		let mut ag = (0..groups.len()).filter(|i| groups[*i].alive()).collect::<Vec<usize>>();
		ag.sort_by_key(|i| (groups[*i].power(), groups[*i].initiative));
		let mut targets = Vec::<(usize, usize)>::new();
		for ai in ag.iter().rev() {
			if let Some(ti) = (0..groups.len())
			   .filter(|ti| groups[*ti].alive())
			   .filter(|ti| groups[*ti].side != groups[*ai].side)
			   .filter(|ti| targets.iter().all(|(_, i)| *ti != *i))
			   .max_by_key(|ti| (
			   	groups[*ai].attack_damage(&groups[*ti]),
			   	groups[*ti].power(),
			   	groups[*ti].initiative)) {
			   if groups[*ai].attack_damage(&groups[ti]) > 0 {
			   	targets.push((*ai, ti));
			   }
			}
		}
		
		if targets.is_empty() {
			break;
		}
		//println!("{:?}", targets);
		
		
		let mut total_units_lost = 0;
		
		targets.sort_by_key(|(ai, _)| groups[*ai].initiative);
		for (ai, ti) in targets.iter().rev() {
			if !groups[*ai].alive() {
				continue;
			}
			let ul = groups[*ai].attack_damage(&groups[*ti]) / groups[*ti].hp;
			total_units_lost += ul;
			groups[*ti].nunits = groups[*ti].nunits.checked_sub(ul).unwrap_or(0);
		}
		
		if total_units_lost == 0 {
			return false;
			//println!("no units lost");
		   //println!("{}", groups.iter().map(|g| format!("{:?}", g)).collect::<Vec<String>>().join("\n"));
		}
	}
	
	return true;
}

fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
   
   let mut groups = Vec::<Group>::new();
   for side in [ImmuneSystem, Infection].iter() {
   	let mut buf = String::new();
	   handle.read_line(&mut buf);
	   
	   loop {
	   	let mut buf = String::new();
	   	handle.read_line(&mut buf);
	   	if buf.trim().is_empty() {
	   		break;
	   	}

	   	let mut parts = buf.trim()
	   	   .split(|c| c == '(' || c == ';' || c == ')')
	   	   .map(|s| s.to_string())
	   	   .collect::<Vec<String>>();
	   	if parts.len() == 1 {
	   		let (p, _) = buf.trim().match_indices("with an attack").next().unwrap();
	   	   let (l, t) = buf.trim().split_at(p);
	   	   parts = vec![l.to_string(), t.to_string()];
	   	}
	   	//println!("{:?}", parts);

	   	let ml = parts.first().unwrap()
	   	   .split(|c: char| !c.is_digit(10))
	   	   .filter(|s| !s.is_empty())
  		    .map(|s| s.parse::<usize>().unwrap())
  		    .collect::<Vec<usize>>();
  		 let mut weak = HashSet::<String>::new();
  		 let mut immune = HashSet::<String>::new();
  		 for prop in parts[1..(parts.len() - 1)].iter() {
  		 	let mut pparts = prop.trim()
  		 	   .split(|c: char| !c.is_alphabetic())
  		 	   .filter(|s| !s.is_empty());
  		 	let pt = pparts.next().unwrap();
  		 	let mut ps = match pt {
  		 		"weak" => &mut weak,
  		 		"immune" => &mut immune,
  		 		_ => panic!("prop type '{}'", pt),
  		 	};
  		 	pparts.next();
  		 	*ps = pparts.map(|s| s.trim().to_string()).collect();
  		 }
  		 let mt = parts.last().unwrap()
	   	   .split(' ')
	   	   .filter(|s| !s.is_empty())
  		    .map(|s| s.to_string())
  		    .collect::<Vec<String>>();
  		 //println!("{:?}", mt);
  		    
  		 groups.push(
  		 	Group{
  		 		side: *side,
  		 		nunits: ml[0],
  		 		hp: ml[1],
  		 		weak: weak,
  		 		immune: immune,
  		 		damage_points: mt[5].parse::<usize>().unwrap(),
  		 		damage_type: mt[6].clone(),
  		 		initiative: mt[10].parse::<usize>().unwrap(),
  		 	});
  	 }
	}
	
	let mut lb = 1;
	let mut ub = None;
   let mut boost = 0;
	loop {
		boost = match ub {
			None => lb * 2,
			Some(b) if b - lb > 1 => lb + (b - lb) / 2,
			_ if boost > 0 => boost - 1,
			_ => break,
		};

		let mut gb = groups.clone();
		for g in gb.iter_mut() {
			if g.side == ImmuneSystem {
				g.damage_points += boost;
			}
		}
		
		//println!("boost {}", boost);
		let term = battle(&mut gb);

		let s: usize = gb.iter().filter(|g| g.side == ImmuneSystem).map(|g| g.nunits).sum();
		println!("{} {} {}", boost, s, term);
		
		if ub.unwrap_or(lb + 2) - lb <= 1 {
			continue;
		}
		
		if s > 0 && term {
			ub = Some(boost);
		} else {
			lb = boost;
		}
	}
}
    
    
    
