use std::io::BufRead;
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::ops::IndexMut;
use std::rc::Rc;

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
  cnt: usize,
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
      cnt: 0,
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
      self.cnt += 1;
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

#[derive(Clone, Debug)]
struct BDT {
  kind: BDTK,
  id: u32,
}

impl BDT {
  fn dump(&self) {
    fn traverse(n: &BDT, l: &mut String) {
      match n.kind {
        BDTK::Var(i, ref neg, ref pos) => {
          let c = std::char::from_u32('a' as u32 + i).unwrap();
          for (b, c) in &[(c, neg), (c.to_ascii_uppercase(), pos)] {
            l.push(*b);
            traverse(c, l);
            l.pop();
          }
        },
        BDTK::DontCare => (),
        BDTK::False => println!("{} -> false", l),
        BDTK::True => println!("{} -> true", l),
      }
    }
    
    let mut l = String::new();
    traverse(self, &mut l);
  }
}

#[derive(Clone, Debug)]
enum BDTK {
  DontCare,
  False,
  True,
  Var(u32, Rc<BDT>, Rc<BDT>),
}

struct BDTContext {
  cache: HashMap<(u32, u32, u32), std::rc::Weak<BDT>>,
  r_dontcare: Rc<BDT>,
  r_false: Rc<BDT>,
  r_true: Rc<BDT>,
  next_id: u32,
}

impl BDTContext {
  fn new() -> BDTContext {
    BDTContext {
      cache: HashMap::new(),
      r_dontcare: Rc::new(BDT { kind: BDTK::DontCare, id: 2 }),
      r_false: Rc::new(BDT { kind: BDTK::False, id: 0 }),
      r_true: Rc::new(BDT { kind: BDTK::True, id: 1 }),
      next_id: 3,
    }
  }
  
  fn get_true(&self) -> Rc<BDT> {
    Rc::clone(&self.r_true)
  }
  
  fn get_false(&self) -> Rc<BDT> {
    Rc::clone(&self.r_false)
  }
  
  fn get_dontcare(&self) -> Rc<BDT> {
    Rc::clone(&self.r_dontcare)
  }
  
  fn get_var(&mut self, i: u32, n: Rc<BDT>, p: Rc<BDT>) -> Rc<BDT> {
    let entry = self.cache.entry((i, n.id, p.id));
    if let Entry::Occupied(ref e) = entry {
      if let Some(r) = e.get().upgrade() {
        return r;
      }
    }
 
    let r = Rc::new(BDT {
      kind: BDTK::Var(i, n, p),
      id: self.next_id,
    });
    self.next_id += 1;
    
    match entry {
      Entry::Occupied(mut e) => { e.insert(Rc::downgrade(&r)); },
      Entry::Vacant(e) => { e.insert(Rc::downgrade(&r)); },
    }
    
    r
  }
  
  fn from(&mut self, pattern: u32, len: u32, value: bool) -> Rc<BDT> {
    let seq = [ 0, 2, 3, 7, 1, 4, 5, 6, 8 ];
    let mut b = if value { self.get_true() } else { self.get_false() };
    for i in seq.iter().rev().cloned() {
      let dontcare = self.get_dontcare();
      b = if pattern & (1u32 << i) > 0 {
        self.get_var(i, b, dontcare)
      } else {
        self.get_var(i, dontcare, b)
      };
    }
    b
  }
  
  fn merge_children(&mut self, i: u32,
                    an: &Rc<BDT>, bn: &Rc<BDT>,
                    ap: &Rc<BDT>, bp: &Rc<BDT>) -> Option<Rc<BDT>> { 
    let on = self.merge(an, bn);
    let op = self.merge(ap, bp);
    if let (Some(n), Some(p)) = (on, op) {
      match (&n.kind, &p.kind) {
        (BDTK::DontCare, BDTK::DontCare) => Some(n),
        (BDTK::False, BDTK::False) => Some(n),
        (BDTK::True, BDTK::True) => Some(n),
        _ => Some(self.get_var(i, n, p)),
      }
    } else {
      None
    }
  }
  
