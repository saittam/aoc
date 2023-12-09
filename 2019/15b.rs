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
  Wall = 0,
  Empty = 1,
  Oxygen = 2,
}

impl TryFrom<isize> for Tile {
  type Error = &'static str;
  
  fn try_from(v: isize) -> Result<Tile, Self::Error> {
    Ok(match v {
      0 => Tile::Wall,
      1 => Tile::Empty,
      2 => Tile::Oxygen,
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
  let mut screen = HashMap::<(isize, isize), Tile>::new();
  screen.insert((0, 0), Tile::Empty);
  
  let len = ex(&mut vm, &mut screen, 0, 0);
 
  show(&screen);
  println!("{:?}", len);
  
  let (x, y) = *screen.iter().filter(|(_, t)| **t == Tile::Oxygen).next().unwrap().0;
  
  let olen = exo(&mut screen, x, y);
  println!("{:?}", olen);
}

#[derive(Debug, Clone, Copy)]
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
}

fn ex(vm: &mut VM, screen: &mut HashMap::<(isize, isize), Tile>, x: isize, y: isize) -> Option<usize> {
  let mut len = None;
  for dir in &Dir::all() {
    let (nx, ny) = match dir {
      Dir::N => (x, y + 1),
      Dir::S => (x, y - 1),
      Dir::W => (x + 1, y),
      Dir::E => (x - 1, y),
    };
    if screen.contains_key(&(nx, ny)) {
      continue;
    }
    
    vm.input.push_back(*dir as isize);
    let out = vm.get_output::<[isize; 1]>().unwrap();
    let t = Tile::try_from(out[0]).unwrap();
    
    screen.insert((nx, ny), t);    
    if t != Tile::Wall {
      let ol = ex(vm, screen, nx, ny);
      len = match (len, ol) {
        (Some(l), Some(ln)) => Some(usize::min(l, ln + 1)),
        (Some(l), None) => Some(l),
        (None, Some(ln)) => Some(ln + 1),
        (None, None) => None,
      };
      vm.input.push_back(dir.rev() as isize);
      let bo = vm.get_output::<[isize; 1]>().unwrap();
      assert_eq!(Tile::try_from(bo[0]).unwrap(), Tile::Empty);
    }
    
    if t == Tile::Oxygen {
      len = Some(1);
    }
  }

  len
}

fn exo(screen: &mut HashMap::<(isize, isize), Tile>, x: isize, y: isize) -> usize {
  let mut len = 0;
  for dir in &Dir::all() {
    let (nx, ny) = match dir {
      Dir::N => (x, y + 1),
      Dir::S => (x, y - 1),
      Dir::W => (x + 1, y),
      Dir::E => (x - 1, y),
    };
    
    if *screen.get(&(nx, ny)).unwrap() == Tile::Empty {
      screen.insert((nx, ny), Tile::Oxygen);
      len = usize::max(len, 1 + exo(screen, nx, ny));
    }
  }

  len
}
  
fn show(screen: &HashMap<(isize, isize), Tile>) {
  println!("----------");
  
  let lx = *screen.keys().map(|(x, _)| x).min().unwrap();
  let ux = *screen.keys().map(|(x, _)| x).max().unwrap();
  let ly = *screen.keys().map(|(_, y)| y).min().unwrap();
  let uy = *screen.keys().map(|(_, y)| y).max().unwrap();
  
  for y in ly..=uy {
    let line = (lx..=ux).map(
      |x| screen.get(&(x, y)).map(|t| match t {
        Tile::Empty => '.',
        Tile::Wall => '#',
        Tile::Oxygen => 'X',
      }).unwrap_or(' ')).collect::<String>();
    println!("{}", line);
  }
  
  println!("----------");
}