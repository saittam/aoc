//rust 1.17.0 
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::io::{self, BufRead};
use std::ops::{Add, Mul};
use std::hash::Hash;
use std::cmp;

fn readline(handle: &mut std::io::StdinLock) -> Vec<usize> {
	let mut buf = String::new();
	handle.read_line(&mut buf);
	
	//println!("{}", buf);
   buf
		.split(|c: char| !c.is_digit(10))
		.filter(|s| !s.is_empty())
	   .map(|s| s.parse::<usize>().unwrap())
		.collect::<Vec<usize>>()
}

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

const optable: [Opcode; 16] = [
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

fn decode(enc: &Vec<usize>) -> Instruction {
	//println!("{:?}", enc);
	let op = *optable.iter().skip(enc[0]).next().unwrap();
	Instruction{ op: op, a: enc[1], b: enc[2], c: enc[3] }
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
      
   let mut seq = Vec::<(Regs, Vec<usize>, Regs)>::new();
   loop {
		let pre = readline(&mut handle);
		if pre.is_empty() {
			break;
		}
		let insn = readline(&mut handle);
		let post = readline(&mut handle);
		readline(&mut handle);
		   
		seq.push((pre, insn, post));
	}
	
	let mut amb3 = 0;
   let mut vops = HashMap::<usize, HashSet<Opcode>>::new();
   for (pre, insn, post) in seq {
   	let mut valid = HashSet::<Opcode>::new();
   	for opc in optable.iter() {
   		let mut decinsn = decode(&insn);
   		decinsn.op = *opc;
   		let mut state = pre.clone();
   		exec(&mut state, &decinsn);
   		if state == post {
   			valid.insert(decinsn.op);
   		}
   	}
      
      if valid.len() >= 3 {
      	amb3 += 1;
      }
      let entry = vops.entry(insn[0]);
      match entry {
      	Entry::Occupied(mut e) => *e.get_mut() = e.get().intersection(&valid).cloned().collect(),
      	Entry::Vacant(mut e) => { e.insert(valid); () }
      };       
   }

   println!("{:?}", amb3);
   println!("{:?}", vops);
}
    
    
    