  fn merge(&mut self, a: &Rc<BDT>, b: &Rc<BDT>) -> Option<Rc<BDT>> {
    match (&a.kind, &b.kind) {
      (BDTK::DontCare, _) => Some(Rc::clone(b)),
      (_, BDTK::DontCare) => Some(Rc::clone(a)),
      (BDTK::False, BDTK::False) => Some(self.get_false()),
      (BDTK::False, BDTK::True) => None,
      (BDTK::True, BDTK::False) => None,
      (BDTK::True, BDTK::True) => Some(self.get_true()),
      (BDTK::Var(ai, ref an, ref ap), BDTK::Var(bi, ref bn, ref bp)) => {
        if ai != bi {
          panic!("variable order");
        }
        self.merge_children(*ai, an, bn, ap, bp)
      },
      (BDTK::Var(ai, ref an, ref ap), _) =>
        self.merge_children(*ai, an, b, ap, b),
      (_, BDTK::Var(bi, ref bn, ref bp)) =>
        self.merge_children(*bi, a, bn, a, bp),
    }
  }
  
  fn simplify(&mut self, f: &Rc<BDT>) -> Rc<BDT> {
    if let BDTK::Var(i, ref n, ref p) = f.kind {
      if n.id == p.id {
        return self.simplify(n);
      }
      
      let sn = self.simplify(n);
      let sp = self.simplify(p);
      if sn.id == sp.id {
        return sn;
      }
      
      //println!("simplify {} {:?} {:?}", i, sn, sp);
      match (&sn.kind, &sp.kind) {
        (BDTK::DontCare, _) => sp,
        (_, BDTK::DontCare) => sn,
        (BDTK::False, BDTK::False) => sn,
        (BDTK::True, BDTK::True) => sn,
        _ => self.get_var(i, sn, sp),
      }
    } else {
      Rc::clone(f)
    }
  }
  
  fn split(&mut self, f: &Rc<BDT>) -> (Rc<BDT>, Rc<BDT>) {
    match f.kind {
      BDTK::Var(_, ref n, ref p) => (Rc::clone(n), Rc::clone(p)),
      _ => (Rc::clone(f), Rc::clone(f)),
    }
  }
  
  fn raise(&mut self, f: &Rc<BDT>, i: u32) -> Rc<BDT> {
    if let BDTK::Var(li, ref n, ref p) = f.kind {
      if li == i {
        return Rc::clone(f);
      }
        
      let rn = self.raise(n, i);
      let rp = self.raise(p, i);   
      let (rnn, rnp) = self.split(&rn);
      let (rpn, rpp) = self.split(&rp);
      let nn = self.get_var(li, rnn, rpn);
      let np = self.get_var(li, rnp, rpp);
      self.get_var(i, nn, np)
    } else {
      Rc::clone(f)
    }
  }
}

fn parse_pattern(text: &str) -> u32 {
  let pat = text
    .split('\n')
    .filter(|s| s.chars().next() == Some('#'))
    .next()
    .unwrap()
    .chars()
    .map(|c| (c != '#') as u32)
    .enumerate()
    .fold(0, |p, (i, c)| p | (c << i));
    
  pat >> pat.trailing_zeros()
}

