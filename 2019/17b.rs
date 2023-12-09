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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
  Scaffold,
  Space,
  Bot(Dir),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
  Left,
  Right,
  Straight,
}

fn main() {
  let stdin = std::io::stdin();
  let mut handle = stdin.lock();
    
  let mut buf = String::new();
  handle.read_line(&mut buf);
      
  let p = buf.split(',')
    .map(|s| s.trim().parse::<isize>().unwrap())
    .collect::<Vec<isize>>();
    
  let mut vm = VM::new(p.clone());
  while vm.run() == Status::Output {}
  
  let mut state = Vec::<Vec<Tile>>::new();
  let mut row = Vec::<Tile>::new();
  while let Some(o) = vm.output.pop_front() {
    match o {
      35 => row.push(Tile::Scaffold),
      46 => row.push(Tile::Space),
      60 => row.push(Tile::Bot(Dir::W)),
      62 => row.push(Tile::Bot(Dir::E)),
      94 => row.push(Tile::Bot(Dir::N)),
      118 => row.push(Tile::Bot(Dir::S)),
      10 => {
        state.push(row);
        row = Vec::<Tile>::new();
      }
      _ => panic!("bad {}", o),
    }
  }
  state.push(row);
  
  show(&state);
  
  let path = trace(&state);
  showpath(&path);
  
  let mut patterns = Vec::<Vec<Move>>::new();
  let mut seq = Vec::<usize>::new();
  let found = find_patterns(&path[0..], &mut patterns, &mut seq);
  
  println!("{} {:?}", found, seq);
  for p in &patterns {
    showpath(p);
  }
  
  fn list<I: IntoIterator<Item = String>>(i: I) -> Vec<isize> {
    let mut v = i.into_iter().fold(
      Vec::<isize>::new(),
      |mut v, e| {
        v.extend(e.chars().map(|c| c as isize));
        v.push(',' as isize);
        v 
      });
    *v.last_mut().unwrap() = '\n' as isize;
    println!("({}) {:?}", v.len(), v);
    v
  };
  
  vm = VM::new(p.clone());
  vm.mem[0] = 2;
  while vm.run() == Status::Output { }
  vm.output.clear();
  
  vm.input.extend(list(seq.iter().map(|m| "ABC"[*m..(m+1)].to_string())));
  
  patterns.resize_with(3, Default::default);
  for p in patterns {
    let mut enc = Vec::<String>::new();
    let mut i = p.iter();
    let mut sm = i.next();
    
'next:
    while let Some(m) = sm {
      let mc = match m {
        Move::Left => "L",
        Move::Right => "R",
        Move::Straight => {
          let mut c = 1;
          loop {
            sm = i.next();
            if sm != Some(&Move::Straight) {
              break;
            }
            c += 1;
          }
          enc.push(c.to_string());
          continue;
        }
      };
      enc.push(mc.to_string());
      sm = i.next();
    }
    vm.input.extend(list(enc.into_iter()));
  }
  
  vm.input.extend(&['n' as isize, '\n' as isize]);
  
  while vm.run() == Status::Output { }
  
  println!("{:?}", vm.run());
  println!("{}", vm.output.iter().map(|v| char::from(*v as u8)).collect::<String>());
  println!("{}", vm.output.back().unwrap());
}

fn find_patterns(path: &[Move], patterns: &mut Vec<Vec<Move>>, seq: &mut Vec<usize>) -> bool {
  if path.len() == 0 {
    return false;
  }
  
  if seq.len() >= 20 {
    return false;
  }
  
  let mut enclen = 0;
  let mut last = None;
  for l in 1..path.len() {
    let pat = &path[0..l];
    
    if last != Some(path[l - 1]) {
      enclen += 2;
    }
    last = Some(path[l - 1]);
    
    if enclen > 20 {
      return false;
    }
   
    let (pop, index) = if 
        let Some(p) = patterns.iter().position(|p| p.as_slice() == pat) {
      (false, p)
    } else {
      patterns.push(Vec::from(pat));
      (true, patterns.len() - 1)
    };
    seq.push(index);
    
    let found = if patterns.len() < 3 {
      find_patterns(&path[l..], patterns, seq)
    } else {
      complete(&path[l..], patterns, seq)
    };
    if found {
      return true;
    }
    
    seq.pop();
    if pop {
      patterns.pop();
    }
  }
      
  false
}

