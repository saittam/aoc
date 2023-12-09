use std::io::BufRead;
use std::collections::{HashMap, VecDeque};
use std::ops::IndexMut;

struct Mem {
  m: Vec<isize>,
  h: HashMap<isize, isize>,
}

impl Mem {
  fn new(m: Vec<isize>) -> Mem {
    Mem {
      m: m,
      h: HashMap::new(),
    }
  }
}

impl std::ops::Index<isize> for Mem {
  type Output = isize;
  
  fn index(&self, index: isize) -> &isize {
    if let Some(v) = self.m.get(index as usize) {
      return v;
    }
    
    if let Some(v) = self.h.get(&index) {
      return v;
    }
    
    &0
  }
}

impl std::ops::IndexMut<isize> for Mem {
  fn index_mut(&mut self, index: isize) -> &mut isize {
    if let Some(v) = self.m.get_mut(index as usize) {
      return v;
    }
    
    self.h.entry(index).or_insert(0)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
  NeedInput,
  Output,
  Halt,
}

struct VM {
  mem: Mem,
  ip: isize,
  base: isize,
  input: VecDeque<isize>,
  output: VecDeque<isize>,
}

#[derive(Clone, Copy)]
enum Mode {
  Positional = 0,
  Immediate = 1,
  Relative = 2,
}

impl VM {
  
  fn new(prog: Vec<isize>) -> VM {
    VM {
      mem: Mem::new(prog),
      ip: 0,
      base: 0,
      input: VecDeque::new(),
      output: VecDeque::new(),
    }
  }
  
  fn decode(insn: isize) -> (isize, Mode, Mode, Mode) {
    let mut v = insn;
    
    let m3 = match (v >> 13) & 0b11 {
      0b00 => Mode::Positional,
      0b01 => Mode::Immediate,
      0b10 => Mode::Relative,
      _ => Mode::Positional,
    };
    v -= 10000 * m3 as isize;
    
    let m2 = match (v >> 9) & 0b111 {
      0b000 => Mode::Positional,
      0b001 | 0b010 => Mode::Immediate,
      0b011 | 0b100 => Mode::Relative,
      _ => Mode::Positional,
    };
    v -= 1000 * m2 as isize;
    
    let m1 = match (v >> 6) & 0b11 {
      0b00 => Mode::Positional,
      0b01 => Mode::Immediate,
      0b11 => Mode::Relative,
      _ => Mode::Positional,
    };
    v -= 100 * m1 as isize;
      
    assert_eq!(insn, v + m1 as isize * 100 + m2 as isize * 1000 + m3 as isize * 10000);
    
    (v, m1, m2, m3)    
  }
  
  fn rd(&self, (addr, m): (isize, Mode)) -> isize {
    let v = self.mem[addr];
    match m {
      Mode::Positional => self.mem[v],
      Mode::Immediate => v,
      Mode::Relative => self.mem[v + self.base],
    }
  }
  
  fn wr(&mut self, (addr, m): (isize, Mode)) -> &mut isize {
    let v = self.mem[addr];
    match m {
      Mode::Positional => self.mem.index_mut(v),
      Mode::Immediate => panic!("immediate mode write"),
      Mode::Relative => self.mem.index_mut(v + self.base),
    }
  }

  fn run(&mut self) -> Status {
    loop {
      let ip = self.ip;
      let insn = self.mem[ip];
    
      let (op, ma, mb, mc) = VM::decode(insn);
      let a = (ip + 1, ma);
      let b = (ip + 2, mb);
      let c = (ip + 3, mc);
      
      //println!("{}@{} {}:{} {}:{} {}"
      
      self.ip += match op {
        1 => { *self.wr(c) = self.rd(a) + self.rd(b); 4 }
        2 => { *self.wr(c) = self.rd(a) * self.rd(b); 4 }
        3 => {
          *self.wr(a) = match self.input.pop_front() {
            None => return Status::NeedInput,
            Some(i) => i,
          };
          2
        }
        4 => {
          let v = self.rd(a);
          self.output.push_back(v); 
          self.ip += 2;
          return Status::Output;
        }
        5 => { if self.rd(a) != 0 { self.rd(b) - ip } else { 3 } }
        6 => { if self.rd(a) == 0 { self.rd(b) - ip } else { 3 } }
        7 => { *self.wr(c) = (self.rd(a) < self.rd(b)) as isize; 4 }
        8 => { *self.wr(c) = (self.rd(a) == self.rd(b)) as isize; 4 }
        9 => { self.base += self.rd(a); 2 }
        -1 => return Status::Halt,
        _ => panic!("invalid opcode {}@{} {} {:b} ", op, ip, insn, insn),
      }
    }
  }
  
