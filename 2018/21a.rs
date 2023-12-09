//rust 1.17.0 
use std::collections::{HashSet};
//use std::collections::hash_map::Entry;
use std::io::{self, BufRead};
//use std::ops::{Add, Mul};
//use std::hash::Hash;
//use std::cmp;
use std::time::{SystemTime};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Opcode {
	addr,
	addi,
	mulr,
	muli,
	banr,
	bani,
	borr,
	bori,
	setr,
	seti,
	gtir,
	gtri,
	gtrr,
	eqir,
	eqri,
	eqrr,
}

use Opcode::*;

static optable: [Opcode; 16] = [
	addr,
	addi,
	mulr,
	muli,
	banr,
	bani,
	borr,
	bori,
	setr,
	seti,
	gtir,
	gtri,
	gtrr,
	eqir,
	eqri,
	eqrr,
];

#[derive(Debug)]
struct Instruction {
	op: Opcode,
	a: usize,
	b: usize,
	c: usize,
}

type Regs = Vec<usize>;

fn exec(regs: &mut Regs, insn: &Instruction) {
	//println!("{:?} {:?}", regs, insn);
	//let idxa = insn.a as usize;
	//let idxb = insn.b as usize;
	regs[insn.c as usize] = match insn.op {
		addr => regs[insn.a] + regs[insn.b],
		addi => regs[insn.a] + insn.b,
		mulr => regs[insn.a] * regs[insn.b],
		muli => regs[insn.a] * insn.b,
		banr => regs[insn.a] & regs[insn.b],
		bani => regs[insn.a] & insn.b,
		borr => regs[insn.a] | regs[insn.b],
		bori => regs[insn.a] | insn.b,
		setr => regs[insn.a],
		seti => insn.a,
		gtir => if insn.a > regs[insn.b] { 1 } else { 0 },
		gtri => if regs[insn.a] > insn.b { 1 } else { 0 },
		gtrr => if regs[insn.a] > regs[insn.b] { 1 } else { 0 },
		eqir => if insn.a == regs[insn.b] { 1 } else { 0 },
		eqri => if regs[insn.a] == insn.b { 1 } else { 0 },
		eqrr => if regs[insn.a] == regs[insn.b] { 1 } else { 0 },
	}
}
	
fn main() {
	let stdin = io::stdin();
   let mut handle = stdin.lock();
   
   let mut ipline = String::new();
	handle.read_line(&mut ipline);
	
   let ipidx = ipline.chars()
      .filter(|c| c.is_digit(10))
		.collect::<String>()
		.parse::<usize>().unwrap();
		
	let mut prog = Vec::<Instruction>::new();
	loop {
		let mut insnline = String::new();
	   handle.read_line(&mut insnline);
	   if insnline.is_empty() {
	   	break;
	   }
	   
	   let mut insniter = insnline
    	  .split(|x: char| x.is_whitespace())
	      .filter(|c| !c.is_empty());
	      
	   let opc_name = insniter.next().unwrap();
	   let opc = optable.iter()
	      .filter(|opc| format!("{:?}", opc) == opc_name)
	      .next().unwrap();
	      
	   let args = insniter
	      .map(|x| x.parse::<usize>().unwrap())
	      .collect::<Vec<usize>>();
	      
		prog.push(Instruction{ op: *opc, a: args[0], b: args[1], c: args[2]});
	}
	
	/*
	for (i, insn) in prog.iter().enumerate() {
		println!("{}: {:?}", i, insn);
	}
	*/
	
	let mut state = vec![0; 6];
	//state = vec![0, 18, 0, 0, 9138982, 65536];
	//state = vec![0, 28, 1, 1, 11696426, 77];
	//state[0] = 12420065;
	//state[2] = 65530;
	//let mut st = SystemTime::now();
	//let mut ns = 10000u64;
	let mut seen = HashSet::<usize>::new();
	let mut ps = state.clone();
	for n in 0.. {
		match prog.get(state[ipidx]) {
			Some(insn) => exec(&mut state, &insn),
			None => {
				println!("halt after {} instructions", n);
				break;
			}
		}
		state[ipidx] += 1;
		
		let ip = state[ipidx];
		
		/*
		if ip == 17 {
			let r3 = state[5] / 256;
			ps = state.clone();
			ps[5] = r3;
			ps[2] = r3;
			ps[1] = 27;
			state = ps;
			//println!("{}", r3);
		}
		*/
		
		/*
		if ip == 27 {
			println!("{} {:?} {:?}", ps == state, ps, state);
		}
		*/
		
		if ip == 28 {
			println!("{} {:?}", state[ipidx], state);
			if seen.contains(&state[4]) {
				println!("cycle {:?}", state);
				break;
			}
			seen.insert(state[4]);
			//break;
		}
		/*
		if n == ns { 
			let el = st.elapsed().unwrap();
			println!("{:?} {} instructions in {:?}", state, n, el);
			ns = n + (100 * n) / (1000 * el.as_secs() + el.subsec_millis() as u64);
			//println!("{}", ns);
		}
		*/
	}
	
   println!("at termination {:?}", state[0]); 
}
    
    
    
    
    