//rust 1.17.0 
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::io::{self, BufRead};
use std::ops::{Add, Mul};
use std::hash::Hash;
use std::cmp;

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

struct Instruction {
	op: Opcode,
	a: usize,
	b: usize,
	c: usize,
}

type Regs = Vec<usize>;

fn exec(regs: &mut Regs, insn: &Instruction) {
	let idxa = insn.a as usize;
	let idxb = insn.b as usize;
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
      .filter(|c: char| c.is_digit(10))
		.collect::<String>()
		.parse::<usize>().unwrap();
      
	let mut prog = Vec::<Instruction>::new();
	loop {
		let mut insnline = String::new();
	   handle.read_line(&mut insnline);
	   
	   let insniter = insnline
    	  .split(|x| x.is_whitespace())
	      .filter(|c| !c.is_empty());
	      
	   let opc_name = insniter.next().unwrap();
	   let opc = optable.iter()
	      .filter(|opc| format!("{:?}", opc) == opc_name)
	      .next().unwrap();
	      
	   let args = insniter
	      .map(|x| x.parse::<usize>().unwrap())
	      .collect::<Vec<usize>>();
	      
		prog.push(Instruction{ op: opc, a: args[0], b: args[1], c: args[2]});
	}
	
	let mut state = vec![0; 6];
	loop {
		match prog.get(state[ipidx]) {
			Some(insn) => exec(&mut state, &insn);
			None => break;
		}
		state[ipidx] += 1;
	}
	
   println!("{:?}", state[0]); 
}
    
    
    