  fn run_far(&mut self) -> Status {
    loop {
      let s = self.run();
      if s != Status::Output {
        return s;
      }
    }
  }
  
  fn get_output<T: AsMut<[isize]> + Default>(&mut self) -> Option<T> {
    let mut obj: T = Default::default();
    {
      let result: &mut [isize] = obj.as_mut();
      let mut i = 0;
      while i < result.len() {
        if let Some(o) = self.output.pop_front() {
          result[i] = o;
          i += 1;
          continue;
        }
    
        match self.run() {
          Status::Halt => return None,
          Status::NeedInput => panic!("need input"),
          Status::Output => (),
        }
      }
    }
    
    Some(obj)
  }
  
  fn write(&mut self, s: &str) {
    self.input.extend(s.chars().map(|c| c as isize));
  }
  
  fn read(&mut self) -> String {
    self.output.drain(..).map(|i| std::char::from_u32(i as u32).unwrap_or('?')).collect::<String>()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Term {
  Neg(usize),
  Pos(usize),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Reg {
  A,
  B,
  C,
  D,
  T,
  J,
}

impl std::fmt::Display for Reg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Reg::A => 'A',
      Reg::B => 'B',
      Reg::C => 'C',
      Reg::D => 'D',
      Reg::T => 'T',
      Reg::J => 'J',
    })
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
  And(Reg, Reg),
  Or(Reg, Reg),
  Not(Reg, Reg),
}

impl std::fmt::Display for Op {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Op::And(a, b) => write!(f, "AND {} {}", a, b),
      Op::Or(a, b) => write!(f, "OR {} {}", a, b),
      Op::Not(a, b) => write!(f, "NOT {} {}", a, b),
    }
  }
}

fn idx(val: usize) -> Vec<Vec<Term>> {
  let mut result = Vec::new();
  for pat in 0..8 {
    if (1 << pat) & val > 0 {
      let mut c = Vec::new();
      for i in 0..3 {
        c.push(if (1 << i) & pat > 0 {
          Term::Pos(i)
        } else {
          Term::Neg(i)
        });
      }
      result.push(c);
    }
  }
  
  let mut comm = None;
'outer:
  for c1 in &result {
    for c2 in &result {
      let c = c1.iter().zip(c2.iter()).filter_map(
          |(t1, t2)| if *t1 == *t2 { Some(*t1) } else { None })
        .collect::<Vec<Term>>();
      if c.len() == 2 {
        comm = Some(c);
        break 'outer;
      }
    }
  }
 
  if let Some(co) = comm {
    let mut simplified = result.into_iter().filter(
        |c| !co.iter().all(|cc| c.contains(cc)))
      .collect::<Vec<Vec<Term>>>();
    simplified.push(co);
    simplified
  } else {
    result
  }
}