fn paths(ctx: &mut BDTContext, pattern: u32) -> Vec<Rc<BDT>> {
  fn trace(ctx: &mut BDTContext, pattern: u32, f: Rc<BDT>, result: &mut Vec<Rc<BDT>>) {
    if pattern & 1 == 1 {
      return;
    }
    
    if pattern == 0 {
      result.push(f);
      return;
    }
    
    for (val, shift) in &[(true, 4), (false, 1)] {
      let step = ctx.from(pattern >> 1, 9, *val);
      //step.dump();
      if let Some(m) = ctx.merge(&f, &step) {
        trace(ctx, pattern >> shift, m, result);
      }
    }
  }
  
  let mut result = Vec::new();
  let dc = ctx.get_dontcare();
  trace(ctx, pattern << 5, dc, &mut result);
  result
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Reg {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
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
      Reg::E => 'E',
      Reg::F => 'F',
      Reg::G => 'G',
      Reg::H => 'H',
      Reg::I => 'I',
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

fn compile(ctx: &mut BDTContext, f: &Rc<BDT>) -> Vec<Op> {
  fn toreg(i: u32) -> Reg {
    match i {
      0 => Reg::A,
      1 => Reg::B,
      2 => Reg::C,
      3 => Reg::D,
      4 => Reg::E,
      5 => Reg::F,
      6 => Reg::G,
      7 => Reg::H,
      8 => Reg::I,
      _ => panic!("index {}", i),
    }
  }

  fn clause(v: &[(bool, u32)], result: &mut Vec<Op>) {
    let mut tmp = if result.len() == 0 { Reg::J } else { Reg::T };
    
    let mut inv;
    
    let itern = v.iter().filter(|(vp, _)| !*vp);
    let iterp = v.iter().filter(|(vp, _)| *vp);
    let mut iter = itern.chain(iterp);
    
    if let Some((vp, index)) = iter.next() {
      result.push(Op::Not(toreg(*index), tmp));
      inv = *vp;
    } else {
      return;
    }
    
    for (vp, index) in iter {
      if *vp == inv {
        result.push(Op::Not(tmp, tmp));
        inv = !inv;
      }
      
      if *vp {
        result.push(Op::And(toreg(*index), tmp));
      } else {
        result.push(Op::Or(toreg(*index), tmp));
      }
    }
    
    if inv {
      result.push(Op::Not(tmp, tmp));
    }
    
    if tmp != Reg::J {
      result.push(Op::Or(tmp, Reg::J));
    }
  }
  
  fn traverse(n: &Rc<BDT>,
              v: &mut Vec<(bool, u32)>,
              negc: &mut Vec<Vec<(bool, u32)>>,
              posc: &mut Vec<Vec<(bool, u32)>>) {
      match n.kind {
        BDTK::Var(i, ref neg, ref pos) => {
          if neg.id == pos.id {
            traverse(neg, v, negc, posc);
          } else {
            for (b, c) in &[(false, neg), (true, pos)] {
              v.push((*b, i));
              traverse(c, v, negc, posc);
              v.pop();
            }
          }
        },
        BDTK::DontCare => (),
        BDTK::False => negc.push(v.clone()),
        BDTK::True => posc.push(v.clone()),
      }
    }
    
  fn join(clauses: &Vec<Vec<(bool, u32)>>) -> Vec<Op> {
    /*
    for c in clauses {
      c.sort_by_key(|(_, i)| i);
    }
    
    let citer = clause.iter();
    for c1 in citer {
      for c2 in citer.clone() {
        let v1iter = c1.iter().peekable();
        let v2iter = c2.iter().peekable();
        
        let mut common = Vec::new();
        loop {
          match (v1iter.peek(), v2iter.peek()) {
            (Some((p1, i1)), Some((p2, i2))) if i1 == i2 => {
              if p1 == p2 {
                common.push((p1, v1));
              }
              v1iter.next();
              v2iter.next();
            }
            (Some((p1, i1)), Some((p2, i2))) if i1 < i2 => {
              v1iter.next();
            }
            (Some((p1, i1)), Some((p2, i2))) if i1 > i2 => {
              v2iter.next();
            }
            _ => break,
          }
        }
            (Some((p1), Some(v2)) if v1 == v2 => common.push(v1),
            (None, _) => break,
            (_, None) => break,
            
          }
        
      }
    }
    */
    
    let mut result = clauses.into_iter()
      .fold(Vec::<Op>::new(), 
            |mut p, c| { clause(c.as_slice(), &mut p); p });
            
    if result.len() == 0 {
      result.extend(&[
        Op::Not(Reg::J, Reg::T),
        Op::Or(Reg::T, Reg::J),
      ]);
    }
    
    result
  }
  
  println!("Encoding");
  f.dump();
  println!("Simplified");
  ctx.simplify(&f).dump();
  
  let mut negc = Vec::new();
  let mut posc = Vec::new();
  let mut v = Vec::new();
  traverse(&ctx.simplify(&f), &mut v, &mut negc, &mut posc);
  
  let ppos = join(&posc);
  let pneg = join(&negc.into_iter()
    .map(|v| v.into_iter()
               .map(|(p, i)| (!p, i))
               .collect::<Vec<(bool, u32)>>())
    .collect())
    .into_iter()
    .map(|o| match o {
      Op::And(a, b) => Op::Or(a, b),
      Op::Or(a, b) => Op::And(a, b),
      _ => o,
    })
    .collect::<Vec<Op>>();
    
  println!("ppos");
  for insn in &ppos {
    println!("{}", insn);
  }
  
  println!("pneg");
  for insn in &pneg {
    println!("{}", insn);
  }
  
  if ppos.len() <= pneg.len() {
    ppos
  } else {
    pneg
  }
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
      
  let prog = buf.split(',')
    .map(|s| s.trim().parse::<isize>().unwrap())
    .collect::<Vec<isize>>();
  
  /*
  for f in [91, 95, 127, 123].iter().cloned() {
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
    
    let mut vm = VM::new(p.clone());
    vm.run_far();
    vm.read();
    for insn in ff {
      vm.write(&format!("{}\n", insn));
    }
    vm.write("RUN\n");
    vm.run_far();
    
    println!("{} {}", f, vm.cnt);
    
    let m = *vm.output.back().unwrap();
    if m >= 256 {
      println!("{:b} {}", f, m);
      //break;
    }
    
    println!("{}", vm.read());
  }
  */
  
  let mut patterns = HashSet::new();
  let mut ctx = BDTContext::new();
  let mut funcs = HashMap::new();
  let dc = ctx.get_dontcare();
  funcs.insert(dc.id, dc);

  for _ in 0.. {
    
    /*
    let ff = [
      Op::Not(Reg::A, Reg::J),
      Op::Not(Reg::B, Reg::T),
      Op::Or(Reg::T, Reg::J),
      Op::Not(Reg::C, Reg::T),
      Op::Or(Reg::T, Reg::J),
      Op::And(Reg::D, Reg::J),
      //Op::Or(Reg::T, Reg::J),
      /*
      Op::Not(Reg::H, Reg::T),
      Op::Not(Reg::T, Reg::T),
      Op::And(Reg::G, Reg::T),
      Op::Not(Reg::T, Reg::T),
      Op::And(Reg::T, Reg::J),
      */
    ];
    */
 
    let (f, p) = funcs.values()
      .map(|f| (Rc::clone(f), compile(&mut ctx, f)))
      .min_by_key(|e| e.1.len())
      .unwrap();
      
    println!("f");
    f.dump();
    
    println!("f simplified");
    //let raised = ctx.raise(&f, 3);
    ctx.simplify(&f).dump();
    
    println!("script");
    for insn in &p {
      println!("{}", insn);
    }
    
    //assert!(p.len() > 0);
    assert!(p.len() <= 15, "len {}", p.len());
    
    let mut vm = VM::new(prog.clone());
    vm.run_far();
    vm.read();
    
    for insn in &p {
      vm.write(&format!("{}\n", insn));
    }
    
    /*
    for _ in 0..(15 - p.len()) {
      vm.write(&format!("{}\n", Op::Not(Reg::A, Reg::T)));
    }
    */
      
    vm.write("RUN\n");
    vm.run_far();
    
    let fo = *vm.output.back().unwrap();
    if fo > 127 {
      println!("{}", fo);
      break;
    }
    
    println!("count {}", vm.cnt);
    
    let vmo = vm.read();
    println!("{}", vmo);
    let pat = parse_pattern(&vmo);
    println!("pat {:b}", pat);
    
    assert!(!patterns.contains(&pat), "failed pattern {:b}", pat);
    patterns.insert(pat);
    
    let mut newfuncs = HashMap::new();
    let patfuncs = paths(&mut ctx, pat);
    for pf in patfuncs {
      pf.dump();
      println!("");
      
      for f in funcs.values() {
        if let Some(m) = ctx.merge(&f, &pf) {
          newfuncs.insert(m.id, m);
        }
      }
    }
    
    if newfuncs.len() == 0 {
      panic!("No solution?");
    }
    
    println!("funcs");
    for f in newfuncs.values() {
      f.dump();
      println!("");
    }
    
    funcs = newfuncs;
  }
}