fn complete(path: &[Move], patterns: &Vec<Vec<Move>>, seq: &mut Vec<usize>) -> bool {
  if path.len() == 0 {
    return true;
  }
  
  if seq.len() >= 10 {
    return false;
  }
  
  for (i, pat) in patterns.iter().enumerate() {
    
    if path.len() >= pat.len() && *pat.as_slice() == path[0..pat.len()] {
      seq.push(i);
      if complete(&path[pat.len()..], patterns, seq) {
        return true;
      }
      seq.pop();
    }
  }
  
  false
}
  
fn trace(state: &Vec<Vec<Tile>>) -> Vec<Move> {
  let (mut bp, mut bd) = state.iter().enumerate().flat_map(
    |(y, r)| r.iter().enumerate().filter_map(
      move |(x, t)| match t {
        Tile::Bot(d) => Some(((x as isize, y as isize), *d)),
        _ => None,
      })
    ).next().unwrap();
    
  let get = |(x, y)|
    if x < 0 || y < 0 {
      Tile::Space
    } else {
      *state.get(y as usize)
        .and_then(|v| v.get(x as usize))
        .unwrap_or(&Tile::Space)
    };

  println!("bot @ {:?} {:?} {:?}", bp, bd, get(bp));
  
  let mut path = Vec::<Move>::new();
  
'step:
  loop {
    //println!("bot @ {:?} {:?}", bp, bd);
    
    if get(bd.step(bp)) == Tile::Scaffold {
      path.push(Move::Straight);
      bp = bd.step(bp);
      continue;
    }
    
    let turns = [
      (bd.left(), Move::Left),
      (bd.right(), Move::Right),
    ];
    for (d, m) in &turns {
      //println!("checking {:?}", d.step(bp));
      if get(d.step(bp)) == Tile::Scaffold {
        path.push(*m);
        bd = *d;
        continue 'step;
      }
    }
    
    break;
  }
  
  path
}

fn showpath(path: &Vec<Move>) {
  println!("{:?}", path.iter().map(|m| match m {
    Move::Left => 'L',
    Move::Right => 'R',
    Move::Straight => '.',
  }).collect::<String>());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
  N = 1,
  S = 2,
  W = 3,
  E = 4,
}

impl Dir {
  fn all() -> [Dir; 4] {
    [Dir::N, Dir::S, Dir::W, Dir::E]
  }
  
  fn rev(&self) -> Dir {
    match self {
      Dir::N => Dir::S,
      Dir::S => Dir::N,
      Dir::W => Dir::E,
      Dir::E => Dir::W,
    }
  }
  
  fn left(&self) -> Dir {
    match self {
      Dir::N => Dir::W,
      Dir::S => Dir::E,
      Dir::W => Dir::S,
      Dir::E => Dir::N,
    }
  }
  
  fn right(&self) -> Dir {
    match self {
      Dir::N => Dir::E,
      Dir::S => Dir::W,
      Dir::W => Dir::N,
      Dir::E => Dir::S,
    }
  }
  
  fn step(&self, (x, y): (isize, isize)) -> (isize, isize) {
    let n = 1;
    match self {
      Dir::N => (x, y - n),
      Dir::S => (x, y + n),
      Dir::W => (x - n, y),
      Dir::E => (x + n, y),
    }
  }
}
  
fn show(state: &Vec<Vec<Tile>>) {
  println!("----------");
  
  for r in state {
    println!("{}", r.iter().map(|t| match t {
      Tile::Scaffold => '#',
      Tile::Space => '.',
      Tile::Bot(_) => 'X',
    }).collect::<String>());
  }

  println!("----------");
}