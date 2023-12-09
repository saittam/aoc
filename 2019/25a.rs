use std::io::BufRead;
use std::collections::{HashMap, VecDeque};
use std::ops::IndexMut;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
enum Dir {
  N,
  S,
  W,
  E,
}

impl Dir {
  fn inv(&self) -> Dir {
    match self {
      Dir::N => Dir::S,
      Dir::S => Dir::N,
      Dir::W => Dir::E,
      Dir::E => Dir::W,
    }
  }
}

impl std::fmt::Display for Dir {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Dir::N => "north",
      Dir::S => "south",
      Dir::W => "west",
      Dir::E => "east",
    })
  }
}

impl TryFrom<&str> for Dir {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "north" => Ok(Dir::N),
      "south" => Ok(Dir::S),
      "west" => Ok(Dir::W),
      "east" => Ok(Dir::E),
      _ => panic!("dir {}", value),
    }
  }
}


fn explore(vm: &mut VM, rooms: &mut HashMap::<String, Vec<Dir>>, p: &mut Vec<Dir>, items: &mut Vec<String>) {
  vm.run_far();
  let blurb = vm.read();
  let mut li = blurb.split('\n').skip_while(|s| s.trim().len() == 0);
  
  let name = li.next().unwrap().split('=').nth(2).unwrap().trim().to_string();
  //let desc = li.next().unwrap();
  
  //println!("n {}", name);
  if rooms.contains_key(&name) {
    return;
  }
  
  let checkpoint = name == "Security Checkpoint";
  rooms.insert(name, p.clone());
  
  println!("{}", blurb);
  
  if checkpoint {
    return;
  }
  
  let mut di = li.skip_while(|s| *s != "Doors here lead:");
  di.next();
  while let Some(s) = di.next() {
    if s.len() < 2 || s[0..2] != *"- " {
      break;
    }
    
    let d = Dir::try_from(&s[2..]).unwrap();
    vm.write(&format!("{}\n", d));
    p.push(d);
    explore(vm, rooms, p, items);
    p.pop();
    vm.write(&format!("{}\n", d.inv()));
    vm.run_far();
    vm.read();
  }
  
  let mut ii = di.skip_while(|s| *s != "Items here:");
  ii.next();
  while let Some(s) = ii.next() {
    if s.len() < 2 || s[0..2] != *"- " {
      break;
    }
    
    let item = &s[2..];
    match item {
      "escape pod" => continue,
      "giant electromagnet" => continue,
      "infinite loop" => continue,
      "photons" => continue,
      "molten lava" => continue,
      _ => (),
    }
    
    vm.write(&format!("take {}\n", item));
    vm.run_far();
    println!("{}", vm.read());
    items.push(item.to_string());
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
  
  let mut vm = VM::new(p.clone());
  let mut rooms = HashMap::<String, Vec<Dir>>::new();
  //let mut pos = (0, 0);
  let mut items = Vec::new();

  explore(&mut vm, &mut rooms, &mut Vec::new(), &mut items);
  
  vm.write(&"inv\n");
  vm.run_far();
  println!("{}", vm.read());
  
  let path = &rooms["Security Checkpoint"];
  for d in path {
    vm.write(&format!("{}\n", d));
  }
  vm.run_far();
  vm.read();
  
  for p in 0..2usize.pow(items.len() as u32) {
    let mut vmt = vm.clone();
    
    println!("{:?}", p);
    
    for i in 0..items.len() {
      if (1 << i) & p > 0 {
        vmt.write(&format!("drop {}\n", items[i]));
        vmt.run_far();
        //println!("{}", vmt.read());
      }
    }
    
    vmt.write(&format!("{}\n", path.last().unwrap()));
    vmt.run_far();
    let reply = vmt.read();
    if !reply.contains("Alert!") {
      println!("{}", reply);
      break;
    }
  }
}