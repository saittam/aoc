use std::io::BufRead;
use std::collections::{HashMap, VecDeque};
use std::ops::IndexMut;
use std::convert::TryFrom;

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
  
  fn get_output<T: AsMut<[isize]> + Default>(&mut self, input: isize) -> Option<T> {
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
          Status::NeedInput => { self.input.push_back(input) },
          Status::Output => (),
        }
      }
    }
    
    Some(obj)
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
  Empty = 0,
  Wall = 1,
  Block = 2,
  Paddle = 3,
  Ball = 4,
}

impl TryFrom<isize> for Tile {
  type Error = &'static str;
  
  fn try_from(v: isize) -> Result<Tile, Self::Error> {
    Ok(match v {
      0 => Tile::Empty,
      1 => Tile::Wall,
      2 => Tile::Block,
      3 => Tile::Paddle,
      4 => Tile::Ball,
      _ => return Err("invalid"),
    })
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
  vm.mem[0] = 2;
  let mut screen = HashMap::<(isize, isize), Tile>::new();
  let mut score = 0;
  let mut ball = 0isize;
  let mut paddle = 0isize;
  while let Some(r) = vm.get_output::<[isize; 3]>((ball - paddle).signum()) {
    //println!("{:?}", r);
    if (r[0], r[1]) == (-1, 0) {
      score = r[2];
    } else {
      let t = Tile::try_from(r[2]).unwrap();
      match t {
        Tile::Ball => ball = r[0],
        Tile::Paddle => paddle = r[0],
        _ => (),
      }
      screen.insert((r[0], r[1]), t);
    }
    
    //show(&screen);
  }
  
  //show(&screen);
  
  //let nb = screen.values().filter(|t| **t == Tile::Block).count();
  //println!("{}", nb);
  
  println!("{}", score);
}
  
fn show(screen: &HashMap<(isize, isize), Tile>) {
  let lx = *screen.keys().map(|(x, _)| x).min().unwrap();
  let ux = *screen.keys().map(|(x, _)| x).max().unwrap();
  let ly = *screen.keys().map(|(_, y)| y).min().unwrap();
  let uy = *screen.keys().map(|(_, y)| y).max().unwrap();
  
  for y in ly..=uy {
    let line = (lx..=ux).map(
      |x| match screen.get(&(x, y)).unwrap_or(&Tile::Empty) {
        Tile::Empty => ' ',
        Tile::Wall => '#',
        Tile::Block => '@',
        Tile::Paddle => '_',
        Tile::Ball => 'o',
      }).collect::<String>();
    println!("{}", line);
  }
}