fn encode(f: &Vec<Vec<Term>>) -> Vec<Op> {
  fn r(i: usize) -> Reg {
    match i {
      0 => Reg::A,
      1 => Reg::B,
      2 => Reg::C,
      _ => panic!("index {}", i),
    }
  }
  
  use Term::{Neg, Pos};

  let mut res = Vec::new();
  
  if f.len() == 0 {
    res.push(Op::Not(Reg::A, Reg::J));
    res.push(Op::And(Reg::A, Reg::J));
    return res;
  }

  let mut tmp = Reg::J;
  for c in f {
    let mut cs = c.clone();
    cs.sort();
    match *cs.as_slice() {
      [Neg(x), Neg(y), Neg(z)] => {
        assert!(tmp == Reg::J);
        res.push(Op::Not(r(x), Reg::J));
        res.push(Op::Not(r(y), Reg::T));
        res.push(Op::And(Reg::T, Reg::J));
        res.push(Op::Not(r(z), Reg::T));
        res.push(Op::And(Reg::T, Reg::J));
      }
      [Neg(x), Neg(y), Pos(z)] => {
        res.push(Op::Not(r(z), tmp));
        res.push(Op::Or(r(y), tmp));
        res.push(Op::Or(r(x), tmp));
        res.push(Op::Not(tmp, tmp));
      }
      [Neg(x), Pos(y), Pos(z)] => {
        res.push(Op::Not(r(x), tmp));
        res.push(Op::And(r(y), tmp));
        res.push(Op::And(r(z), tmp));
      }
      [Pos(x), Pos(y), Pos(z)] => {
        res.push(Op::Not(r(x), tmp));
        res.push(Op::Not(tmp, tmp));
        res.push(Op::And(r(y), tmp));
        res.push(Op::And(r(z), tmp));
      }
      [Neg(x), Neg(y)] => {
        res.push(Op::Not(r(x), tmp));
        res.push(Op::Not(tmp, tmp));
        res.push(Op::Or(r(y), tmp));
        res.push(Op::Not(tmp, tmp));
      }
      [Neg(x), Pos(y)] => {
        res.push(Op::Not(r(x), tmp));
        res.push(Op::And(r(y), tmp));
      }
      [Pos(x), Pos(y)] => {
        res.push(Op::Not(r(x), tmp));
        res.push(Op::Not(tmp, tmp));
        res.push(Op::And(r(y), tmp));
      }
      _ => panic!("clause {:?}", cs)
    }
    
    if tmp == Reg::J {
      tmp = Reg::T;
    } else {
      res.push(Op::Or(Reg::T, Reg::J));
    }
  }
  
  res
}

fn eval(f: &Vec<Op>, s: &mut HashMap<Reg, bool>) {
  for insn in f {
    let (r, v) = match insn {
      Op::Not(a, b) => (*b, !s[a]),
      Op::And(a, b) => (*b, s[a] && s[b]),
      Op::Or(a, b) => (*b, s[a] || s[b]),
    };
    s.insert(r, v);
    //println!("{:?}", insn);
    //println!("{:?}", s);
  } 
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new();
  handle.read_line(&mut buf);
      
  let p = buf.split(',')
    .map(|s| s.trim().parse::<isize>().unwrap())
    .collect::<Vec<isize>>();
  
  let mut get = |(x,y)| {
    let mut vm = VM::new(p.clone());
    vm.input.push_back(x);
    vm.input.push_back(y);
    vm.get_output::<[isize; 1]>().unwrap()[0]
  };
  
  for f in 0..256 {
    let dnf = encode(&idx(f));
    let mut cnfidx = idx(0xff ^ f).into_iter().map(
      |v| v.into_iter().map(|t| match t {
        Term::Pos(i) => Term::Neg(i),
        Term::Neg(i) => Term::Pos(i),
      }).collect::<Vec<Term>>()).collect::<Vec<Vec<Term>>>();
    cnfidx.sort();
    let cnf = encode(&cnfidx).into_iter().map(
      |insn| match insn {
        Op::Or(x, y) => Op::And(x, y),
        Op::And(x, y) => Op::Or(x, y),
        op => op,
      }).collect::<Vec<Op>>();
    
    let mut ff = if dnf.len() <= cnf.len() {
      dnf
    } else {
      cnf
    };
    
    for n in 0..8 {
      let mut s = HashMap::new();
      s.insert(Reg::A, (n & 0b001) > 0);
      s.insert(Reg::B, (n & 0b010) > 0);
      s.insert(Reg::C, (n & 0b100) > 0);
      s.insert(Reg::J, false);
      s.insert(Reg::T, false);
      eval(&ff, &mut s);
      assert_eq!(((f >> n) & 1) > 0, s[&Reg::J],
        "0x{:b} 0x{:b} {}\n{:?}",
        f, n, s[&Reg::J], ff);
    }
    
    ff.push(Op::And(Reg::D, Reg::J));
    
    if ff.len() > 15 {
      println!("0x{:b} {} {:?} {:?}", f, ff.len(), idx(f), ff);
      continue;
    }
    
    //println!("{}...", f);
    
    let mut vm = VM::new(p.clone());
    vm.run_far();
    vm.read();
    for insn in ff {
      vm.write(&format!("{}\n", insn));
    }
    vm.write("WALK\n");
    vm.run_far();
    
    let m = *vm.output.back().unwrap();
    if m >= 256 {
      println!("{}", m);
      break;
    }
  }
}